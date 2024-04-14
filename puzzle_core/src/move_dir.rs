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
}


