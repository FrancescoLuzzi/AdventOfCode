use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
struct XMapY {
    in_min: u64,
    in_max: u64,
    out_min: u64,
}

impl XMapY {
    fn new(in_min: u64, out_min: u64, offset: u64) -> Self {
        Self {
            in_min,
            out_min,
            in_max: in_min + offset - 1,
        }
    }

    fn map(&self, to_map: u64) -> Option<u64> {
        if self.in_min > to_map || to_map > self.in_max {
            return None;
        }
        Some(self.out_min + (to_map - self.in_min))
    }
}

impl From<&str> for XMapY {
    fn from(value: &str) -> Self {
        let mut parsed = value.split_whitespace().flat_map(str::parse::<u64>);
        let out_min = parsed.next().unwrap();
        let in_min = parsed.next().unwrap();
        let offset = parsed.next().unwrap();
        Self::new(in_min, out_min, offset)
    }
}

#[derive(Debug)]
struct Mapper {
    x_mappers: Vec<XMapY>,
    next_mapper: Option<Box<Mapper>>,
}

impl Mapper {
    fn new(x_mappers: Vec<XMapY>) -> Self {
        Self {
            x_mappers,
            next_mapper: None,
        }
    }
    fn set_next_mapper(&mut self, next_mapper: Mapper) {
        if let Some(ref mut next) = self.next_mapper {
            next.set_next_mapper(next_mapper);
        } else {
            self.next_mapper = Some(Box::new(next_mapper));
        }
    }

    fn map(&self, to_map: u64) -> u64 {
        let out = self.x_mappers.iter().find_map(|mapper| mapper.map(to_map));
        let out = if let Some(out) = out { out } else { to_map };
        if let Some(ref next_map) = self.next_mapper {
            next_map.map(out)
        } else {
            out
        }
    }
}

fn main() {
    let mut input_lines = aoc_utils::load_input_file("input.txt");
    let seeds_part1: Vec<u64> = input_lines
        .next()
        .map(|x| {
            x.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .unwrap();
    let mut mappers: Vec<Mapper> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let mut tmp: Option<String> = input_lines.next();
    while let Some(line) = tmp {
        if line.is_empty() && !lines.is_empty() {
            mappers.push(Mapper::new(
                lines.iter().map(|x| XMapY::from(x.as_str())).collect(),
            ));
        } else if line.ends_with(':') {
            lines.clear();
        } else {
            lines.push(line);
        }
        tmp = input_lines.next();
    }
    if !lines.is_empty() {
        mappers.push(Mapper::new(
            lines.iter().map(|x| XMapY::from(x.as_str())).collect(),
        ));
    }
    let h2l = mappers.pop().unwrap();
    let mut t2h = mappers.pop().unwrap();
    let mut l2t = mappers.pop().unwrap();
    let mut w2l = mappers.pop().unwrap();
    let mut f2w = mappers.pop().unwrap();
    let mut s2f = mappers.pop().unwrap();
    let mut s2s = mappers.pop().unwrap();
    t2h.set_next_mapper(h2l);
    l2t.set_next_mapper(t2h);
    w2l.set_next_mapper(l2t);
    f2w.set_next_mapper(w2l);
    s2f.set_next_mapper(f2w);
    s2s.set_next_mapper(s2f);
    let s2s = Arc::new(s2s);
    let out1 = seeds_part1.iter().map(|x| s2s.map(*x)).min().unwrap();

    // get intervals
    let p2_intervals: Vec<(u64, u64)> = seeds_part1
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1] - 1))
        .collect();
    // brute force with multithreading
    // the smart way would be to map the seeds intervals [start,end]
    // into the output intervals [[out_start1,out_end1],...] depending on the XMapY's in the Mapper
    // and then passing each intermidiate output range to the next_mapper (if present)
    // then take the start number the range with smallest start number
    // can't be bothered to implement it
    let out2 = p2_intervals
        .into_iter()
        .map(|(seed_start, seed_end)| {
            (seed_start..=seed_end)
                .into_par_iter()
                .map(|seed| s2s.map(seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    println!("part1 = {out1}");
    println!("part2 = {out2}");
}
