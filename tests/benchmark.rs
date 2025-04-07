#[test]
pub fn num_write_bench() {
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 10000000;
    const FILE: &str = "test.tmp";
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

#[test]
pub fn string_write_bench() {
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 10000000;
    const FILE: &str = "test.tmp";
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
