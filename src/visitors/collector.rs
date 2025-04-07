use std::{borrow::Cow, collections::HashSet, str::from_utf8};

use convert_case::{Case, Casing};

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
        if !self.comments.contains(comment) {
            self.comments.insert(comment.to_owned());
        }
    }

    pub fn print_headers(&self) {
        let mut different = self.headers.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_header in different.iter() {
            match from_utf8(&different_header) {
                Ok(str) => different_str.push(Cow::Borrowed(str)),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_header);
                    println!("Invalid UTF-8: {} <- {:?}", str, different_header);
                    different_str.push(str);
                }
            }
        }

        println!("Headers collection\n");

        #[cfg(not(feature = "full-collection-print"))]
        {
            for different_header_str in different_str {
                println!("{}", different_header_str);
            }
        }
        #[cfg(feature = "full-collection-print")]
        {
            println!("Constants for matching (Headers)\n");

            for different_header_str in different_str.iter() {
                println!(
                    "pub const {}: &[u8] = b\"{}\";",
                    different_header_str.to_case(Case::Constant),
                    different_header_str
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
    }

    pub fn print_comments(&self) {
        let mut different = self.comments.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_comment in different.iter() {
            match from_utf8(&different_comment) {
                Ok(str) => different_str.push(Cow::Borrowed(str)),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_comment);
                    println!("Invalid UTF-8: {} <- {:?}", str, different_comment);
                    different_str.push(str);
                }
            }
        }

        println!("Comments collection\n");

        #[cfg(not(feature = "full-collection-print"))]
        {
            for different_comment_str in different_str {
                println!("{}", different_comment_str);
            }
        }
        #[cfg(feature = "full-collection-print")]
        {
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
}
