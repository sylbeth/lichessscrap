//! Benchmark module for miscellaneous benches.

#[cfg(feature = "memchr")]
mod comm_iter_bench;
#[cfg(feature = "csv")]
mod csv_bench;
#[cfg(any(feature = "chrono", feature = "time"))]
mod datetime_bench;

/// Bench to test which of String::from_utf8_lossy or str::from_utf8 is faster.
#[test]
pub fn utf8_conv_bench() {
    use std::{borrow::Cow, str::from_utf8, time::Instant};
    const EXECUTIONS: i32 = 100000000;
    const TEST_SLICE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let (elapsed_string, elapsed_cow, elapsed_str);
    let mut time;
    let mut buffer = String::new();

    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            buffer.clear();
            buffer = match String::from_utf8(TEST_SLICE.to_owned()) {
                Ok(str) => str,
                Err(_) => {
                    let str = String::from_utf8_lossy(TEST_SLICE);
                    println!("Invalid UTF-8: {} <- {:?}", str, TEST_SLICE);
                    str.into_owned()
                }
            };
        }

        elapsed_string = time.elapsed();
    }
    {
        time = Instant::now();

        for _ in 0..EXECUTIONS {
            buffer.clear();
            buffer.push_str(&match from_utf8(TEST_SLICE) {
                Ok(str) => Cow::Borrowed(str),
                Err(_) => {
                    let str = String::from_utf8_lossy(TEST_SLICE);
                    println!("Invalid UTF-8: {} <- {:?}", str, TEST_SLICE);
                    str
                }
            });
        }

        elapsed_cow = time.elapsed();
    }
    {
        time = Instant::now();

        for _ in 0..EXECUTIONS {
            buffer.clear();
            buffer.push_str(match from_utf8(TEST_SLICE) {
                Ok(str) => str,
                Err(_) => {
                    let str = String::from_utf8_lossy(TEST_SLICE);
                    panic!("Invalid UTF-8: {} <- {:?}", str, TEST_SLICE);
                }
            });
        }

        elapsed_str = time.elapsed();
    }

    println!("String: {}", elapsed_string.as_secs_f32());
    println!("CoW: {}", elapsed_cow.as_secs_f32());
    println!("Str: {}", elapsed_str.as_secs_f32());

    assert!(elapsed_cow < elapsed_string);
    assert!(elapsed_str < elapsed_cow);
}

/// Bench to test which of write!, to_string or format is faster.
#[test]
pub fn num_write_bench() {
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 10000000;
    const FILE: &str = "test_num.tmp";
    const TEST_NUM: u32 = 0123456789;

    let (elapsed_string, elapsed_format, elapsed_write);
    let mut time;

    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            file.write(TEST_NUM.to_string().as_bytes()).unwrap();
        }

        elapsed_string = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();

        for _ in 0..EXECUTIONS {
            file.write(format!("{}", TEST_NUM).as_bytes()).unwrap();
        }

        elapsed_format = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();

        for _ in 0..EXECUTIONS {
            write!(file, "{}", TEST_NUM).unwrap();
        }

        elapsed_write = time.elapsed();
    }

    println!("String: {}", elapsed_string.as_secs_f32());
    println!("Format: {}", elapsed_format.as_secs_f32());
    println!("Write: {}", elapsed_write.as_secs_f32());

    let _ = remove_file(FILE);

    assert!(elapsed_write < elapsed_string);
    assert!(elapsed_write < elapsed_format);
}

/// Bench to test which of write! or as_bytes is faster.
#[test]
pub fn string_write_bench() {
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 10000000;
    const FILE: &str = "test_string.tmp";
    const TEST_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let (elapsed_string, elapsed_write);
    let mut time;

    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            file.write(TEST_STR.as_bytes()).unwrap();
        }

        elapsed_string = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();

        for _ in 0..EXECUTIONS {
            write!(file, "{}", TEST_STR).unwrap();
        }

        elapsed_write = time.elapsed();
    }

    println!("String: {}", elapsed_string.as_secs_f32());
    println!("Write: {}", elapsed_write.as_secs_f32());

    let _ = remove_file(FILE);

    assert!(elapsed_string < elapsed_write);
}
