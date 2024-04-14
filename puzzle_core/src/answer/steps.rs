use std::collections::VecDeque;
use crate::move_dir::MoveDir;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Steps(VecDeque<(usize, MoveDir)>);

impl Steps{
    #[inline]
    pub fn push(&mut self, cell_no: usize, dir: MoveDir){
        self.0.push_back((cell_no, dir));
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<(usize, MoveDir)>{
        self.0.pop_front()
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}