#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::result::Result;

/// https://adventofcode.com/2020/day/1
/// Runtime complexity: O(n.log(n))
/// Space complexity: O(1)
pub fn day_01_part1(array: &mut Vec<u32>, target: u32) -> Result<u32, NoSolution> {
    array.sort_unstable(); // Quicksort, given primitives.
    for &x in array.iter() {
        let y = target - x;
        if array.binary_search(&y).is_ok() {
            return Ok(x * y);
        }
    }
    Err(NoSolution::new(&format!(
        "no two numbers sum up to {}",
        target
    )))
}

/// https://adventofcode.com/2020/day/1#part2
/// Runtime complexity: O(n^2.log(n))
/// Space complexity: O(1)
/// N.B.: we could optimise the binary search by providing from & to indexes.
pub fn day_01_part2(array: &mut Vec<u32>, target: u32) -> Result<u32, NoSolution> {
    array.sort_unstable(); // Quicksort, given primitives.
    for i in 0..array.len() - 1 {
        for j in i + 1..array.len() {
            let sum = array[i] + array[j];
            if let Ok(k) = array.binary_search(&(target - sum)) {
                return Ok(array[i] * array[j] * array[k]);
            }
        }
    }
    Err(NoSolution::new(&format!(
        "no three numbers sum up to {}",
        target
    )))
}

/// https://adventofcode.com/2020/day/2
/// Runtime complexity: O(n)
/// Space complexity: O(1)
pub fn day_02_part1(array: &[(usize, usize, String, String)]) -> usize {
    array
        .iter()
        .filter(|(min, max, character, password)| {
            let count = &password.matches(character).count();
            min <= count && count <= max
        })
        .count()
}

/// https://adventofcode.com/2020/day/2#part2
/// Runtime complexity: O(n)
/// Space complexity: O(1)
pub fn day_02_part2(array: &[(usize, usize, String, String)]) -> usize {
    array
        .iter()
        .filter(|(i, j, character, password)| {
            let c1 = password.chars().nth(i - 1).unwrap(); // Indexes are 1-based and always valid.
            let c2 = password.chars().nth(j - 1).unwrap(); // Indexes are 1-based and always valid.
            let c = character.chars().next().unwrap(); // Always only 1 char.
            ((c1 == c) || (c2 == c)) && !((c1 == c) && (c2 == c))
        })
        .count()
}

/// https://adventofcode.com/2020/day/3
/// Runtime complexity: O(n*m)
/// Space complexity: O(1)
pub fn day_03_part1(grid: &[String]) -> usize {
    count_trees(grid, 3, 1)
}

fn count_trees(grid: &[String], step_right: usize, step_down: usize) -> usize {
    let (mut num_trees, mut row, mut col) = (0, 0, 0);
    while row < grid.len() {
        let cell = grid[row].chars().nth(col).unwrap(); // O(m), s.t. m is number of columns.
        if cell == '#' {
            num_trees += 1;
        }
        col = (col + step_right) % grid[row].len();
        row += step_down;
    }
    num_trees
}

/// https://adventofcode.com/2020/day/3#part2
/// Runtime complexity: O(n*m)
/// Space complexity: O(1)
pub fn day_03_part2(grid: &[String]) -> usize {
    count_trees(grid, 1, 1)
        * count_trees(grid, 3, 1)
        * count_trees(grid, 5, 1)
        * count_trees(grid, 7, 1)
        * count_trees(grid, 1, 2)
}

/// https://adventofcode.com/2020/day/4
/// Runtime complexity: O(n)
/// Space complexity: O(1)
pub fn day_04_part1(array: &[HashMap<String, String>]) -> usize {
    let mandatory_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    array
        .iter()
        .filter(|map| {
            mandatory_fields
                .iter()
                .all(|&field| map.contains_key(field))
        })
        .count()
}

/// https://adventofcode.com/2020/day/4#part2
/// Runtime complexity: O(n)
/// Space complexity: O(1)
pub fn day_04_part2(array: &[HashMap<String, String>]) -> usize {
    array.iter().filter(|&map| is_valid_passport(map)).count()
}

fn is_valid_passport(map: &HashMap<String, String>) -> bool {
    if !contains_valid_byr(map) {
        return false;
    }
    if !contains_valid_iyr(map) {
        return false;
    }
    if !contains_valid_eyr(map) {
        return false;
    }
    if !contains_valid_hgt(map) {
        return false;
    }
    if !contains_valid_hcl(map) {
        return false;
    }
    if !contains_valid_ecl(map) {
        return false;
    }
    if !contains_valid_pid(map) {
        return false;
    }
    true
}

