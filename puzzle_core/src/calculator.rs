use num::FromPrimitive;
use crate::move_dir::MoveDir;
use crate::stage::StageRatioArray;

pub mod small_size;

pub trait Calculator {
    fn can_move<const STAGE_SIZE: usize>(
        &self,
        ratios: &StageRatioArray<STAGE_SIZE>,
        src_no: usize,
        dir: MoveDir,
    ) -> bool;
    
    fn dir_as_isize(dir: &MoveDir) -> isize;
    
    fn dist_no<const STAGE_SIZE: usize>(src_no: usize, dir: &MoveDir) -> Option<usize>{
        let dist_no = src_no as isize + Self::dir_as_isize(dir);
        let dist_no = usize::from_isize(dist_no)?;
        (dist_no < STAGE_SIZE).then_some(dist_no)
    }
}