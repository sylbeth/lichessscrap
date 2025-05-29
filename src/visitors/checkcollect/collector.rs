//! Collector for the PGN reader. Used for finding every accessible field in a PGN file.

use std::{
    borrow::Cow,
    collections::HashSet,
    io::{self, Write},
    str::from_utf8,
};

use log::error;

#[cfg(feature = "full-collect")]
use convert_case::{Case, Casing};

/// Skips the first letter of the text if it's a percentage.
pub fn skip_percentage(text: &str) -> &str {
    return &text[if text.as_bytes()[0] == b'%' { 1 } else { 0 }..];
}

/// Collector for the PGN reader.
#[derive(Debug, Default)]
pub struct Collector {
    /// Collected headers.
    pub headers: HashSet<Vec<u8>>,
    /// Collected comments.
    pub comments: HashSet<Vec<u8>>,
}

impl Collector {
    /// Collects a given header.
    pub fn collect_header(&mut self, header: &[u8]) {
        if !self.headers.contains(header) {
            self.headers.insert(header.to_owned());
        }
    }

    /// Collects a given comment.
    pub fn collect_comment(&mut self, comment: &[u8]) {
        if !self.comments.contains(comment) {
            self.comments.insert(comment.to_owned());
        }
    }

    /// Writes the headers and their generated code to a writer.
    pub fn write_headers<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut different = self.headers.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_header in different.iter() {
            match from_utf8(different_header) {
                Ok(str) => different_str.push(Cow::Borrowed(str)),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_header);
                    error!("Invalid UTF-8 header: {} <- {:?}", str, different_header);
                    different_str.push(str);
                }
            }
        }

        writeln!(writer, "// Headers collection.\n")?;

        #[cfg(not(feature = "full-collect"))]
        {
            for different_header_str in different_str {
                writeln!(writer, "{}", different_header_str)?;
            }
        }
        #[cfg(feature = "full-collect")]
        {
            writeln!(writer, "// Constants for matching (Headers).\n")?;

            for different_header_str in different_str.iter() {
                writeln!(
                    writer,
                    "pub const {}: &[u8] = b\"{}\";",
                    different_header_str.to_case(Case::Constant),
                    different_header_str
                )?;
            }

            writeln!(writer, "\n// Constants for matching (Headers).\n")?;

            for different_header_str in different_str.iter() {
                writeln!(
                    writer,
                    "            {} => self.{} = value,",
                    different_header_str.to_case(Case::Constant),
                    different_header_str.to_case(Case::Snake)
                )?;
            }

            writeln!(writer, "\n// Struct fields (Headers).\n")?;

            for different_header_str in different_str.iter() {
                writeln!(
                    writer,
                    "    {}: String,",
                    different_header_str.to_case(Case::Snake)
                )?;
            }

            writeln!(writer, "\n// Struct fields reset (Headers).\n")?;

            for different_header_str in different_str.iter() {
                writeln!(
                    writer,
                    "        self.{}.clear();",
                    different_header_str.to_case(Case::Snake)
                )?;
            }
        }

        Ok(())
    }

    /// Writes the comments and their generated code to a writer.
    pub fn write_comments<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut different = self.comments.iter().collect::<Vec<&Vec<u8>>>();
        let mut different_str = Vec::new();
        different.sort();
        for different_comment in different.iter() {
            match from_utf8(different_comment) {
                Ok(str) => different_str.push(Cow::Borrowed(str)),
                Err(_) => {
                    let str = String::from_utf8_lossy(different_comment);
                    error!("Invalid UTF-8 comment: {} <- {:?}", str, different_comment);
                    different_str.push(str);
                }
            }
        }

        writeln!(writer, "// Comments collection.\n")?;

        #[cfg(not(feature = "full-collect"))]
        {
            for different_comment_str in different_str {
                writeln!(writer, "{}", skip_percentage(&different_comment_str))?;
            }
        }
        #[cfg(feature = "full-collect")]
        {
            writeln!(writer, "// Constants for matching (Comments).\n")?;

            for (different_comment, different_comment_str) in
                different.iter().zip(different_str.iter())
            {
                writeln!(
                    writer,
                    "pub const {}: &[u8] = b\"{}\";",
                    skip_percentage(different_comment_str).to_case(Case::Constant),
                    different_comment_str
                )?;
            }

            writeln!(writer, "\n// Constants for matching (Comments).\n")?;

            for different_comment_str in different_str.iter() {
                writeln!(
                    writer,
                    "            {} => self.{} = value,",
                    skip_percentage(different_comment_str).to_case(Case::Constant),
                    skip_percentage(different_comment_str).to_case(Case::Snake)
                )?;
            }

            writeln!(writer, "\n// Struct fields (Comments).\n")?;

            for different_comment_str in different_str.iter() {
                writeln!(
                    writer,
                    "    {}: String,",
                    skip_percentage(different_comment_str).to_case(Case::Snake)
                )?;
            }

            writeln!(writer, "\n// Struct fields reset (Comments).\n")?;

            for different_comment_str in different_str.iter() {
                writeln!(
                    writer,
                    "        self.{}.clear();",
                    skip_percentage(different_comment_str).to_case(Case::Snake)
                )?;
            }
        }

        Ok(())
    }
}
