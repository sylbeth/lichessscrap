//! Benchmark module for the usage of chrono and time.

#![cfg(any(feature = "chrono", feature = "time"))]

/// Bench to test which of chrono and time is faster for parsing a date and a time.
#[test]
pub fn datetime_bench() {
    use std::time::{Duration, Instant};
    const EXECUTIONS: i32 = 10000000;
    const TEST_DATE: &str = "2020.10.23";
    const TEST_TIME: &str = "12:01:33";

    let (_elapsed_chrono_date, _elapsed_chrono_time, _elapsed_time_date, _elapsed_time_time): (
        Duration,
        Duration,
        Duration,
        Duration,
    );
    let mut time;

    #[cfg(feature = "chrono")]
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            chrono::NaiveDate::parse_from_str(TEST_DATE, "%Y.%m.%d").unwrap();
        }

        _elapsed_chrono_date = time.elapsed();
    }
    #[cfg(feature = "chrono")]
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            chrono::NaiveTime::parse_from_str(TEST_TIME, "%H:%M:%S").unwrap();
        }

        _elapsed_chrono_time = time.elapsed();
    }
    #[cfg(feature = "time")]
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            time::Date::parse(
                TEST_DATE,
                time::macros::format_description!("[year].[month].[day]"),
            )
            .unwrap();
        }

        _elapsed_time_date = time.elapsed();
    }
    #[cfg(feature = "time")]
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            time::Time::parse(
                TEST_TIME,
                time::macros::format_description!("[hour]:[minute]:[second]"),
            )
            .unwrap();
        }

        _elapsed_time_time = time.elapsed();
    }

    #[cfg(feature = "chrono")]
    {
        println!("Chrono Date: {}", _elapsed_chrono_date.as_secs_f32());
        println!("Chrono Time: {}", _elapsed_chrono_time.as_secs_f32());
    }
    #[cfg(feature = "time")]
    {
        println!("Time Date: {}", _elapsed_time_date.as_secs_f32());
        println!("Time Time: {}", _elapsed_time_time.as_secs_f32());
    }

    #[cfg(all(feature = "chrono", feature = "time"))]
    {
        assert!(elapsed_time_date < elapsed_chrono_date);
        assert!(elapsed_time_time < elapsed_chrono_time);
    }
}
