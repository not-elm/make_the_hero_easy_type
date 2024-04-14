use std::fmt::{Display, Formatter};
use std::num::NonZeroIsize;
use std::ops::{Add, Div, Mul, Sub};

/// Ratio represents a divisor.
/// This struct must be created by [`Ratio::new`] to ensure that [`Ratio::denom`] is non-zero.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Reflect))]
#[non_exhaustive]
pub struct Ratio {
    /// numerator
    pub numer: isize,
    /// denominator
    pub denom: isize,
}

impl Ratio {
    pub fn new(numer: isize, denom: NonZeroIsize) -> Self {
        let gcd = num::integer::gcd(numer, denom.get());
        Self {
            numer: numer / gcd,
            denom: denom.get() / gcd,
        }
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denom == 1 {
            f.write_fmt(format_args!("{}", self.numer))
        } else {
            f.write_fmt(format_args!("{}/{}", self.numer, self.denom))
        }
    }
}

impl From<isize> for Ratio {
    fn from(value: isize) -> Self {
        Self::new(value, NonZeroIsize::new(1).unwrap())
    }
}

impl Add for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Self) -> Self::Output {
        let l_denom = self.denom;
        let r_denom = rhs.denom;

        let lcm = num::integer::lcm(l_denom, r_denom);
        Self::new(
            self.numer * (lcm / l_denom) + rhs.numer * (lcm / r_denom),
            unsafe {
                // SAFETY: Safe because lcm is never zero.
                NonZeroIsize::new_unchecked(lcm)
            },
        )
    }
}


impl Sub for Ratio {
    type Output = Ratio;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * -1)
    }
}


impl Mul<isize> for Ratio {
    type Output = Ratio;

    #[inline]
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.numer * rhs, unsafe {
            // SAFETY: Safe because lcm is never zero.
            NonZeroIsize::new_unchecked(self.denom)
        })
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    #[inline]
    fn mul(self, rhs: Ratio) -> Self::Output {
        Self::new(self.numer * rhs.numer, unsafe {
            // SAFETY: Safe because lcm is never zero.
            NonZeroIsize::new_unchecked(self.denom * rhs.denom)
        })
    }
}


impl Div<Self> for Ratio {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        Some(Self::new(self.numer * rhs.denom, NonZeroIsize::new(self.denom * rhs.numer)?))
    }
}


#[cfg(test)]
mod tests {
    use std::num::NonZeroIsize;

    use crate::ratio::Ratio;

    #[test]
    fn approx() {
        assert_eq!(Ratio::new(2, NonZeroIsize::new(4).unwrap()), Ratio { numer: 1, denom: 2 });
        assert_eq!(Ratio::new(11, NonZeroIsize::new(12).unwrap()), Ratio { numer: 11, denom: 12 });
    }

    #[test]
    fn add() {
        let lhs = Ratio { numer: 3, denom: 4 };
        let rhs = Ratio { numer: 1, denom: 6 };
        // (9/12) + (2/12) = (11/12)
        assert_eq!(lhs + rhs, Ratio { numer: 11, denom: 12 });
    }

    #[test]
    fn add_then_approx() {
        let lhs = Ratio { numer: 3, denom: 4 };
        let rhs = Ratio { numer: 3, denom: 6 };
        // (9/12) + (6/12) = (15/12) = (5/4)
        assert_eq!(lhs + rhs, Ratio { numer: 5, denom: 4 });

        let lhs = Ratio { numer: 3, denom: 25 };
        let rhs = Ratio { numer: 3, denom: 75 };
        assert_eq!(lhs + rhs, Ratio { numer: 4, denom: 25 });
    }

    #[test]
    fn sub() {
        let lhs = Ratio { numer: 3, denom: 4 };
        let rhs = Ratio { numer: 3, denom: 6 };
        // (9/12) - (6/12) = (3/12) = (1/4)
        assert_eq!(lhs - rhs, Ratio { numer: 1, denom: 4 });
    }

    #[test]
    fn mul_scalar() {
        let ratio = Ratio::new(3, NonZeroIsize::new(4).unwrap());
        assert_eq!(ratio * 3, Ratio { numer: 9, denom: 4 });
        assert_eq!(ratio * 4, Ratio { numer: 3, denom: 1 });
    }

    #[test]
    fn mul() {
        let lhs = Ratio::new(3, NonZeroIsize::new(4).unwrap());
        let rhs = Ratio::new(11, NonZeroIsize::new(5).unwrap());
        assert_eq!(lhs * rhs, Ratio { numer: 33, denom: 20 });

        let lhs = Ratio::new(3, NonZeroIsize::new(12).unwrap());
        let rhs = Ratio::new(10, NonZeroIsize::new(5).unwrap());
        assert_eq!(lhs * rhs, Ratio { numer: 1, denom: 2 });
    }

    #[test]
    fn div() {
        let lhs = Ratio::new(3, NonZeroIsize::new(4).unwrap());
        let rhs = Ratio::new(11, NonZeroIsize::new(5).unwrap());
        assert_eq!(lhs / rhs, Some(Ratio { numer: 15, denom: 44 }));
        let lhs = Ratio::new(2, NonZeroIsize::new(3).unwrap());
        let rhs = Ratio::new(2, NonZeroIsize::new(5).unwrap());
        assert_eq!(lhs / rhs, Some(Ratio { numer: 5, denom: 3 }));
        let lhs = Ratio::new(2, NonZeroIsize::new(3).unwrap());
        let rhs = Ratio::new(0, NonZeroIsize::new(5).unwrap());
        assert_eq!(lhs / rhs, None);
    }
}