//! Algebraic division

use crate::notation::{
    atom::{
        number::Number,
        Atom::{self, *},
    },
    expr::fraction::Fraction,
    Notation,
};

impl std::ops::Div for Notation {
    type Output = Self;

    /// Divide two values.
    ///
    /// If the result overflows, returns [`Huge`].\
    /// If the result has a [`Huge`] denominator, returns [`Epsilon`].\
    /// If the result has a denominator of 0, returns [`Undefined`].\
    /// Otherwise returns a [`Number`] with the value of the result.
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Notation::Atom(atom_a), Notation::Atom(atom_b)) => match (atom_a, atom_b) {
                // Basic
                (Number(Number { value: num }), Number(Number { value: den })) => {
                    match (num, den) {
                        (_, 0) => Notation::from(Undefined),
                        (_, 1) => Notation::from(num),
                        (_, -1) => Notation::from(-num), // Todo: i32::MAX.neg() overflows
                        _ => Notation::from(Fraction::new(num, den)),
                    }
                }

                // 𝑛/𝑖 = 𝑖?
                (_, Complex) | (Complex, _) => Notation::from(Complex),

                // 𝑛/∅ = ∅
                // 𝑛/0 = ∅
                (_, Undefined) | (Undefined, _) | (_, Number(Number { value: 0 })) => {
                    Notation::from(Undefined)
                }

                // 𝓗/𝑛 | -𝓗/-𝑛 = 𝓗
                // -𝓗/𝑛 | 𝓗/-𝑛 = -𝓗
                // ε/𝑛 | -ε/-𝑛 = ε
                // -ε/𝑛 | ε/-𝑛 = -ε
                (num @ (Huge | NegativeHuge | Epsilon | NegativeEpsilon), Number(_)) => {
                    match (num, den) {
                        (_, 0) => Notation::from(Undefined),
                        (_, 1) => Notation::from(num),
                        _ => Notation::from(Fraction::new(num, den)),
                    }
                }

                (
                    Number(Number { value: num }),
                    den @ (Huge | NegativeHuge | Epsilon | NegativeEpsilon),
                ) => match num {
                    // 0/𝑛 = 0
                    // where 𝑛 is (-∞, 0) ∪ (0, ∞)
                    0 => Notation::from(0),

                    1.. => match den {
                        // 𝑛/𝓗 = ε
                        Huge => Notation::from(Epsilon),
                        // 𝑛/-𝓗 = -ε
                        NegativeHuge => Notation::from(NegativeEpsilon),
                        // 𝑛/ε = 𝓗
                        Epsilon => Notation::from(Huge),
                        // 𝑛/-ε = -𝓗
                        NegativeEpsilon => Notation::from(NegativeHuge),

                        _ => unreachable!(),
                    },

                    ..=-1 => match den {
                        // -𝑛/𝓗 = -ε
                        Huge => Notation::from(NegativeEpsilon),
                        // -𝑛/-𝓗 = ε
                        NegativeHuge => Notation::from(Epsilon),
                        // -𝑛/ε = -𝓗
                        Epsilon => Notation::from(NegativeHuge),
                        // -𝑛/-ε = 𝓗
                        NegativeEpsilon => Notation::from(Huge),

                        _ => unreachable!(),
                    },
                },

                (
                    num @ (Epsilon | NegativeEpsilon | Huge | NegativeHuge),
                    den @ (Huge | NegativeHuge),
                ) => Notation::from(match den {
                    Huge => num,
                    NegativeHuge => -num,

                    _ => unreachable!(),
                }),

                // ε/ε | -ε/-ε = 𝓗
                (Epsilon, Epsilon) | (NegativeEpsilon, NegativeEpsilon) => Notation::from(Huge),

                // -ε/ε | ε/-ε = -𝓗
                (NegativeEpsilon, Epsilon) | (Epsilon, NegativeEpsilon) => {
                    Notation::from(NegativeHuge)
                }
            },

            _ => todo!(),
        }
    }
}
