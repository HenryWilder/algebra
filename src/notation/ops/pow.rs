//! Algebraic exponentiation

use crate::notation::{
    atom::{
        number::Number as Num,
        Atom::{self, *},
    },
    Notation,
};

impl Notation {
    /// Puts one value to the power of another.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// If the result has a [`Huge`] denominator, returns [`Epsilon`].\
    /// If the result has a denominator of 0, returns [`Undefined`].\
    /// If the base and exponent are both negative, returns [`Complex`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    pub fn pow(self, rhs: Self) -> Self {
        match self {
            Notation::Atom(Number(Num { value: 0 | 1 })) => self,
            base => match rhs {
                Notation::Atom(atom) => match atom {
                    Number(Num { value: exp }) => {
                        let mut result = Notation::from(1);
                        for _ in 0..exp.abs() {
                            result = result * base.clone(); // This seems needlessly expensive...
                        }

                        if exp.is_positive() {
                            result
                        } else {
                            Notation::from(1) / result
                        }
                    }
                    Complex => todo!(),
                    Undefined => todo!(),
                    Huge => Notation::from(Huge), // is Huge even or odd??
                    NegativeHuge => Notation::from(Epsilon),
                    Epsilon => todo!(),
                    NegativeEpsilon => todo!(),
                },
                Notation::Expr(_expr) => todo!(),
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
            assert_eq!(Notation::from(1).pow(Notation::from(exp)), 1);
        }
    }
}
