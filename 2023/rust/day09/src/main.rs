fn main() {
    let input_lines = aoc_utils::load_input_file("input.txt");
    let (part2, part1): (i64, i64) = input_lines
        .map(|line| {
            line.split_whitespace()
                .flat_map(str::parse::<i64>)
                .collect::<Vec<_>>()
        })
        .map(|mut nums| {
            let last = *nums.last().unwrap();
            let first = *nums.first().unwrap();
            let mut is_odd_depth = false;
            let mut accumulator_last = 0;
            let mut accumulator_first = 0;
            loop {
                for x in 0..nums.len() - 1 {
                    nums[x] = nums[x + 1] - nums[x];
                }
                nums.pop();
                accumulator_last += nums.last().unwrap();
                accumulator_first += match is_odd_depth {
                    true => -nums.first().unwrap(),
                    false => *nums.first().unwrap(),
                };
                is_odd_depth = !is_odd_depth;

                if nums.windows(2).all(|couple| couple[1] == couple[0]) {
                    break;
                }
            }
            (first - accumulator_first, last + accumulator_last)
        })
        .reduce(|(acc_first, acc_last), (first, last)| (acc_first + first, acc_last + last))
        .unwrap();
    println!("part1: {part1}");
    println!("part2: {part2}");
}
