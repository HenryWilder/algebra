//! Roots of numbers.

use crate::{
    factor::{Factor, Factoring},
    notation::{expr::Simplify, Atom, Notation},
};

/// The root of some number.
///
/// <div class="warning"> Note: Currently only supports square roots. </div>
///
/// ## Construction
///
/// let 𝑛 and 𝑚 be integers:
///
/// [Radical::from]\(𝑛) is equal to 𝑛√1, which simplifies to exactly 𝑛.
///
/// [Radical::new]\(𝑚) is equal to 1√𝑚, or simply √𝑚.
///
/// [Radical] { coef: 𝑛, rad: 𝑚 } is equal to 𝑛√𝑚.
///
/// ```
/// # use algebra::notation::expr::{radical::Radical, simplify::Simplify};
/// let (n, m) = (4, 5);
///
/// let from_coefficient = Radical::from(n);
/// assert_eq!(from_coefficient.simplified(), n);
///
/// let from_radicand = Radical::new(m);
/// assert_eq!(from_radicand.simplified(), from_radicand);
///
/// let from_explicit = Radical { coef: n, rad: m };
/// assert_eq!(from_explicit.simplified(), from_explicit);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Radical {
    /// The coefficient.
    ///
    /// The number the root is being multiplied by.
    pub coef: i32,

    /// The radicand.
    ///
    /// The number being rooted.
    pub rad: i32,
}

impl From<i32> for Radical {
    /// Convert an integer value into a radical with radicand of 1.
    ///
    /// Use [`new`][Radical::new()] if you need to set the radicand and have a coefficient of 1.\
    /// Use `Radical { coef, rad }` if you need to set both the coefficient and radicand.
    fn from(coef: i32) -> Self {
        Self { coef, rad: 1 }
    }
}

impl Radical {
    /// Construct a new radical from its radicand. Its coefficient will be 1.
    ///
    /// Use [`from`][Radical::from()] if you are creating a radical equivalent to an integer value.\
    /// Use `Radical { coef, rad }` if you need to set both the coefficient and radicand.
    pub fn new(rad: i32) -> Self {
        Self { coef: 1, rad }
    }

    /// Returns the square of the radical.
    ///
    /// Because the radical is already a square root, squaring it turns it into a whole number.
    pub fn squared(&self) -> i32 {
        self.coef * self.coef * self.rad
    }
}

impl std::ops::Mul<i32> for Radical {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            coef: self.coef * rhs,
            rad: self.rad,
        }
    }
}

impl std::fmt::Display for Radical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.coef, self.rad) {
            (c @ (..=0 | 2..), r @ (..=0 | 2..)) => format!("{c}√{r}").fmt(f),
            (1, r @ (..=0 | 2..)) => format!("√{r}").fmt(f),
            (c, 1) => c.fmt(f),
        }
    }
}

/// If the square root of n can be expressed as an integer, returns that integer. Otherwise returns [`None`].
pub fn sqrt_i(n: i32) -> Option<i32> {
    use std::cmp::Ordering::*;
    match n {
        ..=-1 => None,
        0..=1 => Some(n),
        2.. => {
            let mut root = 2;
            loop {
                match (root * root).cmp(&n) {
                    Less => root += 1,
                    Equal => break Some(root),
                    Greater => break None,
                }
            }
        }
    }
}

impl Simplify for Radical {
    fn simplify(self) -> Notation {
        match self.rad {
            ..=-1 => Notation::from(Atom::Complex),
            0 => Notation::from(0),
            1 => Notation::from(self.coef),
            2.. => {
                if let Some(root) = sqrt_i(self.rad) {
                    // Simple

                    Notation::from(self.coef * root)
                } else {
                    // Perfect squares

                    let n = self.squared();

                    let mut gps_fac = 1; // Greatest perfect square factor
                    let mut gps_mul = n; // Factor associated with gps_fac

                    for Factor { common, associated } in n.factors() {
                        let permutations: [(i32, i32); 2] =
                            [(common, associated), (associated, common)];

                        for (a, b) in permutations {
                            if let Some(a_root) = sqrt_i(a) {
                                if a_root > gps_fac {
                                    (gps_fac, gps_mul) = (a_root, b);
                                }
                            }
                        }
                    }

                    Notation::from(Radical {
                        coef: gps_fac,
                        rad: gps_mul,
                    })
                }
            }
        }
    }
}

// todo: ordered?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_radical() {
        // Simplifies to coefficient
        for coef in 0..10 {
            assert_eq!(Radical::from(coef).simplify(), coef);
        }

        // Simplifies to integer
        for root in 0..10 {
            assert_eq!(Radical::new(root * root).simplify(), root);
        }

        // Can't be simplified
        assert_eq!(Radical::new(2).simplify(), Radical::new(2));

        // Simplifies to a radical
        assert_eq!(Radical::new(8).simplify(), Radical { coef: 2, rad: 2 });
    }
}
