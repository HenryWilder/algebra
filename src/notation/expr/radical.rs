//! Roots of numbers.

use crate::{
    factor::{Factor, Factoring},
    notation::{expr::Simplify, AlgAtom, AlgNotation},
    sqrt_i,
};

/// The root of some number.
///
/// <div class="warning"> Currently only supports square roots. </div>
#[derive(Debug, PartialEq, Eq)]
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

impl Default for Radical {
    /// Construct a new radical representing the whole number 1.
    fn default() -> Self {
        Self { coef: 1, rad: 1 }
    }
}

impl Radical {
    /// Construct a new radical from integer coefficient and radicand.
    pub fn from_ints(coef: i32, rad: i32) -> Self {
        Self { coef, rad }
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

impl ToString for Radical {
    fn to_string(&self) -> String {
        match (self.coef, self.rad) {
            (c @ (..=0 | 2..), r @ (..=0 | 2..)) => format!("{c}√{r}"),
            (1, r @ (..=0 | 2..)) => format!("√{r}"),
            (c, 1) => format!("{c}"),
        }
    }
}

impl Simplify for Radical {
    fn simplified(&self) -> AlgNotation {
        match self.rad {
            ..=-1 => AlgNotation::from(AlgAtom::Complex),
            0 => AlgNotation::from(0),
            1 => AlgNotation::from(self.coef),
            2.. => {
                if let Some(root) = sqrt_i(self.rad) {
                    // Simple

                    AlgNotation::from(self.coef * root)
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

                    AlgNotation::from(Radical::from_ints(gps_fac, gps_mul))
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
            assert_eq!(Radical::from_ints(coef, 1).simplified(), coef);
        }

        // Simplifies to integer
        for root in 0..10 {
            assert_eq!(Radical::from_ints(1, root * root).simplified(), root);
        }

        // Can't be simplified
        assert_eq!(
            Radical::from_ints(1, 2).simplified(),
            AlgNotation::from(Radical::from_ints(1, 2))
        );

        // Simplifies to a radical
        assert_eq!(
            Radical::from_ints(1, 8).simplified(),
            AlgNotation::from(Radical::from_ints(2, 2))
        );
    }
}
