use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
    sync::{Arc, Mutex},
};

pub struct Papier {
    x: i64,
    y: i64,
    page: HashMap<(i64, i64), Vec<char>>,
}

impl Default for Papier {
    fn default() -> Self {
        Self::new()
    }
}

impl Papier {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            page: HashMap::new(),
        }
    }

    pub fn write(&mut self, character: char) {
        match character {
            '\n' => {
                self.y += 1;
            }
            '\r' => {
                self.x = 0;
            }
            _ => {
                self.page
                    .entry((self.x, self.y))
                    .or_default()
                    .push(character);
                self.x += 1
            }
        }
    }

    pub fn writes(&mut self, characters: &str) {
        for character in characters.chars() {
            self.write(character);
        }
    }

    pub fn read(&self, dx: i64, dy: i64) -> Option<char> {
        self.page
            .get(&(self.x + dx, self.y + dy))
            .and_then(|v| v.first())
            .copied()
    }

    pub fn reads(&self, dx: i64, dy: i64, width: u64) -> String {
        let mut result = String::new();
        for i in 0..width {
            result.push(self.read(dx + i as i64, dy).unwrap_or(' '));
        }
        result
    }

    pub fn aread(&self, x: i64, y: i64) -> Option<char> {
        self.page.get(&(x, y)).and_then(|v| v.first()).copied()
    }

    pub fn areads(&self, x: i64, y: i64, width: u64) -> String {
        let mut result = String::new();
        for i in 0..width {
            result.push(self.aread(x + i as i64, y).unwrap_or(' '));
        }
        result
    }

    pub fn print(&self) -> String {
        if self.page.is_empty() {
            return String::new();
        }

        let &min_x = self.page.keys().map(|(x, _)| x).min().unwrap();
        let &max_x = self.page.keys().map(|(x, _)| x).max().unwrap();
        let &min_y = self.page.keys().map(|(_, y)| y).min().unwrap();
        let &max_y = self.page.keys().map(|(_, y)| y).max().unwrap();

        let mut result = String::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = self.aread(x, y).unwrap_or(' ');
                result.push(ch);
            }
            result.push('\n');
        }

        result
    }
}

pub struct Numbers {}

impl Numbers {
    pub fn write(papier: &mut Papier, number: i64, width: usize) {
        let s = format!("{}", number);
        let mut correct_width = if s.len() >= width {
            s[..width].to_string()
        } else {
            format!("{:>width$}", s, width = width)
        };
        correct_width.push(' ');
        papier.writes(correct_width.as_str());
    }

    pub fn read<T: FromStr>(papier: &mut Papier, dx: i64, dy: i64, width: u64) -> Option<T> {
        let binding = papier.reads(dx, dy, width);
        let string = binding.trim();
        string.parse::<T>().ok()
    }
}
