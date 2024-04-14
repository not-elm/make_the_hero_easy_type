use rand::prelude::{SliceRandom, ThreadRng};

use crate::answer::steps::Steps;
use crate::calculator::Calculator;
use crate::ratio::Ratio;
use crate::stage::{RatioArray, Stage};

pub mod steps;

pub struct AnswerInfo {
    pub ratio: Ratio,
    pub steps: Steps,
}

impl AnswerInfo {
    pub fn generate<const STAGE_SIZE: usize, Calc: Calculator + Default + 'static>(ratios: RatioArray<STAGE_SIZE>) -> Self {
        let mut stage = Stage::new(Calc::default(), ratios);
        let mut rng = rand::thread_rng();

        loop {
            if let Some(answer_info) = try_generate(&mut stage, &mut rng) {
                return answer_info;
            } else {
                stage = Stage::new(Calc::default(), ratios);
            }
        }
    }
}

pub fn generate_random_ratios<const STAGE_SIZE: usize>() -> RatioArray<STAGE_SIZE> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<isize> = (1..=10).collect();
    nums.shuffle(&mut rng);
    let mut ratios = [Ratio::from(0); STAGE_SIZE];
    ratios.iter_mut().for_each(|r| {
        *r = Ratio::from(nums.pop().unwrap());
    });
    ratios
}

fn try_generate<const STAGE_SIZE: usize, Calc: Calculator + 'static>(stage: &mut Stage<STAGE_SIZE, Calc>, rng: &mut ThreadRng) -> Option<AnswerInfo> {
    let mut steps = Steps::default();
    loop {
        let mut indices = stage.movable_indices();
        indices.shuffle(rng);

        let cell_no = indices.pop()?;
        let mut dirs = stage.movable_dirs(cell_no);
        dirs.shuffle(rng);
        let dir = dirs.pop()?;
        steps.push(cell_no, dir);
        stage.move_cell(cell_no, dir);

        if let Some(r) = stage.last_ratio() {
            // for (i, step) in steps.iter().enumerate() {
            //     println!("[{i}] no: {} dir: {:?}\n", step.0, step.1);
            // }
            return Some(AnswerInfo {
                ratio: r,
                steps,
            });
        }
    }
}

