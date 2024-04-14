use crate::calculator::Calculator;
use crate::movable_ratio::MovableRatio;
use crate::move_dir::MoveDir;
use crate::stage::StageRatioArray;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SmallSizeCalculator;

impl Calculator for SmallSizeCalculator {
    fn can_move<const STAGE_SIZE: usize>(
        &self,
        ratios: &StageRatioArray<STAGE_SIZE>,
        src_no: usize,
        dir: MoveDir,
    ) -> bool {
        let Some(Some(MovableRatio {
                          moved: _n @ false,
                          ratio: src_ratio
                      })) = ratios.get(src_no) else {
            return false;
        };
        let Some(dist_ratio) = Self::dist_no::<STAGE_SIZE>(src_no, &dir)
            .and_then(|dist_no| ratios.get(dist_no))
            else {
                return false;
            };
        let can = if let Some(MovableRatio { ratio: dist_ratio, .. }) = dist_ratio {
            match dir {
                MoveDir::RightDown => {
                    (*dist_ratio / *src_ratio).is_some()
                }
                _ => true
            }
        } else {
            true
        };

        can && match dir {
            MoveDir::Up => {
                src_no == 2
            }
            MoveDir::RightUp => {
                src_no == 0 || src_no == 2
            }
            MoveDir::Right => {
                src_no == 0
            }
            MoveDir::RightDown => {
                src_no == 0 || src_no == 1
            }
            MoveDir::Down => {
                src_no == 1
            }
            MoveDir::LeftDown => {
                src_no == 1 || src_no == 3
            }
            MoveDir::Left => {
                src_no == 3
            }
            MoveDir::LeftUp => {
                src_no == 2 || src_no == 3
            }
        }
    }

    fn dir_as_isize(dir: &MoveDir) -> isize {
        match dir {
            MoveDir::LeftUp => -2,
            MoveDir::Up => -1,
            MoveDir::RightUp => 1,
            MoveDir::Left => -3,
            MoveDir::Right => 3,
            MoveDir::LeftDown => -1,
            MoveDir::Down => 1,
            MoveDir::RightDown => 2
        }
    }
}

