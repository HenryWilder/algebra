#![doc = include_str!("expr.md")]

use crate::{
    factor::Factoring,
    sym::{
        atom::Atom::{self, *},
        Sym::{self, *},
    },
};

use self::Expr::*;

#[doc = include_str!("expr.md")]
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A fraction made from a combination of algebraic atomics.
    ///
    /// # Example
    /// ```
    /// # use algebra::sym::expr::Expr::Fraction;
    /// let one_over_two = Radical { num: 1, den: 2 }; // 1/2
    /// ```
    ///
    /// Division produces an [`Atom`] quotient where possible.
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
    /// # Example
    /// ```
    /// # use algebra::sym::expr::Expr::Radical;
    /// let two_root_two = Radical { coef: 2, rad: 2 }; // 2√2
    /// ```
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

    /// A complex number - the combination of a real and imaginary number.
    Complex {
        /// The real part.
        ///
        /// The square of this number is positive.
        real: i32,

        /// The imaginary part.
        ///
        /// The square of this number is negative.
        imag: i32,
    },
}

impl Expr {
    /// Returns true if the expression represents a [`Fraction`], false otherwise.
    pub fn is_fraction(&self) -> bool {
        matches!(self, Fraction { .. })
    }

    /// Returns true if the expression represents a [`Radical`], false otherwise.
    pub fn is_radical(&self) -> bool {
        matches!(self, Radical { .. })
    }

    /// Converts the expression to its simplest form.
    pub fn simplify(self) -> Sym {
        match self {
            // Simplifying fractions
            Fraction { num, den } => {
                // A fraction's sign is equal to the numerator's sign XNOR the denominator's sign.
                let pos = num.is_positive() == den.is_positive();

                match (num, den) {
                    (Imaginary(_), _) | (_, Imaginary(_)) => todo!(),

                    // Anything divided by 1 simplifies to the numerator.
                    (num, Num(1)) => Atom(num),

                    // Undefined and unknown propogate.
                    (Undefined, _) | (_, Undefined) => Atom(Undefined),
                    (Unknown, _) | (_, Unknown) => Atom(Unknown),

                    // The direct definition of Undefined is a fraction with a denominator of zero.
                    (_, Num(0)) => Atom(Undefined),

                    // Except for undefined, any fraction with a numerator of zero will always be zero.
                    (Num(0), _) => Atom(Num(0)),

                    // Anything divided by itself (except for 0) is 1.
                    // Huge, Epsilon, Unknown, and Undefined do not have an "itself".
                    (n @ (Var(_) | Num(_)), d @ (Var(_) | Num(_))) if n == d => Atom(Num(1)),

                    // Cannot otherwise simplify a fraction containing a variable
                    (Var(_), _) | (_, Var(_)) => Atom(Num(1)),

                    // Huge and Epsilon divided by a number remain as they were, except to change signs if the denominator is different from them.
                    (num @ (Huge | NegHuge | Epsilon | NegEpsilon), Num(_)) => {
                        let flip_sign = pos != num.is_positive();
                        Atom(if flip_sign { -num } else { num })
                    }

                    // Huge divided by Huge does not give enough information.
                    // If the numerator Huge is twice the value of the denominator Huge, the result would be 2.
                    // But Huges cannot be distinguished, making the result Unknown.
                    (Huge | NegHuge, Huge | NegHuge) => Atom(Unknown),

                    // Anything divided by a fraction gets bigger.
                    // Therefore division by epsilon is certain to result in Huge in all cases beyond NegEpsilon < x < Epsilon.
                    // In the case of NegEpsilon < x < Epsilon, the result is Unknown.
                    (num, Epsilon | NegEpsilon) => match num {
                        Epsilon | NegEpsilon => Atom(Unknown),
                        _ => Atom(if pos { Huge } else { NegHuge }),
                    },

                    // Anything divided by Huge is Epsilon.
                    (_, Huge | NegHuge) => Atom(if pos { Epsilon } else { NegEpsilon }),

                    // Ordinary fraction
                    (Num(num), Num(den)) => {
                        if den.is_factor_of(num) {
                            // Division leaves no remainder
                            Atom(Num(num / den))
                        } else {
                            // Transfer sign to the top
                            let sign = if pos { 1 } else { -1 };
                            let (num_abs, den_abs) = (num.abs(), den.abs());
                            let gcf = i32::gcf([num_abs, den_abs]);
                            Expr(Fraction {
                                num: Num(sign * num_abs / gcf),
                                den: Num(den_abs / gcf),
                            })
                        }
                    }
                }
            }

            // Simplifying radicals
            Radical { coef, rad } => {
                if rad.is_positive() {
                    // Root of a positive

                    if let Atom(Num(root)) = sqrt_i(rad) {
                        // Radical simplifies to an integer

                        Atom(Num(coef * root))
                    } else {
                        // Simplify radical using perfect squares

                        let n = coef * coef * rad; // Square of radical

                        let mut gps_fac = 1; // Greatest perfect square factor
                        let mut gps_mul = n; // Factor associated with gps_fac

                        for (common, associated) in n.factors() {
                            let permutations: [(i32, i32); 2] =
                                [(common, associated), (associated, common)];

                            for (a, b) in permutations {
                                if let Atom(Num(a_root)) = sqrt_i(a) {
                                    if a_root > gps_fac {
                                        (gps_fac, gps_mul) = (a_root, b);
                                    }
                                }
                            }
                        }

                        Expr(Radical {
                            coef: gps_fac,
                            rad: gps_mul,
                        })
                    }
                } else {
                    Atom(Imaginary(1))
                }
            }

            Complex { .. } => {
                todo!()
            }
        }
    }

