#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Reflect))]
pub enum MoveDir {
    LeftUp,
    Up,
    RightUp,
    Left,
    Right,
    LeftDown,
    Down,
    RightDown,
}

impl MoveDir {
    #[inline]
    pub const fn is_swap(&self) -> bool {
        matches!(self, MoveDir::Up | MoveDir::Left | MoveDir::Right | MoveDir::Down)
    }
    
    pub const fn reverse(&self) -> MoveDir{
        match self {
            MoveDir::LeftUp => MoveDir::RightDown,
            MoveDir::Up => MoveDir::Down,
            MoveDir::RightUp => MoveDir::LeftDown,
            MoveDir::Left => MoveDir::Right,
            MoveDir::Right => MoveDir::Left,
            MoveDir::LeftDown => MoveDir::RightUp,
            MoveDir::Down => MoveDir::Up,
            MoveDir::RightDown => MoveDir::LeftUp
        }
    }
}


