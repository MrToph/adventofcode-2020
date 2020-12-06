pub fn run(inputs: &[&str]) -> Result<usize, &'static str> {
    let rows: Vec<Vec<char>> = inputs.iter().map(|s| s.chars().collect()).skip(1).collect();

    let mut x = 0; // start at top left
    let mut result = 0;
    for row in rows {
        x = (x + 3) % row.len();
        if row[x] == '#' {
            result += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [&str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    const PUZZLE_INPUT: [&str; 323] = [
        "............#....#.............",
        "...........##....#......#..#..#",
        "......#.......#......#.........",
        "..#.#....#....#.............##.",
        "..#........####....#...#.......",
        "..##.....#.#.#..#.........#....",
        "...#.#..#..#....#..#..#........",
        "#.......#.........#....##.###..",
        "......##..#.#...#.......#.#....",
        "................##.........#.##",
        "..##..........#...#.........#.#",
        "..........#...##...............",
        "#...#......#..#.#..#...##..#...",
        "..##....#.......#......#..#....",
        "....#......#......#....#.......",
        ".........#.....#..#............",
        ".#...#.#.........#........#....",
        "#..........####.....#..........",
        "......##.....#....#..#........#",
        "#......#......#...........#....",
        "....#.........#....#...#..#..#.",
        ".#........#......#.#.....#.....",
        "..#.#.#..........#....#.......#",
        "......#.#........##....##....##",
        ".....#.#..#...#................",
        "......#......##...............#",
        "..#..##.............#...##.....",
        "......##......##..#......#.....",
        "....#.............#..##.....##.",
        "........#...............##.....",
        "..#......#.##..#...#....#...#..",
        "#......#.......#.............#.",
        ".....#....##..............#....",
        "#.#.........#....#..##....#....",
        ".#...#...#....#.#............#.",
        "...#...#.#..##.##.......##.....",
        "......#..#....##..#.#..#..#....",
        ".......##..#..#......#..#.....#",
        ".##..#......#..........#....#..",
        ".....#................#..#....#",
        "........#..#....#.......#....#.",
        "..#......#.......#......#....#.",
        "....#...#.##........##....#....",
        ".....#........#...........#....",
        "...#....##..........#..#...#.#.",
        "...#.......#......#...##...#...",
        ".#.....#........#........#.#..#",
        ".#.........#..##.....#.......#.",
        "....#..#....#.......#......#...",
        ".#.#...##..##................##",
        "......#.#...#.......#....#....#",
        "........#....#..#.....#......#.",
        ".......#..........#......#.....",
        "...............................",
        "..#..#####..#..#..........#.#..",
        ".....#....##................#.#",
        ".................##............",
        ".#...#...#..#...........#...##.",
        "..#..#.#...........#.....##....",
        ".#.......#.....#..##..#.#....#.",
        "..........#.#......##...##.....",
        "........##..#......##...#......",
        "#......................#.......",
        "............#.....#....#.#...#.",
        "#......#..........##..#........",
        ".........#.......#...#.#.......",
        "...........##....#........#....",
        "#........#.....#...#........##.",
        ".#......##......#.##.......#..#",
        ".....#......#.#......#.......#.",
        ".....#.#.........#.............",
        "...........#..#....#.....#.#...",
        "...#............#...#..........",
        "..#..#...#.....................",
        "......#..#...#....#............",
        ".#.#.#........#..#...#.........",
        "..........#........#..#........",
        "..............#...#....#.......",
        "..#....#....##.......#...#.##..",
        ".#.........#...#......#........",
        "..#......#...#.........##.#...#",
        "...#.....#...#..#.............#",
        ".##........#.#.#.............#.",
        "..#.............#..#.#...#....#",
        "#...#.........#......#......#..",
        ".......##..#.#..........#...#..",
        ".......#.............#..#.#....",
        ".#..#....#.#...................",
        "....##...#..#....#..#..........",
        "....#.#............#...........",
        "###........##..#.#..#..........",
        ".#.#.#.......#...........#..#.#",
        "..........##..#.............#..",
        ".#...........#......#.#..#..##.",
        "...###......#.##........#.....#",
        "....#..#..#...#................",
        "...#.....#........#............",
        "....#...#...#..#..##.##.......#",
        "#.......#......#....#.......#..",
        "#.............#...#............",
        "##......#..#...#....##.#...#...",
        ".##....................#....#..",
        "..#.....#....#.#....#......#...",
        ".......#..#..#............#...#",
        ".#.....#.......#..#..#..#......",
        "......##.......................",
        "#..#...#.#.#....#.....#..#.....",
        "...................#...#...#...",
        "........#....##..#....#........",
        "##......#.#......##.###........",
        ".........#...##................",
        ".......#...#...#.......##......",
        "....#.......#......#.........##",
        "....#....#.#..#.....#..........",
        "...........#.......#........#..",
        "..#.........###.#........#.....",
        ".......#...........#.#.....##..",
        "..#...#..#..........#..........",
        "..........#.#....#.............",
        ".##....#........##.............",
        ".............#.#####........#.#",
        ".................##...#........",
        "##...#.#.......##........#.....",
        ".#...#...#..#..#....#....#.....",
        "..#...#........#..#............",
        "##...#.#........#......##.#..##",
        ".##......#..............##.#..#",
        ".........#...#............#...#",
        "....#..#....#...........#......",
        "........#..#....#...##...#.....",
        "..#..............#...#.#.....#.",
        ".#.......#.#.....#..###.......#",
        "...................#.......#...",
        "........##.....#..#.......##...",
        ".....#....................#...#",
        "...#.#....#............#.#.....",
        "#.......#.......#....#.........",
        "..#...............#............",
        "##...#...#...#..............#..",
        "...#..........#..#....##.......",
        "#............##.##......#.#.#..",
        ".#...........#.........#....##.",
        "..##....##.#....#.#.#.##...##.#",
        "........#.#.#.............#....",
        ".#...........#....##...#...#.#.",
        ".##...#.................#......",
        "....#.#..#....................#",
        ".##......#........#..#.........",
        "...#...............#...........",
        ".#.#..##..##.#........#........",
        "...........#....#.#.#......#...",
        "...................#........#.#",
        "..#............#...#.#........#",
        "....#....#.#.##......#...#.....",
        "..................#............",
        "..........................#....",
        "........#......................",
        "......#.#...#.#..##......#.#.#.",
        ".........#...#..#..............",
        "..#.......#..........##..#.....",
        ".........#............#........",
        "......#..#..#...###....#....#..",
        "#..#..............##.###..##..#",
        ".#..................#.....#...#",
        "........#........#........#....",
        ".........#........#.##......#..",
        "..#.....#.#..###...#....#......",
        "..#................##....#.....",
        "..#.#....##.....#......##...#..",
        "...#.......#........##.........",
        "#........#...#.#..........##..#",
        "................#...#.#.....#..",
        ".........#..#..#.#..#.#...#....",
        "##....#...##.........#.#...#.##",
        "....#..#.....##.....#.....##...",
        "................#............#.",
        "..#..#...#.....#......#.....##.",
        "....#.......#...#...#...#..#...",
        "....#..##....#.###.#...#..#....",
        "#..##.....#.....#.##..##...##.#",
        ".............###..........#....",
        "..................#.....###....",
        "..........#....#...#......#....",
        "...#..##.......#......#.#...#..",
        "..#.......................##.#.",
        "..#..#..#....#......#...#...##.",
        "#.............#................",
        "..........#.#.#.........#.#....",
        ".....##..#......##.#...........",
        ".#.#.#.#....#.#...#.....#.#...#",
        "......#.....##..............##.",
        "#..#.......##..##..............",
        "#..#..#................###.....",
        ".....#......#.........#........",
        "#...........#........#.#.......",
        "#........#.#...#....#....###..#",
        "###..#.#...........#.##.....#.#",
        "..#..........#..#............#.",
        "...#....#.......#..#.....###...",
        ".#....#.##.#..###..............",
        ".....#.##.##.......###.##...#.#",
        "..#..##.......###..............",
        ".#.........###..#..............",
        "..................###.....#..#.",
        "#....#....#.........#.....#....",
        ".........#.#..#....#.....#.....",
        "....##.......##.......#.#......",
        ".....#...#.##.....#............",
        "....#.#.#.......#..............",
        ".##..#.#..#.......##...........",
        "....#....##..#.....##.......#.#",
        ".....##....#..#.#........#.....",
        "........#.#.#....#....##...#..#",
        "..#......#.#.#..#.##....#.#.#..",
        "..#...#........#..#..........#.",
        ".........#...................#.",
        "........#.....##..#....#....#..",
        "#..............#..........#....",
        "#........#.#...........#.#.....",
        "..#......................#.#..#",
        ".........#.#.....#.#..........#",
        "......#....#.#.##........#.....",
        ".#....##......##..#...#.......#",
        "..#........#...#.##....#..#.#..",
        ".......#.....#..........#.....#",
        ".........#.#..#.........#....#.",
        "..........#.##.........##..#...",
        "......#.#..#.....#.#..........#",
        "......#.#.#..#..#.#............",
        "...##.#..#..............#....#.",
        "#..........#...................",
        ".#....#..#.#.......#........#..",
        "...#...#......#....#......#....",
        "..#.#.......#.......#.......#.#",
        "...#.#...#........#.....#......",
        "#.......#..#...................",
        "#..#..#.............#..#..#..#.",
        "#.......................#....##",
        ".#.........#....#....#.........",
        "...............#...#..#....#..#",
        "#.....#.#...#.#.....#..........",
        "....##.#..#...#.#....###...#.#.",
        ".................#....#........",
        "####.......##...##.......#.##..",
        "#..#....#....##............#...",
        "..##......#..#........#........",
        "....#..#..........#......#...##",
        "..#.#.............#...........#",
        "#...............#...#.......#.#",
        "#..#.........#.##.#.......#...#",
        "......#.....#.............#...#",
        "......#.##.........##...#......",
        "..#......##.#........#.......#.",
        "#..#.........#.##..............",
        "..#....#...#...#..#.....#.#....",
        "................#.......#......",
        "#.....#..............##....#.##",
        "##.....#...#.#.....#..##...#...",
        "#.#............##..........#..#",
        "..#.##......#..#....#..........",
        "....##.#....#.......##.....#...",
        "......#.#....###...#...........",
        "..................#......#....#",
        "..............##...............",
        "......#..#....#.....#..........",
        ".......#........#...#..........",
        "..#......#......##..#.##..#....",
        "..#.#...#...............#......",
        "....#.#.............#.#......#.",
        "....#.#.....#......#..#.......#",
        "........................#..#...",
        ".................#...........#.",
        "#......#......#.#.#.....##.....",
        "..#....##...#.....##.#.....#..#",
        "....#.........#....#.##.#.#....",
        "..#....###.....................",
        ".....#.#....#......#....##....#",
        "#.......#...#......##.......#..",
        "#....#.........##.....#........",
        "#.....#...........#..#.....#...",
        ".................#.....#..##..#",
        "..#...#......####...##.........",
        "...............................",
        "#........#.....#...............",
        ".#.........#....#.#......##....",
        "...#..........#.........#.#.#.#",
        "......##......#....###........#",
        ".....................#.#.#.....",
        "......#..#..#.......#...#......",
        "...##.#.............#.#.......#",
        "..#.#...#..#....#.....#.....#..",
        "..#..#.....................#..#",
        "........#....#..........#..#...",
        "#.##....#..#.#..#............#.",
        "..............###.............#",
        ".#.#..........#.#....#...#....#",
        "....#..........#.#..#......#...",
        ".........##.#...#..............",
        "..................#.....#.#....",
        ".#....#.......#.##.#.........#.",
        ".##..#...#......#..#...........",
        ".#.........#..........#.#......",
        "#.#......#.#.#.#.......#...#.#.",
        ".......#....#.#......#......#..",
        "...#..#....#.#..#..##...##.....",
        "#.#.#.......#....#.........##..",
        "#..#....#........###....#.#....",
        "....#..#.........#....#...#....",
        "...#.#.#.#..#..##.....#.##.....",
        ".......#.......#...............",
        "#.#.#......##....#.............",
        "...#.##........#.....#...##.#..",
        "...#.#.###..........#.......#..",
        ".....#...#.......#.........#...",
        "............#..#...#..##.......",
        "...#....#..##.##..........#.##.",
        "..................#........#...",
        "....#.##.#.##........#.#.......",
        ".#...........##.....##.......#.",
        "#...#.........#.....##.........",
        "#..#....#.#.........#..........",
        "..#......#.#.#......#.....#..#.",
        "..##......#..............#.....",
    ];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT).unwrap(), 7);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run(&PUZZLE_INPUT).unwrap(), 171);
    }
}
