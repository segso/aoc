use std::{error, fmt, ops::Deref};

/// Represents different errors from the creation of a [`Day`].
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Represents when the provided number is not between 1 inclusive and 25 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use year2025::{Day, day::Error};
    ///
    /// let day = Day::from(26);
    /// assert!(day.is_err());
    /// assert_eq!(day.as_ref().unwrap_err(), &Error::DayOutOfBounds(26));
    /// assert_eq!(
    ///     day.unwrap_err().to_string(),
    ///     "day must be between 1 inclusive and 25 inclusive, got '26'"
    /// );
    /// ```
    DayOutOfBounds(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DayOutOfBounds(day) => {
                write!(
                    f,
                    "day must be between 1 inclusive and 25 inclusive, got '{day}'"
                )
            }
        }
    }
}

impl error::Error for Error {}

/// Represents a day number from 1 to 25 inclusive.
/// Useful for advent calendars.
#[derive(Debug, Clone, Copy)]
pub struct Day {
    day: u8,
}

impl Day {
    /// Creates a new [`Day`].
    ///
    /// # Errors
    ///
    /// Will return a [`DayOutOfBounds`] if provided day is equal to 0 or if is
    /// larger than 25.
    ///
    /// [`DayOutOfBounds`]: Error#variant.DayOutOfBounds
    pub fn from(day: u8) -> Result<Self, Error> {
        if day == 0 || day > 25 {
            Err(Error::DayOutOfBounds(day))
        } else {
            Ok(Self { day })
        }
    }
}

impl Deref for Day {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_to_zero() {
        let day = Day::from(0);

        assert!(day.is_err());
        assert_eq!(day.as_ref().unwrap_err(), &Error::DayOutOfBounds(0));
        assert_eq!(
            day.unwrap_err().to_string(),
            "day must be between 1 inclusive and 25 inclusive, got '0'"
        );
    }

    #[test]
    fn below_twenty_six() {
        let day = Day::from(30);

        assert!(day.is_err());
        assert_eq!(day.as_ref().unwrap_err(), &Error::DayOutOfBounds(30));
        assert_eq!(
            day.unwrap_err().to_string(),
            "day must be between 1 inclusive and 25 inclusive, got '30'"
        );
    }

    #[test]
    fn between_one_and_twenty_five() {
        let day = Day::from(1);
        assert!(day.is_ok());
        assert_eq!(*day.unwrap(), 1);

        let day = Day::from(25);
        assert!(day.is_ok());
        assert_eq!(*day.unwrap(), 25);
    }
}
