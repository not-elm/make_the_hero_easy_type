use crate::ratio::Ratio;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MovableRatio {
    pub moved: bool,
    pub ratio: Ratio,
}

impl MovableRatio {
    #[cfg(test)]
    pub(crate) fn new_moved(ratio: Ratio) -> Self {
        Self {
            moved: true,
            ratio,
        }
    }
}

impl From<Ratio> for MovableRatio {
    #[inline]
    fn from(value: Ratio) -> Self {
        Self {
            moved: false,
            ratio: value,
        }
    }
}

impl From<isize> for MovableRatio {
    #[inline]
    fn from(value: isize) -> Self {
        Self::from(Ratio::from(value))
    }
}