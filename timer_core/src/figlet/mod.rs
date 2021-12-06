use std::fs::read_to_string;
use std::io;
use std::path::Path;

const CHAR_OFFSET: usize = 32;

#[derive(Debug, Clone)]
pub struct Figlet {
    height: usize,
    chars: Vec<Char>,
}

#[derive(Debug, Clone)]
pub struct Char {
    text: Vec<String>,
}

impl Figlet {
    pub fn convert(&self, s: &str) -> String {
        let mut output: Vec<String> = (0..self.height).map(|_| String::new()).collect();
        let chars: Vec<_> = s
            .chars()
            .filter_map(|c| self.chars.get(c as usize - CHAR_OFFSET))
            .collect();

        for (i, row) in output.iter_mut().enumerate() {
            let line: String = chars
                .iter()
                .filter_map(|c| c.text.get(i).map(|s| s.as_str()))
                .collect();

            row.push_str(&line);
        }
        output.join("\n")
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let contents = read_to_string(path)?;
        parse(contents.lines()).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "file exists but appears to be invalid",
            )
        })
    }

    fn from_default() -> Self {
        let contents = include_str!("univers.flf");
        parse(contents.lines()).expect("embedded font should be parsable")
    }

    /// Returns the height of the largest character in the font.
    /// This operation is very fast.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for Figlet {
    fn default() -> Self {
        Figlet::from_default()
    }
}

/// Takes an iterator over lines of a .flf (figlet) file, and attempts to parse
/// it into a Font, which can be used for rendering.
pub fn parse<'a>(mut iter: impl Iterator<Item = &'a str>) -> Option<Figlet> {
    let header: Vec<_> = iter.next()?.split(' ').collect();

    let height: usize = header.get(1)?.parse().ok()?;
    let hard_blank = header.get(0)?.chars().last()?;
    let comments: usize = header.get(5)?.parse().ok()?;

    let mut chars: Vec<Char> = Vec::new();
    let mut current: Vec<String> = Vec::with_capacity(height);

    for line in iter.skip(comments) {
        let mut len = line.len();
        for c in line.as_bytes().iter().rev() {
            if *c == b'@' {
                len -= 1;
            } else {
                break;
            }
        }

        if len == line.len() {
            // NOTE: we only care about the positional characters for now
            // so stop when we reach e.g. "160  NO-BREAK SPACE"
            break;
        }
        let slice = &line[0..len];
        current.push(slice.replace(hard_blank, " "));

        if current.len() == height {
            let width = current.iter().fold(0, |max, s| std::cmp::max(max, s.len()));

            if current.len() == height {
                for line in current.iter_mut() {
                    while line.len() < width {
                        line.push(' ');
                    }
                }

                chars.push(Char {
                    text: std::mem::replace(&mut current, Vec::with_capacity(height)),
                })
            }
        }
    }

    Some(Figlet { height, chars })
}
