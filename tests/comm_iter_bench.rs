#![cfg(feature = "memchr")]

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
