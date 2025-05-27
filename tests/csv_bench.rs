//! Benchmark module for the usage of csv and serde.

#![cfg(feature = "csv")]

#[test]
/// Bench to test which of a manual approach, a bytes record based approach, a string record based approach and a serde based approach is faster for the writing of the games csv file.
pub fn game_csv_bench() {
    use csv::{Reader, Writer};
    use lichess::data::game::Game;
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 1000000;
    let game: Game = Game {
        game_id: 9000,
        site: "https://lichess.org/ABCDEFGH".into(),
        time_control: "300+0".into(),
        result: "0-1".into(),
        termination: "Time forfeit".into(),
        date: "2017.04.01".into(),
        utc_date: "2017.04.01".into(),
        utc_time: "11:32:01".into(),
        opening: "Sicilian Defense: Old Sicilian".into(),
        eco: "B30".into(),
        event: "Rated Bullet tournament".into(),
        round: "-".into(),
        white: "Abbot".into(),
        white_elo: "2100".into(),
        white_title: "FM".into(),
        white_rating_diff: "-4".into(),
        black: "Costello".into(),
        black_elo: "2000".into(),
        black_title: String::new(),
        black_rating_diff: "+1".into(),
    };
    #[cfg(feature = "serde")]
    const SERIALIZED: &str = "games_ser.tmp";
    const CSVB: &str = "games_csvb.tmp";
    const CSVS: &str = "games_csvs.tmp";
    const MANUAL: &str = "games_man.tmp";

    #[cfg(feature = "serde")]
    let elapsed_serialized;
    let (elapsed_csvb, elapsed_csvs, elapsed_manual);
    let mut time;

    #[cfg(feature = "serde")]
    {
        let mut file = Writer::from_path(SERIALIZED).unwrap();
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            file.serialize(Game {
                game_id: 9000,
                site: "https://lichess.org/ABCDEFGH".into(),
                time_control: "300+0".into(),
                result: "0-1".into(),
                termination: "Time forfeit".into(),
                date: "2017.04.01".into(),
                utc_date: "2017.04.01".into(),
                utc_time: "11:32:01".into(),
                opening: "Sicilian Defense: Old Sicilian".into(),
                eco: "B30".into(),
                event: "Rated Bullet tournament".into(),
                round: "-".into(),
                white: "Abbot".into(),
                white_elo: "2100".into(),
                white_title: "FM".into(),
                white_rating_diff: "-4".into(),
                black: "Costello".into(),
                black_elo: "2000".into(),
                black_title: String::new(),
                black_rating_diff: "+1".into(),
            })
            .unwrap();
        }

        elapsed_serialized = time.elapsed();
    }
    {
        let mut file = Writer::from_path(CSVB).unwrap();
        time = Instant::now();

        file.write_byte_record(
            &[
                "GameId",
                "Site",
                "TimeControl",
                "Result",
                "Termination",
                "Date",
                "UTCDate",
                "UTCTime",
                "Opening",
                "Eco",
                "Event",
                "Round",
                "White",
                "WhiteElo",
                "WhiteRatingDiff",
                "WhiteTitle",
                "Black",
                "BlackElo",
                "BlackRatingDiff",
                "BlackTitle",
            ]
            .as_slice()
            .into(),
        )
        .unwrap();

        for _ in 0..EXECUTIONS {
            file.write_byte_record(
                &[
                    &format!("{}", game.game_id),
                    &game.site,
                    &game.time_control,
                    &game.result,
                    &game.termination,
                    &game.date,
                    &game.utc_date,
                    &game.utc_time,
                    &game.opening,
                    &game.eco,
                    &game.event,
                    &game.round,
                    &game.white,
                    &game.white_elo,
                    &game.white_rating_diff,
                    &game.white_title,
                    &game.black,
                    &game.black_elo,
                    &game.black_rating_diff,
                    &game.black_title,
                ]
                .as_slice()
                .into(),
            )
            .unwrap();
        }

        elapsed_csvb = time.elapsed();
    }
    {
        let mut file = Writer::from_path(CSVS).unwrap();
        time = Instant::now();

        file.write_record(&[
            "GameId",
            "Site",
            "TimeControl",
            "Result",
            "Termination",
            "Date",
            "UTCDate",
            "UTCTime",
            "Opening",
            "Eco",
            "Event",
            "Round",
            "White",
            "WhiteElo",
            "WhiteRatingDiff",
            "WhiteTitle",
            "Black",
            "BlackElo",
            "BlackRatingDiff",
            "BlackTitle",
        ])
        .unwrap();

        for _ in 0..EXECUTIONS {
            file.write_record(&[
                &format!("{}", game.game_id),
                &game.site,
                &game.time_control,
                &game.result,
                &game.termination,
                &game.date,
                &game.utc_date,
                &game.utc_time,
                &game.opening,
                &game.eco,
                &game.event,
                &game.round,
                &game.white,
                &game.white_elo,
                &game.white_rating_diff,
                &game.white_title,
                &game.black,
                &game.black_elo,
                &game.black_rating_diff,
                &game.black_title,
            ])
            .unwrap();
        }

        elapsed_csvs = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(MANUAL).unwrap());
        time = Instant::now();

        file.write(b"GameId,Site,TimeControl,Result,Termination,Date,UTCDate,UTCTime,Opening,Eco,Event,Round,White,WhiteElo,WhiteRatingDiff,WhiteTitle,Black,BlackElo,BlackRatingDiff,BlackTitle").unwrap();

        for _ in 0..EXECUTIONS {
            file.write(b"\n").unwrap();
            write!(file, "{}", game.game_id).unwrap();
            file.write(b",").unwrap();
            file.write(game.site.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.time_control.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.result.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.termination.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.date.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.utc_date.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.utc_time.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.opening.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.eco.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.event.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.round.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.white.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.white_elo.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.white_rating_diff.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.white_title.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.black.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.black_elo.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.black_rating_diff.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(game.black_title.as_bytes()).unwrap();
        }

        elapsed_manual = time.elapsed();
    }

    #[cfg(feature = "serde")]
    println!("Serialized: {}", elapsed_serialized.as_secs_f32());
    println!("Bytes CSV: {}", elapsed_csvb.as_secs_f32());
    println!("String CSV: {}", elapsed_csvs.as_secs_f32());
    println!("Manual: {}", elapsed_manual.as_secs_f32());

    #[cfg(feature = "serde")]
    let mut ser_reader = Reader::from_path(SERIALIZED).unwrap();
    let mut csvb_reader = Reader::from_path(CSVB).unwrap();
    let mut csvs_reader = Reader::from_path(CSVS).unwrap();
    let mut manual_reader = Reader::from_path(MANUAL).unwrap();

    #[cfg(feature = "serde")]
    {
        let (ser, manual, csvb, csvs) = (
            ser_reader.headers().unwrap(),
            manual_reader.headers().unwrap(),
            csvb_reader.headers().unwrap(),
            csvs_reader.headers().unwrap(),
        );
        assert_eq!(ser, manual);
        assert_eq!(csvs, csvb);
        assert_eq!(ser, csvs);
    }
    #[cfg(not(feature = "serde"))]
    {
        let (manual, csvb, csvs) = (
            manual_reader.headers().unwrap(),
            csvb_reader.headers().unwrap(),
            csvs_reader.headers().unwrap(),
        );
        assert_eq!(csvs, csvb);
        assert_eq!(manual, csvs);
    }

    #[cfg(feature = "serde")]
    for ((ser, manual), (csvb, csvs)) in ser_reader
        .records()
        .zip(manual_reader.records())
        .zip(csvb_reader.records().zip(csvs_reader.records()))
    {
        let (ser, manual, csvb, csvs) =
            (ser.unwrap(), manual.unwrap(), csvb.unwrap(), csvs.unwrap());
        assert_eq!(ser, manual);
        assert_eq!(csvs, csvb);
        assert_eq!(ser, csvs);
    }
    #[cfg(not(feature = "serde"))]
    for (manual, (csvb, csvs)) in manual_reader
        .records()
        .zip(csvb_reader.records().zip(csvs_reader.records()))
    {
        let (manual, csvb, csvs) = (manual.unwrap(), csvb.unwrap(), csvs.unwrap());
        assert_eq!(csvs, csvb);
        assert_eq!(manual, csvs);
    }

    #[cfg(feature = "serde")]
    let _ = remove_file(SERIALIZED);
    let _ = remove_file(CSVB);
    let _ = remove_file(CSVS);
    let _ = remove_file(MANUAL);

    #[cfg(feature = "serde")]
    assert!(
        elapsed_manual
            < *[elapsed_csvb, elapsed_csvs, elapsed_serialized]
                .iter()
                .min()
                .unwrap()
    );
    #[cfg(not(feature = "serde"))]
    assert!(elapsed_manual < *[elapsed_csvb, elapsed_csvs].iter().min().unwrap());
}

