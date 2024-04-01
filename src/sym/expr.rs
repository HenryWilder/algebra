//! Algebraic expressions comprised of multiple parts, which can be simplified.

use crate::{
    factor::{gcf, Factoring},
    sym::{atom::Atom, Sym},
};

/// An expression capable of being simplified.
pub trait Simplify {
    /// Converts the expression to its simplest form.
    fn simplify(self) -> Sym;

    /// Returns the simplest form of the expression.
    fn simplified(&self) -> Sym
    where
        Self: Clone,
    {
        self.clone().simplify()
    }
}

/// Algebraic Expression.
///
/// Notation representing an algebraic expression.
/// Expressions can be simplified.
///
/// <div class="warning">
///
/// # Equality operation is intended only to be used on notation that has already been simplified.
///
/// **Does not simplify.** Fractions are not considered equal to radicals, even if they are mathematically equivalent.\
///
/// </div>
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A fraction made from a combination of algebraic atomics.
    Fraction {
        /// The numerator.
        ///
        /// Upper side of the fraction; the part being divided.
        num: Atom,

        /// The denominator.
        ///
        /// Lower side of the fraction; the part dividing the numerator.
        den: Atom,
    },

    /// A radical - the root of some number.
    ///
    /// <div class="warning"> Note: Currently only supports square roots. </div>
    Radical {
        /// The coefficient.
        ///
        /// The number the root is being multiplied by.
        coef: i32,

        /// The radicand.
        ///
        /// The number being rooted.
        rad: i32,
    },
}

impl Expr {
    /// Returns true if the expression represents a [`Fraction`], false otherwise.
    pub fn is_fraction(&self) -> bool {
        matches!(self, Expr::Fraction { num: _, den: _ })
    }

    /// Returns true if the expression represents a [`Radical`], false otherwise.
    pub fn is_radical(&self) -> bool {
        matches!(self, Expr::Radical { coef: _, rad: _ })
    }
}

