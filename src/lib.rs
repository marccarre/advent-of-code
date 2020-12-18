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
    use regex::Regex;
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
        let lines = input_day_03()?;
        assert_eq!(day_03_part1(&lines), 211);
        Ok(())
    }

    #[test]
    fn test_day_03_part2() -> Result<(), Error> {
        let lines = input_day_03()?;
        assert_eq!(day_03_part2(&lines), 3584591857);
        Ok(())
    }

    fn input_day_01() -> Result<Vec<u32>, Error> {
        let mut array = Vec::new();
        let lines = read_lines("2020-12-01.txt")?;
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
        let lines = read_lines("2020-12-02.txt")?;
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

    fn input_day_03() -> Result<Vec<String>, Error> {
        let content = fs::read_to_string(filepath("2020-12-03.txt"))?;
        Ok(content.trim().split('\n').map(String::from).collect())
    }

    fn read_lines(filename: &str) -> Result<io::Lines<io::BufReader<File>>, Error> {
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