#[test]
/// Bench to test which of a manual approach, a bytes record based approach, a string record based approach and a serde based approach is faster for the writing of the moves csv file.
pub fn move_csv_bench() {
    use csv::{Reader, Writer};
    use lichess::data::r#move::Move;
    use pgn_reader::Nag;
    use std::{
        fs::{File, remove_file},
        io::{BufWriter, Write},
        time::Instant,
    };
    const EXECUTIONS: i32 = 1000000;
    let r#move: Move = Move {
        game_id: 9000,
        num: 30,
        san: "Nbd2".into(),
        nag: Some(Nag::BLUNDER.0),
        eval: "0.17".into(),
        clk: "0:00:30".into(),
    };
    #[cfg(feature = "serde")]
    const SERIALIZED: &str = "moves_ser.tmp";
    const CSVB: &str = "moves_csvb.tmp";
    const CSVS: &str = "moves_csvs.tmp";
    const MANUAL: &str = "moves_man.tmp";

    #[cfg(feature = "serde")]
    let elapsed_serialized;
    let (elapsed_csvb, elapsed_csvs, elapsed_manual);
    let mut time;

    #[cfg(feature = "serde")]
    {
        let mut file = Writer::from_path(SERIALIZED).unwrap();
        time = Instant::now();
        for _ in 0..EXECUTIONS {
            file.serialize(Move {
                game_id: 9000,
                num: 30,
                san: "Nbd2".into(),
                nag: Some(Nag::BLUNDER.0),
                eval: "0.17".into(),
                clk: "0:00:30".into(),
            })
            .unwrap();
        }

        elapsed_serialized = time.elapsed();
    }
    {
        let mut file = Writer::from_path(CSVB).unwrap();
        time = Instant::now();

        file.write_byte_record(
            &["GameId", "Num", "San", "Nag", "Eval", "Clk"]
                .as_slice()
                .into(),
        )
        .unwrap();

        for _ in 0..EXECUTIONS {
            file.write_byte_record(
                &[
                    &format!("{}", r#move.game_id),
                    &format!("{}", r#move.num),
                    &r#move.san,
                    &r#move.nag.map(|nag| format!("{}", nag)).unwrap_or_default(),
                    &r#move.eval,
                    &r#move.clk,
                ]
                .as_slice()
                .into(),
            )
            .unwrap();
        }

        elapsed_csvb = time.elapsed();
    }
    {
        let mut file = Writer::from_path(CSVS).unwrap();
        time = Instant::now();

        file.write_record(&["GameId", "Num", "San", "Nag", "Eval", "Clk"])
            .unwrap();

        for _ in 0..EXECUTIONS {
            file.write_record(&[
                &format!("{}", r#move.game_id),
                &format!("{}", r#move.num),
                &r#move.san,
                &r#move.nag.map(|nag| format!("{}", nag)).unwrap_or_default(),
                &r#move.eval,
                &r#move.clk,
            ])
            .unwrap();
        }

        elapsed_csvs = time.elapsed();
    }
    {
        let mut file = BufWriter::new(File::create(MANUAL).unwrap());
        time = Instant::now();

        file.write(b"GameId,Num,San,Nag,Eval,Clk").unwrap();

        for _ in 0..EXECUTIONS {
            file.write(b"\n").unwrap();
            write!(file, "{}", r#move.game_id).unwrap();
            file.write(b",").unwrap();
            write!(file, "{}", r#move.num).unwrap();
            file.write(b",").unwrap();
            file.write(r#move.san.as_bytes()).unwrap();
            file.write(b",").unwrap();
            if let Some(nag) = &r#move.nag {
                write!(file, "{}", nag).unwrap();
            }
            file.write(b",").unwrap();
            file.write(r#move.eval.as_bytes()).unwrap();
            file.write(b",").unwrap();
            file.write(r#move.clk.as_bytes()).unwrap();
        }

        elapsed_manual = time.elapsed();
    }

    #[cfg(feature = "serde")]
    println!("Serialized: {}", elapsed_serialized.as_secs_f32());
    println!("Bytes CSV: {}", elapsed_csvb.as_secs_f32());
    println!("String CSV: {}", elapsed_csvs.as_secs_f32());
    println!("Manual: {}", elapsed_manual.as_secs_f32());

    #[cfg(feature = "serde")]
    let mut ser_reader = Reader::from_path(SERIALIZED).unwrap();
    let mut csvb_reader = Reader::from_path(CSVB).unwrap();
    let mut csvs_reader = Reader::from_path(CSVS).unwrap();
    let mut manual_reader = Reader::from_path(MANUAL).unwrap();

    #[cfg(feature = "serde")]
    {
        let (ser, manual, csvb, csvs) = (
            ser_reader.headers().unwrap(),
            manual_reader.headers().unwrap(),
            csvb_reader.headers().unwrap(),
            csvs_reader.headers().unwrap(),
        );
        assert_eq!(ser, manual);
        assert_eq!(csvs, csvb);
        assert_eq!(ser, csvs);
    }
    #[cfg(not(feature = "serde"))]
    {
        let (manual, csvb, csvs) = (
            manual_reader.headers().unwrap(),
            csvb_reader.headers().unwrap(),
            csvs_reader.headers().unwrap(),
        );
        assert_eq!(csvs, csvb);
        assert_eq!(manual, csvs);
    }

    #[cfg(feature = "serde")]
    for ((ser, manual), (csvb, csvs)) in ser_reader
        .records()
        .zip(manual_reader.records())
        .zip(csvb_reader.records().zip(csvs_reader.records()))
    {
        let (ser, manual, csvb, csvs) =
            (ser.unwrap(), manual.unwrap(), csvb.unwrap(), csvs.unwrap());
        assert_eq!(ser, manual);
        assert_eq!(csvs, csvb);
        assert_eq!(ser, csvs);
    }
    #[cfg(not(feature = "serde"))]
    for (manual, (csvb, csvs)) in manual_reader
        .records()
        .zip(csvb_reader.records().zip(csvs_reader.records()))
    {
        let (manual, csvb, csvs) = (manual.unwrap(), csvb.unwrap(), csvs.unwrap());
        assert_eq!(csvs, csvb);
        assert_eq!(manual, csvs);
    }

    #[cfg(feature = "serde")]
    let _ = remove_file(SERIALIZED);
    let _ = remove_file(CSVB);
    let _ = remove_file(CSVS);
    let _ = remove_file(MANUAL);

    #[cfg(feature = "serde")]
    assert!(
        elapsed_manual
            < *[elapsed_csvb, elapsed_csvs, elapsed_serialized]
                .iter()
                .min()
                .unwrap()
    );
    #[cfg(not(feature = "serde"))]
    assert!(elapsed_manual < *[elapsed_csvb, elapsed_csvs].iter().min().unwrap());
}
