//! Functions related to factoring numbers.

/// A factor shared among multiple numbers.
///
/// Produced by [`common_factors()`].
pub struct CommonFactor {
    /// The factor itself.
    pub common: i32,

    /// The numbers multiplied by `common` to result in the original numbers being factored.
    pub associated: Vec<i32>,
}

/// Given a set of numbers, returns the factors shared between them.
pub fn common_factors(ns: Vec<i32>) -> Vec<CommonFactor> {
    let mut factors = Vec::from([CommonFactor {
        common: 1,
        associated: ns.clone(),
    }]);

    let ns_iter = ns.into_iter();
    let abs_ns = ns_iter.clone().map(|n| n.abs());

    for i in 2..=abs_ns.clone().min().unwrap() {
        if abs_ns.clone().all(|x| x % i == 0) {
            factors.push(CommonFactor {
                common: i,
                associated: ns_iter.clone().map(|x| x / i).collect(),
            });
        }
    }

    factors
}

/// A single factor of a number.
///
/// Produced by [`factors()`].
pub struct Factor {
    /// The factor itself.
    pub common: i32,

    /// The number multiplied by `common` to result in the original number being factored.
    pub associated: i32,
}

impl From<CommonFactor> for Factor {
    fn from(CommonFactor { common, associated }: CommonFactor) -> Self {
        debug_assert_eq!(associated.len(), 1);
        Self {
            common,
            associated: associated[0],
        }
    }
}

/// Given a number, returns its factors.
pub fn factors(n: i32) -> Vec<Factor> {
    common_factors(vec![n])
        .iter()
        .map(|CommonFactor { common, associated }| Factor {
            common: *common,
            associated: associated[0],
        })
        .collect()
}

/// Returns the Greatest Common Factor of the provided numbers.
pub fn gcf(ns: Vec<i32>) -> i32 {
    todo!()
}
