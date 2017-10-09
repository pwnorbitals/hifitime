use super::traits;
use super::instant::{Era, Instant};

/// J1900_OFFSET determines the offset in julian days between 01 Jan 1900 at midnight and the
/// Modified Julian Day at Epoch. NOTE: The J1900.0 offset in Vallado is different from the one
/// given by NIST. This library uses the NIST provided value (one day shorter).
pub const J1900_OFFSET: f64 = 15020.0;
/// J2000_OFFSET determines the offset in julian days between 01 Jan 2000 at midnight and the
/// Modified Julian Day at Epoch. NOTE: The J1900.0 offset in Vallado is different from the one
/// given by NIST. This library uses the NIST provided value (one day shorter).
pub const J2000_OFFSET: f64 = 51545.0;
/// DAYS_PER_YEAR corresponds to the number of days per year in the Julian calendar. This is fixed.
pub const DAYS_PER_YEAR: f64 = 365.25;
/// SECONDS_PER_DAY defines the number of seconds per day.
pub const SECONDS_PER_DAY: f64 = 86400.0;

#[derive(Copy, Clone, Debug)]
pub struct ModifiedJulian {
    pub days: f64,
}

impl ModifiedJulian {
    /// `julian_days` returns the true Julian days from epoch 01 Jan -4713, 12:00
    /// as explained in "Fundamentals of astrodynamics and applications", Vallado et al.
    /// 4th edition, page 182.
    pub fn julian_days(self) -> f64 {
        self.days + 2_400_000.0
    }
}

impl traits::TimeSystem for ModifiedJulian {
    /// `from_instant` converts an Instant to a ModifiedJulian as detailed
    /// in https://www.ietf.org/timezones/data/leap-seconds.list , specifically the following
    /// quote:
    /// The NTP timestamps are in units of seconds since the NTP epoch,
    /// which is 1 January 1900, 00:00:00. The Modified Julian Day number
    /// corresponding to the NTP time stamp, X, can be computed as
    ///
    /// X/86400 + 15020
    ///
    /// where the first term converts seconds to days and the second
    /// term adds the MJD corresponding to the time origin defined above.
    /// The integer portion of the result is the integer MJD for that
    /// day, and any remainder is the time of day, expressed as the
    /// fraction of the day since 0 hours UTC. The conversion from day
    /// fraction to seconds or to hours, minutes, and seconds may involve
    /// rounding or truncation, depending on the method used in the
    /// computation.
    fn from_instant(instant: Instant) -> ModifiedJulian {
        let modifier: f64;
        if instant.era() == Era::Present {
            modifier = 1.0;
        } else {
            modifier = -1.0;
        }
        ModifiedJulian {
            days: J1900_OFFSET + modifier * (instant.secs() as f64) / SECONDS_PER_DAY +
                instant.nanos() as f64 * 1e-9,
        }
    }

    fn as_instant(self) -> Instant {
        let era: Era;
        let modifier: f64;
        if self.days >= J1900_OFFSET {
            era = Era::Present;
            modifier = 1.0;
        } else {
            era = Era::Past;
            modifier = -1.0;
        }
        let secs_frac = (self.days - J1900_OFFSET) * SECONDS_PER_DAY * modifier;
        let seconds = secs_frac.round();
        let nanos = (secs_frac - seconds) * 1e9 / (SECONDS_PER_DAY * modifier);
        Instant::new(seconds as u64, nanos.round() as u32, era)
    }
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