impl Simplify for Expr {
    fn simplify(self) -> Sym {
        use {Atom::*, Expr::*};
        match self {
            Fraction { num, den } => {
                match (num, den) {
                    (Complex, _) | (_, Complex) => Sym::Atom(Complex),

                    (Undefined, _) | (_, Undefined) | (_, Num(0)) => Sym::Atom(Undefined),

                    (Num(0), _) => Sym::Atom(Num(0)),

                    (num @ (Huge | NegHuge | Epsilon | NegEpsilon), den @ Num(_)) => {
                        let pos = num.is_positive() == den.is_positive();
                        Sym::Atom(if pos { num } else { -num })
                    }

                    (Num(num), Num(den)) => {
                        if den.is_factor_of(num) {
                            // Division leaves no remainder
                            Sym::Atom(Num(num / den))
                        } else {
                            // Transfer sign to the top
                            let sign = if (num < 0) != (den < 0) { -1 } else { 1 };
                            let (num_abs, den_abs) = (num.abs(), den.abs());
                            let gcf = gcf([num_abs, den_abs]);
                            Sym::Expr(Fraction {
                                num: Num(sign * num_abs / gcf),
                                den: Num(den_abs / gcf),
                            })
                        }
                    }

                    (num @ (Huge | NegHuge), den @ (Huge | NegHuge)) => {
                        let pos = num.is_positive() == den.is_positive();
                        Sym::Atom(if pos { Huge } else { NegHuge })
                    }

                    (num @ (Num(_) | Epsilon | NegEpsilon), den @ (Huge | NegHuge)) => {
                        let pos = num.is_positive() == den.is_positive();
                        Sym::Atom(if pos { Epsilon } else { NegEpsilon })
                    }

                    (num, den @ (Epsilon | NegEpsilon)) => {
                        let pos = num.is_positive() == den.is_positive();
                        Sym::Atom(if pos { Huge } else { NegHuge })
                    }
                }
            }

            Radical { coef, rad } => {
                match rad {
                    ..=-1 => Sym::Atom(Complex),
                    0 => Sym::Atom(Num(0)),
                    1 => Sym::Atom(Num(coef)),
                    2.. => {
                        if let Some(root) = sqrt_i(rad) {
                            // Simple

                            Sym::Atom(Num(coef * root))
                        } else {
                            // Perfect squares

                            let n = coef * coef * rad; // Squared

                            let mut gps_fac = 1; // Greatest perfect square factor
                            let mut gps_mul = n; // Factor associated with gps_fac

                            for (common, associated) in n.factors() {
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

                            Sym::Expr(Radical {
                                coef: gps_fac,
                                rad: gps_mul,
                            })
                        }
                    }
                }
            }
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expr::*;
        match self {
            Fraction { num, den } => format!("{num}/{den}").fmt(f),

            Radical { coef, rad } => match (coef, rad) {
                (c @ (..=0 | 2..), r @ (..=0 | 2..)) => format!("{c}√{r}").fmt(f),
                (1, r @ (..=0 | 2..)) => format!("√{r}").fmt(f),
                (c, 1) => c.fmt(f),
            },
        }
    }
}

#[cfg(test)]
mod simplify_fraction_tests {
    use super::{Atom::*, *};

    #[test]
    fn test_denominator_of_1() {
        for num in 0..=10 {
            let frac = Expr::Fraction {
                num: Num(num),
                den: Num(1),
            };
            assert_eq!(frac.simplify(), num);
        }
    }

    #[test]
    fn test_simplifies_to_integer() {
        for den in 1..=10 {
            for n in 0..=10 {
                let frac = Expr::Fraction {
                    num: Num(den * n),
                    den: Num(den),
                };
                assert_eq!(frac.simplify(), n);
            }
        }
    }

    #[test]
    fn test_already_simplest() {
        for den in 2..=10 {
            let frac = Expr::Fraction {
                num: Num(1),
                den: Num(den),
            };
            assert_eq!(frac.simplified(), frac);
        }
    }

    #[test]
    fn test_simplifies_to_half() {
        for num in 1..=10 {
            let frac = Expr::Fraction {
                num: Num(num),
                den: Num(num * 2),
            };
            let simplest = Expr::Fraction {
                num: Num(1),
                den: Num(2),
            };
            assert_eq!(frac.simplify(), simplest);
        }
    }

    #[test]
    fn test_division_by_zero() {
        for num in 1..=10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Num(0),
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_undefined()));
        }
    }

    #[test]
    fn test_positive_division_by_huge() {
        for num in 1..=10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Huge,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_positive_epsilon()));
        }
    }

    #[test]
    fn test_negative_division_by_huge() {
        for num in -1..=-10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Huge,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_negative_epsilon()));
        }
    }

    #[test]
    fn test_positive_division_by_negative_huge() {
        for num in 1..=10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: NegHuge,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_negative_epsilon()));
        }
    }

    #[test]
    fn test_negative_division_by_negative_huge() {
        for num in -1..=-10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: NegHuge,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_positive_epsilon()));
        }
    }

    #[test]
    fn test_positive_division_by_epsilon() {
        for num in 1..=10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Epsilon,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_positive_huge()));
        }
    }

    #[test]
    fn test_negative_division_by_epsilon() {
        for num in -1..=-10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Epsilon,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_negative_huge()));
        }
    }

    #[test]
    fn test_positive_division_by_negative_epsilon() {
        for num in 1..=10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: NegEpsilon,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_negative_huge()));
        }
    }

    #[test]
    fn test_negative_division_by_negative_epsilon() {
        for num in -1..=-10 {
            let simple = Expr::Fraction {
                num: Num(num),
                den: Epsilon,
            }
            .simplify();
            assert!(simple.atom().is_some_and(|x| x.is_positive_huge()));
        }
    }
}

#[cfg(test)]
mod simplify_radical_tests {
    use super::*;

    #[test]
    fn test_simplify_radical() {
        // Simplifies to coefficient
        for coef in 0..10 {
            assert_eq!(Expr::Radical { coef, rad: 1 }.simplify(), coef);
        }

        // Simplifies to integer
        for root in 0..10 {
            assert_eq!(
                Expr::Radical {
                    coef: root * root,
                    rad: 1
                }
                .simplify(),
                root
            );
        }

        // Can't be simplified
        assert_eq!(
            Expr::Radical { coef: 2, rad: 1 }.simplify(),
            Expr::Radical { coef: 2, rad: 1 }
        );

        // Simplifies to a radical
        assert_eq!(
            Expr::Radical { coef: 8, rad: 1 }.simplify(),
            Expr::Radical { coef: 2, rad: 2 }
        );
    }
}
