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

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT).unwrap(), 35);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run(&PUZZLE_INPUT).unwrap(), 2046);
    }
}
