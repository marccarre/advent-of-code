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
        return vec![
            1293, 1207, 1623, 1675, 1842, 1410, 85, 1108, 557, 1217, 1506, 1956, 1579, 1614, 1360,
            1544, 1946, 1666, 1972, 1814, 1699, 1778, 1529, 2002, 1768, 1173, 1407, 1201, 1264,
            1739, 1774, 1951, 1980, 1428, 1381, 1714, 884, 1939, 1295, 1694, 1168, 1971, 1352,
            1462, 1828, 1402, 1433, 1542, 1144, 1331, 1427, 1261, 1663, 1820, 1570, 1874, 1486,
            1613, 1769, 1721, 1753, 1142, 1677, 2010, 1640, 1465, 1171, 534, 1790, 2005, 1604,
            1891, 1247, 1281, 1867, 1403, 2004, 1668, 1416, 2001, 1359, 686, 1965, 1728, 1551,
            1565, 1128, 1832, 1757, 1350, 1808, 1711, 1799, 1590, 1989, 1547, 1140, 1905, 1368,
            1179, 1902, 1473, 1908, 1859, 1257, 1394, 1244, 1800, 1695, 1731, 1474, 1781, 1885,
            1154, 1990, 1929, 1193, 1302, 1831, 1226, 1418, 1400, 1435, 1645, 1655, 1843, 1227,
            1481, 1754, 1290, 1685, 1498, 71, 1286, 1137, 1288, 1758, 1987, 1471, 1839, 1545, 1682,
            1615, 1475, 1849, 1985, 1568, 1795, 1184, 1863, 1362, 1271, 1802, 1944, 1821, 1880,
            1788, 1733, 1150, 1314, 1727, 1434, 1833, 1312, 1457, 160, 1629, 1967, 1505, 1239,
            1266, 1838, 1687, 1630, 1591, 1893, 1450, 1234, 1755, 1523, 1533, 1499, 1865, 1725,
            1444, 1517, 1167, 1738, 1519, 1263, 1901, 1627, 1644, 1771, 1812, 1270, 1497, 1707,
            1708, 1396,
        ];
    }
}
