const GALAXY: char = '#';

#[cfg(feature = "part1")]
const EXPAND_MULTIPLIER: usize = 2;
#[cfg(feature = "part2")]
const EXPAND_MULTIPLIER: usize = 1_000_000;
const CALCULATION_MULTIPLIER: usize = EXPAND_MULTIPLIER - 1;

fn main() {
    let input_lines: Vec<_> = aoc_utils::load_input_file("input.txt").collect();
    let galaxy_width = input_lines[0].len();
    let galaxy_height = input_lines.len();
    let mut occupied_columns: Vec<_> = vec![0; galaxy_width];
    let mut occupied_rows: Vec<_> = vec![0; galaxy_height];
    let mut galaxies: Vec<(usize, usize)> = Vec::with_capacity(galaxy_height);
    for (y, line) in input_lines.iter().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| c == &GALAXY) {
            occupied_columns[x] = 1;
            occupied_rows[y] = 1;
            galaxies.push((x, y));
        }
    }
    let expanded_columns: Vec<usize> = occupied_columns
        .into_iter()
        .enumerate()
        .filter(|(_, val)| *val == 0)
        .map(|(h, _)| h)
        .collect();
    let expanded_rows: Vec<usize> = occupied_rows
        .into_iter()
        .enumerate()
        .filter(|(_, val)| *val == 0)
        .map(|(w, _)| w)
        .collect();
    let mut total_expansion: u64 = 0;
    for i in 0..galaxies.len() - 1 {
        let start_galaxy = galaxies[i];
        for dest_galaxy in galaxies.iter().skip(i + 1) {
            let height_expand: usize = expanded_columns
                .iter()
                .filter(|&&x| {
                    if x > start_galaxy.0 {
                        x < dest_galaxy.0
                    } else {
                        x > dest_galaxy.0
                    }
                })
                .fold(0, |i, _| i + 1);
            let width_expand = expanded_rows
                .iter()
                .filter(|&&y| {
                    if y > start_galaxy.1 {
                        y < dest_galaxy.1
                    } else {
                        y > dest_galaxy.1
                    }
                })
                .fold(0, |i, _| i + 1);
            total_expansion += (start_galaxy.0.abs_diff(dest_galaxy.0)
                + start_galaxy.1.abs_diff(dest_galaxy.1)
                + (width_expand + height_expand) * CALCULATION_MULTIPLIER)
                as u64;
        }
    }
    println!("part1 = {}", total_expansion);
}
