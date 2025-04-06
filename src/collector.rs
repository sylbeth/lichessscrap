use std::collections::HashSet;

use convert_case::{Case, Casing};

#[derive(Debug, Default)]
pub struct Collector {
    pub headers: HashSet<Vec<u8>>,
    pub comments: HashSet<Vec<u8>>,
}

impl Collector {
    #[allow(dead_code)]
    pub fn collect_header(&mut self, header: &[u8]) {
        if !self.headers.contains(header) {
            self.headers.insert(header.to_owned());
        }
    }

    #[allow(dead_code)]
    pub fn collect_comment(&mut self, comment: &[u8]) {
        if !self.comments.contains(comment) {
            self.comments.insert(comment.to_owned());
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
            println!("    {}: String,", different_header_str.to_case(Case::Snake));
        }

        println!("\nStruct fields reset (Headers)\n");

        for different_header_str in different_str.iter() {
            println!(
                "        self.{}.clear();",
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
                "    {}: String,",
                different_comment_str.to_case(Case::Snake)
            );
        }

        println!("\nStruct fields reset (Comments)\n");

        for different_comment_str in different_str.iter() {
            println!(
                "        self.{}.clear();",
                different_comment_str.to_case(Case::Snake)
            );
        }
    }
}
