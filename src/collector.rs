use std::collections::HashSet;

use convert_case::{Case, Casing};
use memchr::memchr_iter;

#[derive(Debug, Default)]
pub struct Collector {
    pub headers: HashSet<Vec<u8>>,
    pub comments: HashSet<Vec<u8>>,
}

impl Collector {
    pub fn collect_header(&mut self, header: &[u8]) {
        if !self.headers.contains(header) {
            self.headers.insert(header.to_owned());
        }
    }
    pub fn collect_comment(&mut self, comment: &[u8]) {
        let mut spaces = memchr_iter(b' ', comment);
        let mut comment_found;
        for start in memchr_iter(b'[', comment) {
            while let Some(end) = spaces.next() {
                if end > start {
                    if comment[start + 1] == b'%' {
                        comment_found = &comment[start + 2..end];
                    } else {
                        comment_found = &comment[start + 1..end];
                    }
                    if !self.comments.contains(comment_found) {
                        self.comments.insert(comment_found.to_owned());
                    }
                    break;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_headers(&self) {
        let mut different = self.headers.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_header in different.iter() {
            match String::from_utf8((*different_header).clone()) {
                Ok(str) => different_str.push(str),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_header);
                    println!("Invalid UTF-8: {} <- {:?}", str, different_header);
                    different_str.push(str.into_owned());
                }
            }
        }

        println!("Headers collection\n");

        println!("Constants for matching (Headers)\n");

        for (different_header, different_header_str) in different.iter().zip(different_str.iter()) {
            println!(
                "pub const {}: &[u8] = &{:?};",
                different_header_str.to_case(Case::Constant),
                different_header
            );
        }

        println!("\nConstants for matching (Headers)\n");

        for different_header_str in different_str.iter() {
            println!(
                "            {} => self.{} = value,",
                different_header_str.to_case(Case::Constant),
                different_header_str.to_case(Case::Snake)
            );
        }

        println!("\nStruct fields (Headers)\n");

        for different_header_str in different_str.iter() {
            println!(
                "    {}: Option<String>,",
                different_header_str.to_case(Case::Snake)
            );
        }
    }

    #[allow(dead_code)]
    pub fn print_comments(&self) {
        let mut different = self.comments.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_comment in different.iter() {
            match String::from_utf8((*different_comment).clone()) {
                Ok(str) => different_str.push(str),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_comment);
                    println!("Invalid UTF-8: {} <- {:?}", str, different_comment);
                    different_str.push(str.into_owned());
                }
            }
        }

        println!("Comments collection\n");

        println!("Constants for matching (Comments)\n");

        for (different_comment, different_comment_str) in different.iter().zip(different_str.iter())
        {
            println!(
                "pub const {}: &[u8] = &{:?};",
                different_comment_str.to_case(Case::Constant),
                different_comment
            );
        }

        println!("\nConstants for matching (Comments)\n");

        for different_comment_str in different_str.iter() {
            println!(
                "            {} => self.{} = value,",
                different_comment_str.to_case(Case::Constant),
                different_comment_str.to_case(Case::Snake)
            );
        }

        println!("\nStruct fields (Comments)\n");

        for different_comment_str in different_str.iter() {
            println!(
                "    {}: Option<String>,",
                different_comment_str.to_case(Case::Snake)
            );
        }
    }
}