lazy_static! {
    static ref YEAR: Regex = Regex::new(r"^\d{4}$").unwrap();
    static ref HEIGHT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_COLOR: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn contains_valid_byr(map: &HashMap<String, String>) -> bool {
    if let Some(byr) = map.get(&"byr".to_string()) {
        if is_valid_byr(&byr) {
            return true;
        }
    }
    false
}

fn is_valid_byr(byr: &str) -> bool {
    if YEAR.is_match(byr) {
        let byr = byr.parse::<u16>().unwrap();
        1920 <= byr && byr <= 2002
    } else {
        false
    }
}

fn contains_valid_iyr(map: &HashMap<String, String>) -> bool {
    if let Some(iyr) = map.get(&"iyr".to_string()) {
        if is_valid_iyr(&iyr) {
            return true;
        }
    }
    false
}

fn is_valid_iyr(iyr: &str) -> bool {
    if YEAR.is_match(iyr) {
        let iyr = iyr.parse::<u16>().unwrap();
        2010 <= iyr && iyr <= 2020
    } else {
        false
    }
}

fn contains_valid_eyr(map: &HashMap<String, String>) -> bool {
    if let Some(eyr) = map.get(&"eyr".to_string()) {
        if is_valid_eyr(&eyr) {
            return true;
        }
    }
    false
}

fn is_valid_eyr(eyr: &str) -> bool {
    if YEAR.is_match(eyr) {
        let eyr = eyr.parse::<u16>().unwrap();
        2020 <= eyr && eyr <= 2030
    } else {
        false
    }
}

fn contains_valid_hgt(map: &HashMap<String, String>) -> bool {
    if let Some(hgt) = map.get(&"hgt".to_string()) {
        if is_valid_hgt(&hgt) {
            return true;
        }
    }
    false
}

fn is_valid_hgt(hgt: &str) -> bool {
    if let Some(cap) = HEIGHT.captures(&hgt) {
        let value = cap[1].parse::<u16>().unwrap();
        let unit = cap[2].to_string();
        if unit == "cm" && 150 <= value && value <= 193 {
            return true;
        }
        if unit == "in" && 59 <= value && value <= 76 {
            return true;
        }
    }
    false
}

fn contains_valid_hcl(map: &HashMap<String, String>) -> bool {
    if let Some(hcl) = map.get(&"hcl".to_string()) {
        if HAIR_COLOR.is_match(&hcl) {
            return true;
        }
    }
    false
}

fn contains_valid_ecl(map: &HashMap<String, String>) -> bool {
    if let Some(ecl) = map.get(&"ecl".to_string()) {
        if EYE_COLOR.is_match(&ecl) {
            return true;
        }
    }
    false
}

fn contains_valid_pid(map: &HashMap<String, String>) -> bool {
    if let Some(pid) = map.get(&"pid".to_string()) {
        if PASSPORT_ID.is_match(&pid) {
            return true;
        }
    }
    false
}

/// https://adventofcode.com/2020/day/5
/// Runtime complexity: O(n)
/// Space complexity: O(1)
pub fn day_05_part1(array: &[String]) -> Option<usize> {
    array
        .iter()
        .map(|ticket| {
            let (row, col) = ticket.split_at(7);
            let row = binary_search_ticket(row).unwrap();
            let col = binary_search_ticket(col).unwrap();
            row * 8 + col
        })
        .max()
}

fn binary_search_ticket(ticket: &str) -> Option<usize> {
    let (mut lo, mut hi) = (0, 2usize.pow(ticket.len() as u32) - 1);
    for c in ticket.chars() {
        let mid = lo + (hi - lo) / 2;
        if c == 'F' || c == 'L' {
            hi = mid;
        } else if c == 'B' || c == 'R' {
            lo = mid + 1;
        } else {
            return None;
        }
    }
    Some(lo)
}

/// https://adventofcode.com/2020/day/5#part2
/// Runtime complexity: O(n)
/// Space complexity: O(n)
pub fn day_05_part2(array: &[String]) -> usize {
    let (mut min_id, mut max_id) = (usize::max_value(), 0);
    let mut seat_ids = HashSet::new();
    for ticket in array.iter() {
        let (row, col) = ticket.split_at(7);
        let row = binary_search_ticket(row).unwrap();
        let col = binary_search_ticket(col).unwrap();
        let seat_id = row * 8 + col;
        min_id = min_id.min(seat_id);
        max_id = max_id.max(seat_id);
        seat_ids.insert(seat_id);
    }
    for seat_id in min_id..max_id {
        if !seat_ids.contains(&seat_id) {
            return seat_id;
        }
    }
    max_id
}

/// https://adventofcode.com/2020/day/6
/// Runtime complexity: O(n)
/// Space complexity: O(|alphabet|)
pub fn day_06_part1(groups: &[Vec<String>]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut chars = HashSet::new();
            for answers in group.iter() {
                for c in answers.chars() {
                    chars.insert(c);
                }
            }
            chars.len()
        })
        .sum()
}