    /// Returns the simplest form of the expression.
    pub fn simplified(&self) -> Sym {
        self.clone().simplify()
    }
}

/// If the square root of n can be expressed as an integer, returns that integer. Otherwise returns [`None`].
pub fn sqrt_i(n: i32) -> Sym {
    use std::cmp::Ordering::*;
    match n {
        ..=-1 => Atom(Imaginary(-n)),
        0 | 1 => Atom(Num(n)), // Zero and one, specifically, are their own square roots
        2.. => {
            let mut root = 2;
            loop {
                match (root * root).cmp(&n) {
                    Less => root += 1,
                    Equal => break Atom(Num(root)),
                    Greater => break Expr(Radical { coef: 1, rad: n }),
                }
            }
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The formatter will not call `simplify` on expressions, as this could result in infinite recursion.
        match self {
            Fraction { num, den } => {
                if matches!(den, Num(0)) {
                    Undefined.fmt(f)
                } else {
                    let (num, den) = if den.is_negative() {
                        (-num.clone(), -den.clone())
                    } else {
                        (num.clone(), den.clone())
                    };

                    if matches!(den, Num(1)) {
                        num.fmt(f)
                    } else {
                        format!("{num}/{den}").fmt(f)
                    }
                }
            }

            Radical { coef, rad } => {
                if *coef == 0 || *rad == 0 {
                    "0".fmt(f)
                } else {
                    let mut str = match coef {
                        1 => String::new(),
                        -1 => "-".to_owned(),
                        _ => coef.to_string(),
                    };

                    if *rad != 1 {
                        str.push_str(format!("√{rad}").as_str());
                    }

                    str.fmt(f)
                }
            }

            Complex { real, imag } => {
                let mut str = match real {
                    0 => String::new(),
                    _ => real.to_string(),
                };

                if *imag != 0 {
                    str.extend(match imag {
                        ..=-1 => Some('-'),
                        0.. if *real != 0 => Some('+'),
                        _ => None,
                    });

                    let imag_mag = imag.abs(); // Imaginary magnitude
                    if imag_mag > 1 {
                        str.push_str(imag_mag.to_string().as_str());
                    }

                    str.push('𝑖');
                }

                str.fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod format_expr_tests {
    use super::*;

    #[cfg(test)]
    mod format_fraction_tests {
        use super::*;

        #[test]
        fn test_fraction_n_over_1() {
            for n in -10..=10 {
                assert_eq!(
                    Fraction {
                        num: Num(n),
                        den: Num(1),
                    }
                    .to_string(),
                    format!("{n}")
                );
            }
        }

        #[test]
        fn test_fraction_non_simplifying_pos() {
            for n in 1..=10 {
                let d = n * 2;
                assert_eq!(
                    Fraction {
                        num: Num(n),
                        den: Num(d),
                    }
                    .to_string(),
                    format!("{n}/{d}")
                );
            }
        }

        #[test]
        fn test_fraction_non_simplifying_neg_num() {
            for n in 1..=10 {
                let d = n * 2;
                assert_eq!(
                    Fraction {
                        num: Num(-n),
                        den: Num(d),
                    }
                    .to_string(),
                    format!("-{n}/{d}")
                );
            }
        }

        #[test]
        fn test_fraction_non_simplifying_neg_den() {
            for n in 1..=10 {
                let d = n * 2;
                assert_eq!(
                    Fraction {
                        num: Num(n),
                        den: Num(-d),
                    }
                    .to_string(),
                    format!("-{n}/{d}")
                );
            }
        }

        #[test]
        fn test_fraction_non_simplifying_double_neg() {
            for n in 1..=10 {
                let d = n * 2;
                assert_eq!(
                    Fraction {
                        num: Num(-n),
                        den: Num(-d),
                    }
                    .to_string(),
                    format!("{n}/{d}")
                );
            }
        }

        #[test]
        fn test_fraction_non_simplifying_undefined() {
            for n in 1..=10 {
                assert_eq!(
                    Fraction {
                        num: Num(n),
                        den: Num(0),
                    }
                    .to_string(),
                    Undefined.to_string()
                );
            }
        }
    }

    #[cfg(test)]
    mod format_radical_tests {
        use super::*;

        #[test]
        fn test_radical_1() {
            for n in -10..=10 {
                let frac = Fraction {
                    num: Num(n),
                    den: Num(1),
                };
                assert_eq!(frac.to_string(), n.to_string());
            }
        }
    }

    #[cfg(test)]
    mod format_complex_tests {
        use super::*;

        #[test]
        fn test_complex_1() {
            for n in -10..=10 {
                let frac = Fraction {
                    num: Num(n),
                    den: Num(1),
                };
                assert_eq!(frac.to_string(), n.to_string());
            }
        }
    }
}

#[cfg(test)]
mod simplify_fraction_tests {
    use super::*;

    #[test]
    fn test_denominator_of_1() {
        for num in 0..=10 {
            let frac = Fraction {
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
                let frac = Fraction {
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
            let frac = Fraction {
                num: Num(1),
                den: Num(den),
            };
            assert_eq!(frac.simplified(), frac);
        }
    }

    #[test]
    fn test_simplifies_to_half() {
        for num in 1..=10 {
            let frac = Fraction {
                num: Num(num),
                den: Num(num * 2),
            };
            let simplest = Fraction {
                num: Num(1),
                den: Num(2),
            };
            assert_eq!(frac.simplify(), simplest);
        }
    }

    #[test]
    fn test_division_by_zero() {
        for num in 1..=10 {
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            let simple = Fraction {
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
            assert_eq!(Radical { coef, rad: 1 }.simplify(), coef);
        }

        // Simplifies to integer
        for root in 0..10 {
            assert_eq!(
                Radical {
                    coef: root * root,
                    rad: 1
                }
                .simplify(),
                root
            );
        }

        // Can't be simplified
        assert_eq!(
            Radical { coef: 2, rad: 1 }.simplify(),
            Radical { coef: 2, rad: 1 }
        );

        // Simplifies to a radical
        assert_eq!(
            Radical { coef: 8, rad: 1 }.simplify(),
            Radical { coef: 2, rad: 2 }
        );
    }
}
