// https://adventofcode.com/2023/day/2

use std::ops::Add;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct GameConfig {
    red_balls: u32,
    blue_balls: u32,
    green_balls: u32,
}
impl GameConfig {
    fn is_game_valid(&self, game: &Game) -> bool {
        game.reveals
            .iter()
            .map(|reveal| {
                reveal.red_balls.unwrap_or(0) <= self.red_balls
                    && reveal.green_balls.unwrap_or(0) <= self.green_balls
                    && reveal.blue_balls.unwrap_or(0) <= self.blue_balls
            })
            .all(|x| x)
    }

    fn get_power(&self) -> u64 {
        self.red_balls as u64 * self.blue_balls as u64 * self.green_balls as u64
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct GameReveal {
    red_balls: Option<u32>,
    blue_balls: Option<u32>,
    green_balls: Option<u32>,
}

impl TryFrom<&str> for GameReveal {
    type Error = String;
    /// format, note that it could be any order
    /// 3 blue, 7 green, 10 red
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game_reveal = GameReveal::default();
        let errs: Vec<_> = value
            .split(',')
            .map(
                |rev| match rev.split_whitespace().map(str::trim).collect::<Vec<&str>>()[..] {
                    [num, "red"] => {
                        game_reveal.red_balls = num
                            .parse::<u32>()
                            .map_err(|_| "couldn't parse number of red balls".to_owned())?
                            .into();
                        Ok(())
                    }
                    [num, "blue"] => {
                        game_reveal.blue_balls = num
                            .parse::<u32>()
                            .map_err(|_| "couldn't parse number of blue balls".to_owned())?
                            .into();
                        Ok(())
                    }

                    [num, "green"] => {
                        game_reveal.green_balls = num
                            .parse::<u32>()
                            .map_err(|_| "couldn't parse number of green balls".to_owned())?
                            .into();
                        Ok(())
                    }

                    _ => Err(format!("GameReveal not known: {}", rev)),
                },
            )
            .filter(|x| x.is_err())
            .collect();
        if !errs.is_empty() {
            let mut tmp_err = "".to_owned();
            for err in errs {
                tmp_err += &err.err().unwrap().add("\n");
            }
            return Err(tmp_err);
        }
        Ok(game_reveal)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    reveals: Vec<GameReveal>,
}

impl Game {
    fn get_min_config(&self) -> GameConfig {
        let mut out_config = GameConfig {
            red_balls: 0,
            blue_balls: 0,
            green_balls: 0,
        };

        for rev in self.reveals.iter() {
            let red_balls = rev.red_balls.unwrap_or(0);
            let green_balls = rev.green_balls.unwrap_or(0);
            let blue_balls = rev.blue_balls.unwrap_or(0);
            if red_balls > out_config.red_balls {
                out_config.red_balls = red_balls
            }
            if blue_balls > out_config.blue_balls {
                out_config.blue_balls = blue_balls
            }
            if green_balls > out_config.green_balls {
                out_config.green_balls = green_balls
            }
        }
        out_config
    }
}

impl TryFrom<&str> for Game {
    type Error = String;

    /// game format:
    /// Game $game_id: 3 blue, 7 green, 10 red; games...;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':');
        let game = parts
            .next()
            .ok_or("game format not compliant".to_owned())?
            .trim();
        let reveals = parts
            .next()
            .ok_or("game format not compliant".to_owned())?
            .trim();
        let game_id = game
            .split_whitespace()
            .last()
            .ok_or("game id not found".to_owned())?
            .parse::<u32>()
            .map_err(|_| "couldn't parse game id ".to_owned())?;
        let (reveals, errs): (Vec<_>, Vec<_>) = reveals
            .split(';')
            .map(str::trim)
            .map(GameReveal::try_from)
            .partition(Result::is_ok);
        if !errs.is_empty() {
            let mut tmp_err = "".to_owned();
            for err in errs {
                tmp_err += &err.err().unwrap().add("\n");
            }
            return Err(tmp_err);
        }
        let reveals: Vec<_> = reveals.into_iter().map(Result::unwrap).collect();

        Ok(Self {
            id: game_id,
            reveals,
        })
    }
}

fn main() {
    let input_lines = aoc_utils::load_input_file("input.txt");
    let game_config = GameConfig {
        red_balls: 12,
        blue_balls: 14,
        green_balls: 13,
    };
    let games: Vec<Game> = input_lines
        .flat_map(|line| Game::try_from(line.as_str()))
        .collect();
    let games_ids_sum: u64 = games
        .iter()
        .filter(|game| game_config.is_game_valid(game))
        .map(|game| game.id as u64)
        .sum();

    let total_power_games: u64 = games
        .iter()
        .map(|game| game.get_min_config().get_power())
        .sum();

    println!("part1: {games_ids_sum}");
    println!("part2: {total_power_games}");
}

#[cfg(test)]
mod tests {
    use crate::GameConfig;

    use super::{Game, GameReveal};
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 7 green, 10 red; 4 green, 4 red\n\n", Ok(Game{id:1,reveals:[GameReveal{blue_balls:Some(3),green_balls:Some(7),red_balls:Some(10)},GameReveal{blue_balls:None,green_balls:Some(4),red_balls:Some(4)}].to_vec()}))]
    fn test_parse_game(#[case] input: &str, #[case] expected: Result<Game, String>) {
        assert_eq!(Game::try_from(input), expected);
    }

    #[rstest]
    #[case("3 blue, 7 green, 10 red ", Ok(GameReveal{blue_balls:Some(3),green_balls:Some(7),red_balls:Some(10)}))]
    #[case("4 green, 4 red", Ok(GameReveal{blue_balls:None,green_balls:Some(4),red_balls:Some(4)}))]
    fn test_parse_game_reveal(#[case] input: &str, #[case] expected: Result<GameReveal, String>) {
        assert_eq!(GameReveal::try_from(input), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 7 green, 10 red; 4 green, 4 red\n\n", GameConfig{ red_balls: 10, blue_balls: 3, green_balls: 7 })]
    fn test_get_min_config(#[case] input: &str, #[case] expected: GameConfig) {
        assert_eq!(Game::try_from(input).unwrap().get_min_config(), expected);
    }
    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_get_min_config_power(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(
            Game::try_from(input).unwrap().get_min_config().get_power(),
            expected
        );
    }
}
