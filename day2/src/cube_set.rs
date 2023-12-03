use std::{
    cmp::max,
    iter::Sum,
    ops::{Add, Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CubeSet {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl CubeSet {
    pub fn superset_of(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn power(self) -> u32 {
        u32::from(self.red) * u32::from(self.green) * u32::from(self.blue)
    }
}

impl Add<&CubeSet> for CubeSet {
    type Output = CubeSet;

    fn add(self, other: &CubeSet) -> CubeSet {
        CubeSet {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl<'a> Sum<&'a CubeSet> for CubeSet {
    fn sum<I: Iterator<Item = &'a CubeSet>>(iter: I) -> Self {
        iter.fold(CubeSet::default(), CubeSet::add)
    }
}

#[derive(Debug)]
pub struct ParseCubeSetError;

impl FromStr for CubeSet {
    type Err = ParseCubeSetError;

    // Expects a string like "5 blue, 4 red, 13 green".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = CubeSet::default();
        for p in s.split(", ") {
            let (num_str, color) = p.split_once(" ").ok_or(ParseCubeSetError)?;
            let num = u8::from_str(num_str).or(Err(ParseCubeSetError))?;
            match color {
                "red" => out.red = num,
                "green" => out.green = num,
                "blue" => out.blue = num,
                _ => return Err(ParseCubeSetError),
            }
        }
        Ok(out)
    }
}

#[derive(Debug, Default)]
pub struct Game {
    pub number: u32,
    pub cube_sets: Vec<CubeSet>,
}

impl Deref for Game {
    type Target = Vec<CubeSet>;

    fn deref(&self) -> &Vec<CubeSet> {
        &self.cube_sets
    }
}

impl DerefMut for Game {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cube_sets
    }
}

#[derive(Debug)]
pub struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    // Expects a string like "Game 5: 1 blue, 2 red; 3 green, 4 blue".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, cube_sets) = s.split_once(": ").ok_or(ParseGameError)?;
        let game_number = game_id
            .strip_prefix("Game ")
            .map(u32::from_str)
            .and_then(Result::ok)
            .ok_or(ParseGameError)?;
        let mut out = Game {
            number: game_number,
            ..Default::default()
        };

        for set_str in cube_sets.split("; ") {
            let set = CubeSet::from_str(set_str).or(Err(ParseGameError))?;
            out.push(set)
        }

        Ok(out)
    }
}

impl Game {
    pub fn minimum_set(&self) -> CubeSet {
        self.cube_sets
            .iter()
            .fold(CubeSet::default(), |acc, set| CubeSet {
                red: max(acc.red, set.red),
                green: max(acc.green, set.green),
                blue: max(acc.blue, set.blue),
            })
    }
}
