/// Returns the binary entropy of a Bernoulli variable with probability p as one of its values.
///
/// # Examples
///
/// ```
/// use probability_ghosts::non_total::binary_entropy;
///
/// assert_eq!(binary_entropy(0.5), 1.0);
/// assert_eq!(binary_entropy(0.0), 0.0);
/// ```
pub fn binary_entropy(p: f64) -> f64 {
    let comp_p = complement(p);
    if !is_valid_probability(p) || !is_valid_probability(comp_p) {
        panic!("Probability not between 0 or 1");
    }

    information_content(p) + information_content(comp_p)
}

fn information_content(p: f64) -> f64 {
    if !is_valid_probability(p) {
        panic!("Probability not between 0 or 1");
    }

    // Follow convention that the information content of a zero-probability event is 0.
    if p == 0.0 {
        return 0.0;
    }

    -p * f64::log2(p)
}

/// Returns the complement of this probability (ie. 1 - p) if valid.
/// Panics if the argument given is not in [0, 1].
///
/// # Examples
///
/// ```
/// use probability_ghosts::non_total::{is_valid_probability, complement};
///
/// let half = 0.5;
/// assert!(is_valid_probability(half));
/// assert_eq!(complement(half), 0.5);
///
/// let two_thirds = 2.0 / 3.0;
/// assert!(is_valid_probability(two_thirds));
/// assert_eq!(complement(complement(two_thirds)), 2.0 / 3.0);
/// ```
///
pub fn complement(p: f64) -> f64 {
    if !is_valid_probability(p) {
        panic!("Probability not between 0 or 1");
    }
    1.0 - p
}

/// True iff p is in [0, 1].
///
/// # Examples
///
/// ```
/// use probability_ghosts::non_total::is_valid_probability;
///
/// assert!(!is_valid_probability(f64::NAN));
/// assert!(!is_valid_probability(-1.0));
/// assert!(is_valid_probability(0.5));
/// ```
///
pub fn is_valid_probability(p: f64) -> bool {
    p >= 0.0 && p <= 1.0
}
