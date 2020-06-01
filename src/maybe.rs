/// Returns the binary entropy of a Bernoulli variable with probability p as one of its values.
///
/// # Examples
///
/// ```
/// use probability_ghosts::maybe::binary_entropy;
///
/// assert_eq!(binary_entropy(0.5), Some(1.0));
/// assert_eq!(binary_entropy(0.0), Some(0.0));
/// assert_eq!(binary_entropy(2.0), None);
/// ```
///
pub fn binary_entropy(p: f64) -> Option<f64> {
    let prob = to_probability(p)?;
    let comp_prob = complement(prob)?;
    Some(information_content(prob) + information_content(comp_prob))
}

fn information_content(p: f64) -> f64 {
    // Follow convention that the information content of a zero-probability event is 0.
    if p == 0.0 {
        return 0.0;
    }

    -p * f64::log2(p)
}

/// Returns an Option representing the complement of this probability (ie. 1 - p) if valid.
/// Returns None if the argument given is not in [0, 1].
///
/// # Examples
///
/// ```
/// use probability_ghosts::maybe::{to_probability, complement};
///
/// let half = to_probability(0.5).unwrap();
/// assert_eq!(complement(half).unwrap(), 0.5);
///
/// let two_thirds = to_probability(2.0 / 3.0).unwrap();
/// assert_eq!(complement(complement(two_thirds).unwrap()).unwrap(), 2.0 / 3.0);
/// ```
///
pub fn complement(p: f64) -> Option<f64> {
    let prob = to_probability(p)?;
    Some(1.0 - prob)
}

/// True iff p is in [0, 1].
///
/// # Examples
///
/// ```
/// use probability_ghosts::maybe::to_probability;
///
/// assert_eq!(to_probability(f64::NAN), None);
/// assert_eq!(to_probability(-1.0), None);
/// assert_eq!(to_probability(0.5), Some(0.5));
/// ```
///
pub fn to_probability(p: f64) -> Option<f64> {
    if p >= 0.0 && p <= 1.0 {
        return Some(p);
    }
    None
}
