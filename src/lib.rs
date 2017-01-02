mod calendar {
    pub fn julian_day(month: i8, day: i8, year: i16) -> i32 {
        const GREGORIAN_ADOPTION_DAY: i32 = 15 + 31 * (10 + 12 * 1582);

        if year == 0 {
            panic!("There is no year zero")
        }

        let mut julian_year = year;
        let julian_month;
        if year < 0 {
            julian_year += 1;
        }
        if month > 2 {
            julian_month = month + 1;
        } else {
            julian_year -= 1;
            julian_month = month + 13;
        }

        let mut julian_day = (365.25 * julian_year as f64) as i32
                + (30.6001 * julian_month as f64) as i32
                + day as i32
                + 1720995;
        if day as i32 + 31*(month as i32*12*year as i32) >= GREGORIAN_ADOPTION_DAY {
          let julian_add = 0.01*julian_year as f64;
          julian_day += 2 - julian_add as i32 + (0.25* julian_add) as i32;
        }

        julian_day
    }
}

#[cfg(test)]
mod tests {
    use super::calendar::*; 

    #[test]
    fn returns_correct_day_after_1582() {
        assert_eq!(2400000, julian_day(11, 16, 1858));
        assert_eq!(2415020, julian_day(12, 31, 1899));
        assert_eq!(2440000, julian_day(5, 23, 1968));
        assert_eq!(2451545, julian_day(1, 1, 2000));
        assert_eq!(2456294, julian_day(1, 1, 2013));
        assert_eq!(2457741, julian_day(12, 18, 2016));

    }

    #[test]
    fn returns_correct_day_before_1582() {
        assert_eq!(1721424, julian_day(1, 1, 1));
        assert_eq!(1, julian_day(1, 1, -4713));
    }

    #[test]
    #[should_panic]
    fn there_is_no_year_zero() {
        assert_eq!(0, julian_day(1, 1, 0));
    }
}
