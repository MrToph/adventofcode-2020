#![allow(clippy::map_entry)]
use std::collections::HashMap;

pub fn run(inputs: &[u64]) -> Result<u64, &'static str> {
    let mut inputs = inputs.to_owned();
    inputs.sort_unstable();

    //
    let mut one_differences = 0;
    // your device's built-in adapter is always 3
    let mut three_differences = 1;
    // The charging outlet has an effective rating of 0 jolts
    let mut prev: u64 = 0;
    inputs.iter().for_each(|num| {
        let diff = num - prev;
        if diff == 1 {
            one_differences += 1;
        } else if diff == 3 {
            three_differences += 1;
        }
        prev = *num;
    });

    Ok(one_differences * three_differences)
}

// part 2
pub fn count(adapters: &[u64], target: &u64, hashmap: &mut HashMap<usize, u64>) -> u64 {
    if !hashmap.contains_key(&adapters.len()) {
        if adapters.is_empty() {
            return 0;
        } else if adapters.len() == 1 {
            // only valid if within range of 3 from target
            if target - adapters[0] <= 3 {
                return 1;
            } else {
                return 0;
            }
        }

        // now do dynmaic programming, see what we could reach within a single step (max 3 diff)
        let mut sum: u64 = 0;
        let current = adapters[0];
        for (index, jolt) in adapters.iter().enumerate().skip(1) {
            if jolt - current <= 3 {
                sum += count(&adapters[index..], target, hashmap)
            } else {
                break;
            }
        }

        hashmap.insert(adapters.len(), sum);
    }

    return *hashmap.get(&adapters.len()).unwrap();
}
pub fn run2(inputs: &[u64]) -> Result<u64, &'static str> {
    let mut inputs = inputs.to_owned();
    inputs.insert(0, 0);
    inputs.sort_unstable();

    let my_adapter = inputs.last().unwrap() + 3;

    let mut cache = HashMap::<usize, u64>::new();
    Ok(count(&inputs, &my_adapter, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [u64; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const PUZZLE_INPUT: [u64; 94] = [
        54, 91, 137, 156, 31, 70, 143, 51, 50, 18, 1, 149, 129, 151, 95, 148, 41, 144, 7, 125, 155,
        14, 114, 108, 57, 118, 147, 24, 25, 73, 26, 8, 115, 44, 12, 47, 106, 120, 132, 121, 35,
        105, 60, 9, 6, 65, 111, 133, 38, 138, 101, 126, 39, 78, 92, 53, 119, 136, 154, 140, 52, 15,
        90, 30, 40, 64, 67, 139, 76, 32, 98, 113, 80, 13, 104, 86, 27, 61, 157, 79, 122, 59, 150,
        89, 158, 107, 77, 112, 5, 83, 58, 21, 2, 66,
    ];

    // #[test]
    // fn part1_works_for_sample_input() {
    //     assert_eq!(run(&SAMPLE_INPUT).unwrap(), 35);
    // }

    // #[test]
    // fn part1_works_for_puzzle_input() {
    //     assert_eq!(run(&PUZZLE_INPUT).unwrap(), 2046);
    // }

    #[test]
    fn part2_works_for_sample_input() {
        assert_eq!(run2(&SAMPLE_INPUT).unwrap(), 8);
    }

    #[test]
    fn part2_works_for_puzzle_input() {
        assert_eq!(run2(&PUZZLE_INPUT).unwrap(), 1157018619904);
    }
}
