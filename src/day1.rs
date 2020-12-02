use std::cmp::Ordering;

const TARGET_SUM: usize = 2020;

pub fn run(list: &[usize]) -> Option<usize> {
    let mut list = list.to_owned();
    list.sort_unstable();
    let mut left = 0;
    let mut right = list.len() - 1;

    // find a,b in L, s.t. a + b = TARGET_SUM
    while left < right {
        // smallest value
        let a: &usize = &list[left];
        // largest value
        let b: &usize = &list[right];
        let sum = a + b;

        match sum.cmp(&TARGET_SUM) {
            Ordering::Equal => return Some(a * b),

            // can discard smallest value a
            // as forall c in L: a + c <= a + b < TARGET
            Ordering::Less => left += 1,

            // can discard highest value b
            // as forall c in L: c + b >= a + b > TARGET
            Ordering::Greater => right -= 1,
        }
    }

    panic!("should never get here {} {}", left, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];
    const PUZZLE_INPUT: [usize; 200] = [
        1935, 1956, 1991, 1425, 1671, 1537, 1984, 1569, 1873, 1840, 1720, 1937, 1823, 1625, 1727,
        1812, 1714, 1900, 1939, 1931, 1951, 1756, 1942, 1611, 1979, 1930, 1996, 2000, 1544, 1780,
        1687, 1760, 1836, 1814, 1691, 1817, 1964, 1899, 1577, 1547, 866, 1560, 1988, 1601, 1970,
        1738, 1507, 1667, 1851, 1933, 1515, 1856, 1969, 1860, 1801, 2007, 1866, 1800, 1749, 1843,
        1711, 1495, 1905, 763, 1672, 1858, 1987, 1492, 1849, 1993, 1737, 1874, 1658, 1810, 1665,
        1768, 1950, 1879, 1816, 1868, 1995, 1763, 1783, 1833, 1968, 1847, 1748, 1725, 1891, 1755,
        286, 1976, 1977, 1655, 1808, 1986, 1779, 1861, 1953, 1888, 1792, 1811, 1872, 1790, 1839,
        1985, 1827, 1842, 1925, 1735, 1635, 1821, 1820, 1973, 1531, 1770, 59, 1846, 1932, 1907,
        1730, 933, 1395, 1753, 1751, 361, 1530, 1782, 1087, 1589, 1929, 1795, 1815, 1732, 1765,
        1877, 1722, 526, 1709, 1789, 1892, 1913, 1662, 1809, 1670, 1947, 1835, 1587, 1758, 1982,
        2009, 1757, 670, 1983, 1524, 1878, 1796, 1952, 566, 1922, 1882, 1870, 1799, 1731, 1724,
        1805, 2003, 1596, 1566, 1853, 1911, 1857, 1739, 1744, 1627, 1729, 1745, 1845, 1582, 1884,
        1883, 1941, 1764, 1685, 1791, 1837, 1697, 1742, 1781, 1948, 1876, 1989, 1643, 1871, 1906,
        1726, 1958, 1502, 1927, 1946,
    ];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT).unwrap(), 514579);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run(&PUZZLE_INPUT).unwrap(), 1014171);
    }
}
