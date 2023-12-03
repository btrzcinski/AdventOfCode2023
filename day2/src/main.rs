use std::{path::Path, str::FromStr};

mod cube_set;

use crate::cube_set::{CubeSet, Game};
use advent_tools::*;
use dialoguer::{Input, Select};

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

fn possible_games(path: &Path) {
    let target_bag = ask_for_target_bag();
    let games = report_runtime(|| {
        let games = read_games_from_file(path);
        sum_of_possible_games(&target_bag, &games)
    });
    println!("Sum of possible game IDs: {:#?}", games);
}

fn power_sum_of_minimum_possible_games(path: &Path) {
    let power_sum = report_runtime(|| {
        let games = read_games_from_file(path);
        games
            .iter()
            .map(Game::minimum_set)
            .map(CubeSet::power)
            .sum::<u32>()
    });
    println!("Sum of powers of minimum possible cubes: {}", power_sum);
}

fn main() {
    let file_path = pick_data_file();
    match Select::new()
        .with_prompt("Choose an exercise")
        .item("Part 1: Sum of possible game IDs")
        .item("Part 2: Sum of powers of minimum possible bags")
        .interact_opt()
        .unwrap()
    {
        Some(0) => possible_games(file_path.as_path()),
        Some(1) => power_sum_of_minimum_possible_games(file_path.as_path()),
        Some(_) => panic!("Out-of-bounds exercise choice"),
        None => println!("You did not choose anything."),
    }
}
