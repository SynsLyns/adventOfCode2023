use std::fs;
use std::env;
use std::time::Instant;
use std::cmp::max;
use std::cmp::min;
use std::vec;

#[derive(Debug)]
struct AlmanacEntry {
    source: u64,
    dest: u64,
    range: u64,
}

#[derive(Debug)]
struct AlmanacMap {
    map: Vec<AlmanacEntry>
}

#[derive(Debug)]
struct Almanac {
    maps: Vec<AlmanacMap>
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64
}

impl AlmanacEntry {
    fn convert(&self, s: u64) -> Option<u64> {
        if s >= self.source && s <= self.source + self.range {
            return Some(self.dest + (s - self.source));
        }
        None
    }

    fn convert_reverse(&self, s: u64) -> Option<u64> {
        if s >= self.dest && s <= self.dest + self.range {
            return Some(self.source + (s - self.dest));
        }
        None
    }

    fn convert_ranges(&self, s: &Vec<Range>) -> Option<Vec<Range>> {
        let mut result: Vec<Range> = vec![];
        let entry_range = Range::new(self.source, self.source + self.range);
        for range in s {
            result.append(&mut range.transform(&entry_range, self.dest));
        }
        if result.len() == 0 {
            return None
        }
        return Some(result);
    }
}

impl AlmanacMap {
    fn convert(&self, s: u64) -> u64 {
        let mut result: Option<u64> = Option::None;
        for entry in &self.map {
            match entry.convert(s) {
                Some(dest) => {
                    result = Some(dest);
                    break;
                },
                None => {}
            }
        }
        match result {
            Some(dest) => dest,
            None => s
        }
    }

    fn convert_reverse(&self, s: u64) -> u64 {
        let mut result: Option<u64> = Option::None;
        for entry in &self.map {
            match entry.convert_reverse(s) {
                Some(dest) => {
                    result = Some(dest);
                    break;
                },
                None => {}
            }
        }
        match result {
            Some(dest) => dest,
            None => s
        }
    }

    fn convert_ranges(&self, s: &Vec<Range>) -> Option<Vec<Range>> {
        let mut result: Vec<Range> = vec![];
        for entry in &self.map {
            match entry.convert_ranges(s) {
                Some(mut r) => {
                    result.append(&mut r);
                    result = compress_ranges(result);
                }
                None => {}
            }
        }
        if result.len() == 0 {
            return None
        }
        return Some(result);
    }
}

impl Almanac {
    fn convert(&self, s: u64) -> u64 {
        let mut result = s;
        for map in &self.maps {
            result = map.convert(result);
        }
        result
    }

    fn convert_reverse(&self, s: u64) -> u64 {
        let mut result = s;
        for map in self.maps.iter().rev() {
            result = map.convert_reverse(result);
        }
        result
    }

    fn convert_ranges(&self, s: Vec<Range>) -> Vec<Range> {
        let mut result = s;
        for map in &self.maps {
            match map.convert_ranges(&result) {
                Some(r) => result = r,
                None => {}
            }
        }
        result
    }
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range {
            start,
            end
        }
    }

    fn transform(&self, r2: &Self, add: u64) -> Vec<Self> {
        let start1 = self.start;
        let end1 = self.end;
        let start2 = r2.start;
        let end2 = r2.end;
        let mut result: Vec<Range> = vec![];

        if end1 < start2 || end2 < start1 {
            result.push(Range::new(start1, start1));
            return result
        }
        
        let overlap_start = max(start1, start2);
        let overlap_end = min(end1, end2);

        result.push(Range::new(overlap_start + add, overlap_end + add));
        if start1 < start2 {
            result.push(Range::new(start1, start2 - 1));
        }
        if end1 > end2 {
            result.push(Range::new(end2 + 1, end1));
        }

        return result;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let maps: Vec<&str> = contents.split("\n\r").map(|x| x.trim()).skip(1).collect();
    let mut almanac_maps: Vec<AlmanacMap> = vec![];
    for map in &maps {
        let mut almanac_map: Vec<AlmanacEntry> = vec![];
        let entries = map.split("\r\n").skip(1);
        for entry in entries {
            let mut entry_iter = entry.split_whitespace();
            almanac_map.push(
                AlmanacEntry {
                    dest: entry_iter.next().unwrap().parse().unwrap(),
                    source: entry_iter.next().unwrap().parse().unwrap(),
                    range: entry_iter.next().unwrap().parse().unwrap()
                }
            );
        }
        almanac_maps.push(AlmanacMap { map: almanac_map });
    }
    let almanac = Almanac {
        maps: almanac_maps
    };

    let seeds: Vec<u64> = contents.lines().next().unwrap().split(":").nth(1).unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let mut min = std::u64::MAX;
    for seed in &seeds {
        let location = almanac.convert(*seed);
        if location < min {
            min = location;
        }
    }

    let mut seeds_iter = seeds.iter();

    let mut seed_map:Vec<AlmanacEntry> = vec![];
    let mut seed_ranges: Vec<Range> = vec![];
    while seeds_iter.len() != 0 {
        let source = *seeds_iter.next().unwrap();
        let range = *seeds_iter.next().unwrap();
        seed_ranges.push(Range::new(source, source + range));
        seed_map.push(
            AlmanacEntry {
                source,
                dest: 1,
                range
            }
        ); 
    }

    let now = Instant::now();
    let mut min2 = std::u64::MAX;
    for i in 0..std::u64::MAX {
        let s = almanac.convert_reverse(i);
        let mut result: Option<u64> = Option::None;
        for entry in &seed_map {
            match entry.convert(s) {
                Some(dest) => {
                    result = Some(dest);
                    break;
                },
                None => {}
            }
        }
        match result {
            Some(_) => {
                min2 = i;
                break;
            },
            None => {}
        }
    }

    let location_ranges = almanac.convert_ranges(seed_ranges);
    let mut mins: Vec<u64> = vec![];
    let mut mins3 = std::u64::MAX;
    for range in location_ranges {
        mins.push(range.start);
    }

    for i in mins {
        if i < mins3 {
            mins3 = i;
        }
    }


    println!("Part 1: {min}, Part 2: {min2} {mins3}");
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);
}

fn compress_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    // Ensure the input is sorted based on the start of each range
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();
    result.push(ranges[0]);

    for i in 1..ranges.len() {
        let current_range = ranges[i];
        let last_merged_range = result.last_mut().unwrap();

        // Check for overlap or adjacency
        if current_range.start <= last_merged_range.end {
            // Merge the ranges
            last_merged_range.end = last_merged_range.end.max(current_range.end);
        } else {
            // Add the current range to the result
            result.push(current_range);
        }
    }

    result
}