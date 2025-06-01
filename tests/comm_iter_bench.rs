//! Benchmark module for the usage of memchr.

use pretty_assertions::assert_eq;

/// Auxiliary function to test a memchr based comment iterator.
pub fn comm_iter_memchr<T: std::io::Write>(file: &mut T, comment: &[u8]) {
    use memchr::memchr_iter;
    let mut spaces = memchr_iter(b' ', comment);
    let brackets = memchr_iter(b'[', comment).zip(memchr_iter(b']', comment));
    for (start, end) in brackets {
        while let Some(sep) = spaces.next() {
            if (start..end).contains(&sep) {
                write!(
                    file,
                    "{:?}{:?}",
                    &comment[start + 1..sep],
                    &comment[sep + 1..end]
                )
                .unwrap()
            }
        }
    }
}

/// Auxiliary function to test a manual comment iterator.
pub fn comm_iter_manual<T: std::io::Write>(file: &mut T, comment: &[u8]) {
    let (mut start, mut sep) = (0, 0);
    for (i, c) in comment.iter().enumerate() {
        match c {
            b' ' => sep = i,
            b'[' => start = i + 1,
            b']' => write!(file, "{:?}{:?}", &comment[start..sep], &comment[sep + 1..i]).unwrap(),
            _ => (),
        }
    }
}

/// Bench to test which of comm_iter_memchr or com_iter_manual is faster.
#[test]
pub fn comm_iter_bench() {
    use self::{comm_iter_manual, comm_iter_memchr};
    use std::{
        fs::{File, remove_file},
        io::BufWriter,
        time::Instant,
    };
    const EXECUTIONS: i32 = 10000000;
    const FILE: &str = "test.tmp";
    const TEST_COMMENT: &[u8] = b" [%eval 0.17] [%clk 0:00:30] ";

    let (elapsed_memchr, elapsed_manual);
    let mut time;

    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            comm_iter_memchr(&mut file, TEST_COMMENT);
        }

        elapsed_memchr = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(FILE).unwrap());
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            comm_iter_manual(&mut file, TEST_COMMENT);
        }

        elapsed_manual = time.elapsed();
    }

    println!("Memchr: {}", elapsed_memchr.as_secs_f32());
    println!("Manual: {}", elapsed_manual.as_secs_f32());

    let _ = remove_file(FILE);

    assert!(elapsed_memchr < elapsed_manual);
}

/// Auxiliary function to test the finding of the separator of a time control manually.
pub fn time_control_iter_manual(time_control: &[u8]) -> Option<usize> {
    for (i, c) in time_control.iter().enumerate() {
        if *c == b'+' {
            return Some(i);
        }
    }
    None
}

/// Bench to test which of time_control_iter_manual, memchr or find is faster.
#[test]
pub fn time_control_iter_bench() {
    use std::time::Instant;
    const EXECUTIONS: i32 = 100000000;
    let test_time_control: String = "900+900".into();

    let (elapsed_memchr, elapsed_manual, elapsed_str);
    let mut time;

    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            assert_eq!(
                memchr::memchr(b'+', test_time_control.as_bytes()).unwrap(),
                3
            );
        }

        elapsed_memchr = time.elapsed();
    }
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            assert_eq!(
                time_control_iter_manual(test_time_control.as_bytes()).unwrap(),
                3
            );
        }

        elapsed_manual = time.elapsed();
    }
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            assert_eq!(test_time_control.find('+').unwrap(), 3);
        }

        elapsed_str = time.elapsed();
    }

    println!("Memchr: {}", elapsed_memchr.as_secs_f32());
    println!("Manual: {}", elapsed_manual.as_secs_f32());
    println!("Str: {}", elapsed_str.as_secs_f32());

    assert!(elapsed_memchr < elapsed_str);
    assert!(elapsed_manual < elapsed_memchr);
}

/// Bench to test which of time_control_iter_manual, split_once is faster.
#[test]
pub fn time_control_split_bench() {
    use std::time::Instant;
    const EXECUTIONS: i32 = 100000000;
    let test_time_control: String = "900+900".into();

    let (elapsed_manual, elapsed_str);
    let mut time;
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            let i = time_control_iter_manual(test_time_control.as_bytes()).unwrap();
            assert_eq!(
                (&test_time_control[..i], &test_time_control[i + 1..]),
                ("900", "900")
            );
        }

        elapsed_manual = time.elapsed();
    }
    {
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            assert_eq!(
                test_time_control.as_str().split_once('+').unwrap(),
                ("900", "900")
            );
        }

        elapsed_str = time.elapsed();
    }
    println!("Manual: {}", elapsed_manual.as_secs_f32());
    println!("Str: {}", elapsed_str.as_secs_f32());

    assert!(elapsed_manual < elapsed_str);
}