/// https://adventofcode.com/2020/day/6#part2
/// Runtime complexity: O(|groups| * (|group| + |alphabet|))
/// Space complexity: O(|alphabet|)
pub fn day_06_part2(groups: &[Vec<String>]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut counter: HashMap<char, usize> = HashMap::new();
            for answers in group.iter() {
                for c in answers.chars() {
                    *counter.entry(c).or_insert(0) += 1;
                }
            }
            counter
                .values()
                .filter(|&&count| count == group.len())
                .count()
        })
        .sum()
}

/// https://adventofcode.com/2020/day/7
/// Runtime complexity: O(|colors|)
/// Space complexity: O(|colors|)
pub fn day_07_part1(rules: &HashMap<String, HashMap<String, usize>>, target: &str) -> usize {
    let mut cache = HashMap::new();
    rules
        .keys()
        .filter(|color| search_bags(&rules, &mut cache, &color, target))
        .count()
}

fn search_bags(
    rules: &HashMap<String, HashMap<String, usize>>,
    cache: &mut HashMap<String, bool>,
    key: &str,
    target: &str,
) -> bool {
    if let Some(&found) = cache.get(key) {
        return found;
    }
    let mut found = false;
    if let Some(content) = rules.get(key) {
        if content.contains_key(target) {
            found = true;
        } else {
            for color in content.keys() {
                if search_bags(&rules, cache, &color, &target) {
                    found = true;
                    break;
                }
            }
        }
    }
    cache.insert(key.to_string(), found);
    found
}

#[derive(Debug, PartialEq)]
pub struct NoSolution {
    why: String,
}

