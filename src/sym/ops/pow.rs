//! Algebraic exponentiation

use crate::sym::{
    atom::Atom::{self, *},
    Sym,
};

impl Sym {
    /// Puts one value to the power of another.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// If the result has a [`Huge`] denominator, returns [`Epsilon`].\
    /// If the result has a denominator of 0, returns [`Undefined`].\
    /// If the base and exponent are both negative, returns [`Complex`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    pub fn pow(self, rhs: Self) -> Self {
        match self {
            Sym::Atom(Num(0 | 1)) => self,
            base => match rhs {
                Sym::Atom(atom) => match atom {
                    Num(exp) => {
                        let mut result = Sym::Atom(Num(1));
                        for _ in 0..exp.abs() {
                            result = result * base.clone(); // This seems needlessly expensive...
                        }

                        if exp.is_positive() {
                            result
                        } else {
                            Sym::Atom(Num(1)) / result
                        }
                    }
                    Complex => todo!(),
                    Undefined => todo!(),
                    Huge => Sym::Atom(Huge), // is Huge even or odd??
                    NegHuge => Sym::Atom(Epsilon),
                    Epsilon => todo!(),
                    NegEpsilon => todo!(),
                },
                Sym::Expr(_expr) => todo!(),
            },
        }
    }
}

#[cfg(test)]
mod pow_test {
    use super::*;

    #[test]
    fn test_pow_simple() {
        for exp in 0..=10 {
            assert_eq!(Sym::Atom(Num(1)).pow(Sym::Atom(Num(exp))), 1);
        }
    }
}
