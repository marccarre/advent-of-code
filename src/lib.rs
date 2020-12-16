use std::result::Result;

/// https://adventofcode.com/2020/day/1
/// Runtime complexity: O(n.log(n))
/// Space complexity: O(1)
pub fn day_01_part1(array: &mut Vec<u32>, target: u32) -> Result<u32, Error> {
    array.sort_unstable(); // Quicksort, given primitives.
    for &x in array.iter() {
        let y = target - x;
        if array.binary_search(&y).is_ok() {
            return Ok(x * y);
        }
    }
    Err(Error::new(&format!("no 2 numbers sum up to {}", target)))
}

/// https://adventofcode.com/2020/day/1#part2
/// Runtime complexity: O(n^2.log(n))
/// Space complexity: O(1)
/// N.B.: we could optimise the binary search by providing from & to indexes.
pub fn day_01_part2(array: &mut Vec<u32>, target: u32) -> Result<u32, Error> {
    array.sort_unstable(); // Quicksort, given primitives.
    for i in 0..array.len() - 1 {
        for j in i + 1..array.len() {
            let sum = array[i] + array[j];
            if let Ok(k) = array.binary_search(&(target - sum)) {
                return Ok(array[i] * array[j] * array[k]);
            }
        }
    }
    Err(Error::new(&format!("no 3 numbers sum up to {}", target)))
}

#[derive(Debug, PartialEq)]
pub struct Error {
    why: String,
}

impl Error {
    fn new(why: &str) -> Error {
        Error {
            why: why.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::PathBuf;

    #[test]
    fn test_day_01_part1() {
        let target = 2020;
        assert_eq!(day_01_part1(&mut input_day_01(), target), Ok(793524));
        assert_eq!(
            day_01_part1(&mut vec![100, 200], target),
            Err(Error::new(&format!("no 2 numbers sum up to {}", target)))
        );
    }

    #[test]
    fn test_day_01_part2() {
        let target = 2020;
        assert_eq!(day_01_part2(&mut input_day_01(), target), Ok(61515678));
        assert_eq!(
            day_01_part2(&mut vec![100, 200, 300], target),
            Err(Error::new(&format!("no 3 numbers sum up to {}", target)))
        );
    }

    fn input_day_01() -> Vec<u32> {
        let mut array = Vec::new();
        if let Ok(lines) = read_lines("2020-12-01.txt") {
            for line in lines {
                if let Ok(line) = line {
                    if let Ok(x) = line.parse::<u32>() {
                        array.push(x);
                    }
                }
            }
        }
        array
    }

    fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        let filepath = [env!("CARGO_MANIFEST_DIR"), "tests", "resources", filename]
            .iter()
            .collect::<PathBuf>();
        let file = File::open(filepath)?;
        Ok(io::BufReader::new(file).lines())
    }
}
