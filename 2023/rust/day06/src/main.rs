enum Direction {
    Forward,
    Backward,
    Both,
    None,
}

struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn get_winnings(&self) -> usize {
        let (starting_point, direction) = self.get_winning_start_speed();
        match direction {
            Direction::Forward => self.wins_until_lose(starting_point, Direction::Forward),
            Direction::Backward => self.wins_until_lose(starting_point, Direction::Backward),
            Direction::Both => {
                self.wins_until_lose(starting_point, Direction::Backward)
                    + self.wins_until_lose(starting_point + 1, Direction::Forward)
            }
            Direction::None => 1,
        }
    }

    fn wins_until_lose(&self, starting_point: usize, direction: Direction) -> usize {
        let offsetting = match direction {
            Direction::Forward => |x| x + 1,
            Direction::Backward => |x| x - 1,
            _ => panic!("Impossible direction"),
        };
        let mut wins = 0;
        let mut curr_point = starting_point;
        while self.is_winnable(curr_point) {
            wins += 1;
            if curr_point == 0 || curr_point == self.time {
                break;
            }
            curr_point = offsetting(curr_point);
        }
        wins
    }
    fn get_winning_start_speed(&self) -> (usize, Direction) {
        //start by estimating half time as speed (rounded down)
        let speed = self.time / 2;
        let mut front_speed = speed + 1;
        let mut back_speed = speed - 1;
        if self.is_winnable(speed) {
            let is_front = self.is_winnable(front_speed);
            let is_back = self.is_winnable(back_speed);
            let direction = match (is_back, is_front) {
                (true, true) => Direction::Both,
                (true, false) => Direction::Backward,
                (false, true) => Direction::Forward,
                (false, false) => Direction::None,
            };
            return (speed, direction);
        }
        loop {
            if self.is_winnable(front_speed) {
                return (front_speed, Direction::Forward);
            } else if self.is_winnable(back_speed) {
                return (front_speed, Direction::Backward);
            }
            if back_speed == 0 && front_speed == self.time {
                panic!("This race is unwinnable")
            }
            back_speed = back_speed.saturating_sub(1);
            if front_speed < self.time {
                front_speed += 1;
            }
        }
    }
    fn is_winnable(&self, speed: usize) -> bool {
        let remaning_time = self.time - speed;
        speed * remaning_time > self.distance
    }
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split(':')
        .skip(1)
        .flat_map(|x| x.split_whitespace().flat_map(str::parse::<usize>))
        .collect()
}

fn main() {
    let mut input_lines = aoc_utils::load_input_file("input.txt");
    let times: Vec<usize> = parse_line(&input_lines.next().unwrap());
    let distances: Vec<usize> = parse_line(&input_lines.next().unwrap());
    let races_part1: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();
    let part1: usize = races_part1.iter().map(|x| x.get_winnings()).product();
    let time_part2: String = times.iter().map(|x| x.to_string()).collect();
    let distance_part2: String = distances.iter().map(|x| x.to_string()).collect();
    let race_part2 = Race {
        time: time_part2.parse().unwrap(),
        distance: distance_part2.parse().unwrap(),
    };
    println!("part1 = {part1}");
    println!("part2 = {}", race_part2.get_winnings());
}