impl NoSolution {
    fn new(why: &str) -> NoSolution {
        NoSolution {
            why: why.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::num;
    use std::path::PathBuf;

    #[test]
    fn test_day_01_part1() -> Result<(), Error> {
        let mut array = input_day_01()?;
        let target = 2020;
        assert_eq!(day_01_part1(&mut array, target), Ok(793524));
        assert_eq!(
            day_01_part1(&mut vec![100, 200], target),
            Err(NoSolution::new(&format!(
                "no two numbers sum up to {}",
                target
            )))
        );
        Ok(())
    }

    #[test]
    fn test_day_01_part2() -> Result<(), Error> {
        let mut array = input_day_01()?;
        let target = 2020;
        assert_eq!(day_01_part2(&mut array, target), Ok(61515678));
        assert_eq!(
            day_01_part2(&mut vec![100, 200, 300], target),
            Err(NoSolution::new(&format!(
                "no three numbers sum up to {}",
                target
            )))
        );
        Ok(())
    }

    #[test]
    fn test_day_02_part1() -> Result<(), Error> {
        let array = input_day_02()?;
        assert_eq!(day_02_part1(&array), 439);
        Ok(())
    }

    #[test]
    fn test_day_02_part2() -> Result<(), Error> {
        let array = input_day_02()?;
        assert_eq!(day_02_part2(&array), 584);
        Ok(())
    }

    #[test]
    fn test_day_03_part1() -> Result<(), Error> {
        let lines = read_lines("2020-12-03.txt")?;
        assert_eq!(day_03_part1(&lines), 211);
        Ok(())
    }

    #[test]
    fn test_day_03_part2() -> Result<(), Error> {
        let lines = read_lines("2020-12-03.txt")?;
        assert_eq!(day_03_part2(&lines), 3584591857);
        Ok(())
    }

    #[test]
    fn test_day_04_part1() -> Result<(), Error> {
        let array = input_day_04()?;
        assert_eq!(day_04_part1(&array), 237);
        Ok(())
    }

    #[test]
    fn test_day_04_part2() -> Result<(), Error> {
        let array = input_day_04()?;
        assert_eq!(day_04_part2(&array), 172);
        Ok(())
    }

    #[test]
    fn test_day_05_part1() -> Result<(), Error> {
        let lines = read_lines("2020-12-05.txt")?;
        assert_eq!(day_05_part1(&lines), Some(861));
        Ok(())
    }

    #[test]
    fn test_day_05_part2() -> Result<(), Error> {
        let lines = read_lines("2020-12-05.txt")?;
        assert_eq!(day_05_part2(&lines), 633);
        Ok(())
    }

    #[test]
    fn test_day_06_part1() -> Result<(), Error> {
        let groups = input_day_06()?;
        assert_eq!(day_06_part1(&groups), 7110);
        Ok(())
    }

    #[test]
    fn test_day_06_part2() -> Result<(), Error> {
        let groups = input_day_06()?;
        assert_eq!(day_06_part2(&groups), 3628);
        Ok(())
    }

    #[test]
    fn test_day_07_part1() -> Result<(), Error> {
        let rules = input_day_07()?;
        assert_eq!(day_07_part1(&rules, "shiny gold"), 121);
        Ok(())
    }

    fn input_day_01() -> Result<Vec<u32>, Error> {
        let mut array = Vec::new();
        let lines = read_lines_iter("2020-12-01.txt")?;
        for line in lines {
            let line = line?;
            let x = line.parse::<u32>()?;
            array.push(x);
        }
        Ok(array)
    }

    fn input_day_02() -> Result<Vec<(usize, usize, String, String)>, Error> {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;
        let mut array = Vec::new();
        let lines = read_lines_iter("2020-12-02.txt")?;
        for line in lines {
            let line = line?;
            let cap = re.captures(&line).unwrap();
            let min = cap[1].parse::<usize>()?;
            let max = cap[2].parse::<usize>()?;
            let character = cap[3].to_string();
            let password = cap[4].to_string();
            array.push((min, max, character, password));
        }
        Ok(array)
    }

    fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
        let content = fs::read_to_string(filepath(filename))?;
        Ok(content.trim().split('\n').map(String::from).collect())
    }

    fn input_day_04() -> Result<Vec<HashMap<String, String>>, Error> {
        let mut array = Vec::new();
        let mut map = HashMap::new();
        let lines = read_lines_iter("2020-12-04.txt")?;
        for line in lines {
            let line = line?;
            if line.is_empty() {
                array.push(map);
                map = HashMap::new();
            } else {
                for pair in line.split_whitespace() {
                    let kvp = pair.splitn(2, ':').collect::<Vec<&str>>();
                    map.insert(kvp[0].to_string(), kvp[1].to_string());
                }
            }
        }
        Ok(array)
    }

    fn input_day_06() -> Result<Vec<Vec<String>>, Error> {
        let mut groups = Vec::new();
        let mut group = Vec::new();
        let lines = read_lines_iter("2020-12-06.txt")?;
        for line in lines {
            let line = line?;
            if line.is_empty() {
                groups.push(group);
                group = Vec::new();
            } else {
                group.push(line);
            }
        }
        groups.push(group);
        Ok(groups)
    }

    fn input_day_07() -> Result<HashMap<String, HashMap<String, usize>>, Error> {
        let re = Regex::new(r"(\d+) ([\w\s]+?) bags?")?;
        let mut rules = HashMap::new();
        let lines = read_lines_iter("2020-12-07.txt")?;
        for line in lines {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let rule = line.splitn(2, " bags contain ").collect::<Vec<&str>>();
            let mut content = HashMap::new();
            for cap in re.captures_iter(&rule[1]) {
                let color = cap[2].to_string();
                let count = cap[1].parse::<usize>()?;
                content.insert(color, count);
            }
            rules.insert(rule[0].to_string(), content);
        }
        Ok(rules)
    }

    fn read_lines_iter(filename: &str) -> Result<io::Lines<io::BufReader<File>>, Error> {
        let file = File::open(filepath(filename))?;
        Ok(io::BufReader::new(file).lines())
    }

    fn filepath(filename: &str) -> PathBuf {
        [env!("CARGO_MANIFEST_DIR"), "tests", "resources", filename]
            .iter()
            .collect::<PathBuf>()
    }
    #[derive(Debug)]
    enum Error {
        Io(io::Error),
        ParseInt(num::ParseIntError),
        Regex(regex::Error),
    }

    impl From<io::Error> for Error {
        fn from(error: io::Error) -> Self {
            Error::Io(error)
        }
    }

    impl From<num::ParseIntError> for Error {
        fn from(error: num::ParseIntError) -> Self {
            Error::ParseInt(error)
        }
    }

    impl From<regex::Error> for Error {
        fn from(error: regex::Error) -> Self {
            Error::Regex(error)
        }
    }
}
