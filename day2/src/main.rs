use std::{
    iter::Sum,
    ops::{Add, Deref, DerefMut},
    path::Path,
    str::FromStr,
};

use advent_tools::*;
use dialoguer::Input;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSet {
    fn superset_of(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
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
struct ParseCubeSetError;

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
struct Game {
    number: u32,
    cube_sets: Vec<CubeSet>,
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
struct ParseGameError;

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

fn read_games_from_file(path: &Path) -> Vec<Game> {
    read_input_file(path)
        .lines()
        .map(Game::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn sum_of_possible_games<'a, T>(bag: &CubeSet, games: T) -> u32
where
    T: IntoIterator<Item = &'a Game>,
{
    games
        .into_iter()
        .filter_map(|Game { number, cube_sets }| {
            match cube_sets.into_iter().all(|set| bag.superset_of(set)) {
                false => None,
                true => Some(number),
            }
        })
        .sum()
}

fn ask_for_target_bag() -> CubeSet {
    println!("Enter the number of cubes in the target bag to check.");
    let red = Input::new().with_prompt("Red").interact_text().unwrap();
    let green = Input::new().with_prompt("Green").interact_text().unwrap();
    let blue = Input::new().with_prompt("Blue").interact_text().unwrap();
    CubeSet { red, green, blue }
}

fn main() {
    let file_path = pick_data_file();
    let target_bag = ask_for_target_bag();
    let games = report_runtime(|| {
        let games = read_games_from_file(file_path.as_path());
        sum_of_possible_games(&target_bag, &games)
    });
    println!("Games: {:#?}", games);
}
