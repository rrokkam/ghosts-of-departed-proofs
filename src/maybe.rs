/// Returns the binary entropy of a Bernoulli variable with probability p as one of its values.
///
/// # Examples
///
/// ```
/// # use probability_ghosts::maybe::binary_entropy;
/// assert_eq!(binary_entropy(0.5), Some(1.0));
/// ```
///
/// ```
/// # use probability_ghosts::maybe::binary_entropy;
/// assert_eq!(binary_entropy(0.0), Some(0.0));
/// ```
///
/// ```
/// # use probability_ghosts::maybe::binary_entropy;
/// assert_eq!(binary_entropy(2.0), None);
/// ```
///
pub fn binary_entropy(p: f64) -> Option<f64> {
    let prob = to_probability(p)?;
    let comp_prob = to_probability(1.0-p)?
    Some(information_content(prob) + information_content(comp_prob))
}

fn information_content(p: f64) -> f64 {
    // Follow convention that the information content of a zero-probability event is 0.
    if p == 0.0 {
        return 0.0;
    }

    -p * f64::log2(p)
}

/// True iff p is in [0, 1].
///
/// # Examples
///
/// ```
/// # use probability_ghosts::maybe::to_probability;
/// assert_eq!(to_probability(f64::NAN), None);
/// ```
///
/// ```
/// # use probability_ghosts::maybe::to_probability;
/// assert_eq!(to_probability(-1.0), None);
/// ```
///
/// ```
/// # use probability_ghosts::maybe::to_probability;
/// assert_eq!(to_probability(0.5), Some(0.5));
/// ```
///
pub fn to_probability(p: f64) -> Option<f64> {
    if p >= 0.0 && p <= 1.0 {
        return Some(p);
    }
    None
}
