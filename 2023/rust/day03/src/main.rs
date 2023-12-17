use std::{ops::RangeInclusive, str::from_utf8};

fn get_number_index(line: &[u8], index: usize) -> Option<(RangeInclusive<usize>, u32)> {
    if let Some(c) = line.get(index) {
        if !c.is_ascii_digit() {
            return None;
        }
    } else {
        return None;
    }
    let mut start = index;
    let mut end = index;
    let string_size = line.len();

    if index > usize::MIN {
        start -= 1;
    }
    if index < string_size {
        end += 1;
    }

    while let Some(c) = line.get(start) {
        if c.is_ascii_digit() {
            if start == usize::MIN {
                break;
            }
            start -= 1;
        } else {
            start += 1;
            break;
        }
    }
    while let Some(c) = line.get(end) {
        if c.is_ascii_digit() {
            if end == string_size {
                break;
            }
            end += 1;
        } else {
            break;
        }
    }
    end -= 1;
    let num = from_utf8(&line[start..=end]).ok()?.parse().ok()?;

    Some((start..=end, num))
}

// #,$,%,&,*,+,-,/,=
fn is_symbol(c: &char) -> bool {
    matches!(c, '#' | '$' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@')
}
fn is_gear(c: &char) -> bool {
    matches!(c, '*')
}

type DirectionChange = fn(usize, usize) -> Option<(usize, usize)>;
const DIRECTIONS: [DirectionChange; 8] = [
    |x, y| Some((x.checked_sub(1)?, y.checked_add(1)?)), // (-1,1)
    |x, y| Some((x, y.checked_add(1)?)),                 //(0, 1)
    |x, y| Some((x.checked_add(1)?, y.checked_add(1)?)), //(1, 1)
    |x, y| Some((x.checked_add(1)?, y)),                 //(1, 0)
    |x, y| Some((x.checked_add(1)?, y.checked_sub(1)?)), //(1, -1)
    |x, y| Some((x, y.checked_sub(1)?)),                 //(0, -1)
    |x, y| Some((x.checked_sub(1)?, y.checked_sub(1)?)), //(-1, -1)
    |x, y| Some((x.checked_sub(1)?, y)),                 //(-1, 0)
];

fn main() {
    let mut input_lines: Vec<_> = aoc_utils::load_input_file("input.txt").collect();
    let empty_line = ".".repeat(input_lines[0].len());
    input_lines.insert(0, empty_line.clone());
    input_lines.push(empty_line);

    const WORKING_LINE: usize = 1;
    let mut nums_found: Vec<(usize, RangeInclusive<usize>, u32)> = Vec::new();
    let mut gears_found: Vec<u32> = Vec::new();
    let mut partial_gear: Vec<(RangeInclusive<usize>, u32)> = Vec::new();
    let mut partial_found: Vec<(usize, RangeInclusive<usize>, u32)> = Vec::new();

    for lines in input_lines.windows(3) {
        partial_found.clear();
        for (x, c) in lines[WORKING_LINE].bytes().enumerate() {
            if is_symbol(&(c as char)) {
                partial_gear.clear();
                'direction: for dir in &DIRECTIONS {
                    if let Some((x, y)) = dir(x, WORKING_LINE) {
                        for found in partial_found.iter() {
                            if found.0 == y && found.1.contains(&x) {
                                continue 'direction;
                            }
                        }
                        if let Some((range, num)) = get_number_index(lines[y].as_bytes(), x) {
                            partial_found.push((y, range.clone(), num));
                            if is_gear(&(c as char)) {
                                partial_gear.push((range, num));
                            }
                        }
                    }
                }
                if partial_gear.len() == 2 {
                    gears_found.push(partial_gear[0].1 * partial_gear[1].1);
                }
            }
        }
        nums_found.append(&mut partial_found);
    }

    let sum: u32 = nums_found.iter().map(|x| x.2).sum();

    println!("part1= {}", sum);
    println!("part1= {}", gears_found.iter().sum::<u32>());
}

#[cfg(test)]
mod test {
    use super::get_number_index;
    use rstest::rstest;
    use std::ops::RangeInclusive;

    #[rstest]
    #[case("467..114..722",[0,2,5,9,10,12],[Some((0..=2,467)),Some((0..=2,467)),Some((5..=7,114)),None,Some((10..=12,722)),Some((10..=12,722))])]
    fn test_get_number_index(
        #[case] input: &str,
        #[case] indexes: [usize; 6],
        #[case] results: [Option<(RangeInclusive<usize>, u32)>; 6],
    ) {
        for (indx, result) in indexes.iter().zip(results.iter()) {
            assert_eq!(get_number_index(input.as_bytes(), *indx), *result)
        }
    }
}
