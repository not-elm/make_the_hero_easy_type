use std::ops::Index;

use num::FromPrimitive;

use crate::calculator::Calculator;
use crate::movable_ratio::MovableRatio;
use crate::move_dir::MoveDir;
use crate::ratio::Ratio;

pub type RatioArray<const STAGE_SIZE: usize> = [Ratio; STAGE_SIZE];

pub type StageRatioArray<const STAGE_SIZE: usize> = [Option<MovableRatio>; STAGE_SIZE];


#[derive(Debug)]
pub struct Stage<const STAGE_SIZE: usize, Calc: Calculator> {
    ratios: StageRatioArray<STAGE_SIZE>,
    calculator: Calc,
}


impl<const STAGE_SIZE: usize, Calc> Stage<STAGE_SIZE, Calc>
    where Calc: Calculator + 'static
{
    #[inline]
    pub fn new(calculator: Calc, ratios: RatioArray<STAGE_SIZE>) -> Self {
        Self {
            ratios: ratios.map(|r| Some(MovableRatio::from(r))),
            calculator,
        }
    }

    pub fn last_ratio(&self) -> Option<Ratio> {
        if self.ratios.iter().any(|r| { r.is_some_and(|r| r.moved) }) {
            return None;
        }

        let ratios = self.ratios
            .iter()
            .copied()
            .flatten()
            .filter(|r| !r.moved)
            .collect::<Vec<_>>();
        if ratios.len() == 1 {
            Some(ratios[0].ratio)
        } else {
            None
        }
    }

    pub fn movable_indices(&self) -> Vec<usize> {
        self.ratios
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                r.is_some_and(|r| !r.moved).then_some(i)
            })
            .collect()
    }

    pub fn failed(&self) -> bool {
        self.ratios
            .iter()
            .all(|r| {
                r
                    .map(|r| r.moved)
                    .unwrap_or(true)
            })
    }

    pub fn can_move(&self, src: usize, dir: MoveDir) -> bool {
        self.calculator.can_move(&self.ratios, src, dir)
    }

    pub fn movable_dirs(&self, src: usize) -> Vec<MoveDir> {
        let Some(MovableRatio { moved: _m @ false, .. }) = self.ratios[src] else {
            return Vec::with_capacity(0);
        };
        let mut moves = Vec::with_capacity(8);
        let mut push = |dir: MoveDir| {
            if self.can_move(src, dir) {
                moves.push(dir);
            }
        };
        push(MoveDir::LeftUp);
        push(MoveDir::Up);
        push(MoveDir::RightUp);
        push(MoveDir::Left);
        push(MoveDir::Right);
        push(MoveDir::LeftDown);
        push(MoveDir::RightDown);

        moves
    }

    pub fn move_dist(&self, src: usize, dir: MoveDir) -> Option<&Option<MovableRatio>> {
        let index = usize::from_isize(src as isize + dir as isize)?;
        self.ratios.get(index)
    }

    pub fn movable_ratios(&self) -> &[Option<MovableRatio>; STAGE_SIZE] {
        &self.ratios
    }

    pub fn ratios(&self) -> [Option<Ratio>; STAGE_SIZE] {
        self.ratios.map(|r| r.map(|r| r.ratio))
    }

    pub fn move_cell(&mut self, src_no: usize, dir: MoveDir) {
        if !self.can_move(src_no, dir) {
            return;
        }
        let dist_no = Calc::dist_no::<STAGE_SIZE>(src_no, &dir).unwrap();
        let src_ratio = self.ratios[src_no].map(|m| m.ratio).unwrap();
        match dir {
            // add
            MoveDir::LeftUp => {
                self.execute_mov(src_no, dist_no, src_ratio, |d, s| Some(d + s));
            }
            // sub
            MoveDir::RightUp => {
                self.execute_mov(src_no, dist_no, src_ratio, |d, s| Some(d - s));
            }
            // div
            MoveDir::RightDown => {
                self.execute_mov(src_no, dist_no, src_ratio, |d, s| d / s);
            }
            // mul
            MoveDir::LeftDown => {
                self.execute_mov(src_no, dist_no, src_ratio, |d, s| Some(d * s));
            }
            // swap
            _ => {
                self.swap(src_no, dist_no)
            }
        }
    }

    fn execute_mov(
        &mut self,
        src: usize,
        dest: usize,
        src_ratio: Ratio,
        calc: impl FnOnce(Ratio, Ratio) -> Option<Ratio>,
    ) {
        if let Some(MovableRatio { ratio: dest_ratio, .. }) = self.ratios[dest] {
            if let Some(ratio) = calc(dest_ratio, src_ratio) {
                self.ratios[dest] = Some(MovableRatio::from(ratio));
                self.ratios[src] = None;
            }
        } else {
            self.swap(src, dest);
        }
    }

    fn swap(&mut self, src: usize, dest: usize) {
        self.ratios.swap(src, dest);
        if let Some(d) = self.ratios[dest].as_mut() {
            d.moved = true;
        }
    }
}

