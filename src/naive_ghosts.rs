use std::convert::TryFrom;

pub struct Probability {
    p: f64,
}

impl Probability {
    /// Returns the binary entropy of a Bernoulli variable with probability p as one of its values.
    ///
    /// # Examples
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// let prob = Probability::try_from(0.5).unwrap();
    /// assert_eq!(prob.binary_entropy(), 1.0);
    /// ```
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// let prob = Probability::try_from(0.0).unwrap();
    /// assert_eq!(prob.binary_entropy(), 0.0);
    /// ```
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// let prob = Probability::try_from(2.0);
    /// assert!(prob.is_err());
    /// ```
    ///
    pub fn binary_entropy(&self) -> f64 {
        self.information_content() + self.complement().information_content()
    }

    fn information_content(&self) -> f64 {
        // Follow convention that the information content of a zero-probability event is 0.
        if self.p == 0.0 {
            return 0.0;
        }

        -self.p * f64::log2(self.p)
    }

    /// Returns a Probability representing the complement of this probability (ie. 1 - p)
    ///
    /// # Examples
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// let half = Probability::try_from(0.5).unwrap();
    /// assert_eq!(f64::from(half.complement()), 0.5)
    /// ```
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// let two_thirds = Probability::try_from(2.0 / 3.0).unwrap();
    /// assert_eq!(f64::from(two_thirds.complement().complement()), 2.0 / 3.0);
    /// ```
    ///
    pub fn complement(&self) -> Self {
        Probability { p: 1.0 - self.p }
    }
}

impl TryFrom<f64> for Probability {
    type Error = &'static str;

    /// True iff p is in [0, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// assert!(Probability::try_from(f64::NAN).is_err());
    /// ```
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// assert!(Probability::try_from(-1.0).is_err());
    /// ```
    ///
    /// ```
    /// use probability_ghosts::naive_ghosts::Probability;
    /// use std::convert::TryFrom;
    /// assert!(Probability::try_from(0.5).is_ok());
    /// ```
    ///
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 && value <= 1.0 {
            return Ok(Probability { p: value });
        }
        Err("Probability not in [0, 1]")
    }
}

impl From<Probability> for f64 {
    fn from(prob: Probability) -> Self {
        prob.p
    }
}