impl<const STAGE_SIZE: usize, Calc: Calculator + Default> Default for Stage<STAGE_SIZE, Calc> {
    fn default() -> Self {
        Self {
            ratios: [None; STAGE_SIZE],
            calculator: Calc::default(),
        }
    }
}

impl<const STAGE_SIZE: usize, Calc: Calculator + Default + 'static> From<RatioArray<STAGE_SIZE>> for Stage<STAGE_SIZE, Calc> {
    #[inline]
    fn from(ratios: RatioArray<STAGE_SIZE>) -> Self {
        Self::new(Calc::default(), ratios)
    }
}

impl<const STAGE_SIZE: usize, Calc: Calculator + Default + 'static> From<[isize; STAGE_SIZE]> for Stage<STAGE_SIZE, Calc> {
    #[inline]
    fn from(ratios: [isize; STAGE_SIZE]) -> Self {
        Self::new(Calc::default(), ratios.map(Ratio::from))
    }
}

impl<const STAGE_SIZE: usize, Calc: Calculator + 'static> Index<usize> for Stage<STAGE_SIZE, Calc> {
    type Output = Option<MovableRatio>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ratios[index]
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroIsize;

    use crate::calculator::small_size::SmallSizeCalculator;
    use crate::movable_ratio::MovableRatio;
    use crate::move_dir::MoveDir;
    use crate::ratio::Ratio;
    use crate::stage::Stage;

    #[test]
    fn swap_left() {
        let mut stage = stage();
        stage.move_cell(3, MoveDir::Left);
        assert_eq!(stage[3], Some(MovableRatio::from(1)));
        assert_eq!(stage[0], Some(MovableRatio::new_moved(Ratio::from(4))));
    }

    #[test]
    fn swap_up() {
        let mut stage = stage();
        stage.move_cell(2, MoveDir::Up);
        assert_eq!(stage[2], Some(MovableRatio::from(2)));
        assert_eq!(stage[1], Some(MovableRatio::new_moved(Ratio::from(3))));
    }


    #[test]
    fn swap_right() {
        let mut stage = stage();
        stage.move_cell(0, MoveDir::Right);
        assert_eq!(stage[0], Some(MovableRatio::from(4)));
        assert_eq!(stage[3], Some(MovableRatio::new_moved(Ratio::from(1))));
    }

    #[test]
    fn swap_down() {
        let mut stage = stage();
        stage.move_cell(1, MoveDir::Down);
        assert_eq!(stage[1], Some(MovableRatio::from(3)));
        assert_eq!(stage[2], Some(MovableRatio::new_moved(Ratio::from(2))));
    }

    #[test]
    fn add() {
        let mut stage = stage();
        stage.move_cell(2, MoveDir::LeftUp);
        assert_eq!(stage.ratios[2], None);
        assert_eq!(stage.ratios[0], Some(MovableRatio::from(Ratio::from(4))));
    }

    #[test]
    fn sub() {
        let mut stage = stage();
        stage.move_cell(0, MoveDir::RightUp);
        assert_eq!(stage.ratios[0], None);
        assert_eq!(stage.ratios[1], Some(MovableRatio::from(Ratio::from(1))));
    }

    #[test]
    fn mul() {
        let mut stage = stage();
        stage.move_cell(1, MoveDir::LeftDown);
        assert_eq!(stage.ratios[0], Some(MovableRatio::from(Ratio::from(2))));
        assert_eq!(stage.ratios[1], None);
    }

    #[test]
    fn div() {
        let mut stage = stage();
        stage.move_cell(1, MoveDir::RightDown);
        assert_eq!(stage.ratios[1], None);
        assert_eq!(stage.ratios[3], Some(MovableRatio::from(Ratio::new(1, NonZeroIsize::new(2).unwrap()))));
    }

    fn stage() -> Stage<4, SmallSizeCalculator> {
        Stage::from([
            1, 2, 3, 4,
        ])
    }
}