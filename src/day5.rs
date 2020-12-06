use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug)]
#[from_str(regex = "(?P<row_instructions>[FB]{7})(?P<col_instructions>[RL]{3})")]
struct PassRow {
    row_instructions: String,
    col_instructions: String,
}

pub fn binary_search(instructions: &str) -> u32 {
    let mut low = 0;
    let mut high = 2u32.pow(instructions.len() as u32) - 1;
    for char in instructions.chars() {
        // println!("char {} low {} high {}", char, low, high);
        if char == 'F' || char == 'L' {
            high = low + (high - low) / 2;
        } else {
            low = low + (high - low) / 2 + 1;
        }
    }
    low
}

pub fn run(inputs: &[&str]) -> Result<u32, &'static str> {
    let rows: Vec<PassRow> = inputs.iter().filter_map(|s| s.parse().ok()).collect();

    let mut result = 0;
    for row in rows {
        // println!("row {}, col {}", row.row_instructions, row.col_instructions);
        let row_index = binary_search(&row.row_instructions);
        let col_index = binary_search(&row.col_instructions);
        let seat_id = row_index * 8 + col_index;
        // println!("row_index {}, col_index {}", row_index, col_index);
        if seat_id > result {
            result = seat_id;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [&str; 3] = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
    const PUZZLE_INPUT: [&str; 908] = [
        "FBBFBBBRLL",
        "FBBFFBFLLL",
        "BFFFFFFLRL",
        "FBFBFFFLRR",
        "BBBFBFBRLL",
        "BFBBFFBRLL",
        "BBFFFFBRLL",
        "BFBFBBBLRR",
        "FFBBFBFLLR",
        "FBBBBFBRRR",
        "FFBFBBFRLR",
        "FBBFFFFRRR",
        "BFBFFBFRRL",
        "FFFBBFFLLL",
        "FFFFBBFLLL",
        "FFBFFBBRLL",
        "FBBBBBFLRR",
        "FFFBFFFLRR",
        "FBBBBBFRLR",
        "BFFBFFBLRL",
        "FBBFBFBRRL",
        "FBFBFBFRRR",
        "FBBBFBFLRR",
        "FFBBFBFRRL",
        "FFBFFBFLRL",
        "BFBFBFBLRR",
        "BBFBBBFRRL",
        "BFBFBBFRLL",
        "FBBFBFBLLR",
        "BFFFBFFLRR",
        "FBBBFFFRLR",
        "FBBFFFFLLL",
        "BFFFBBBLRL",
        "BFBBBFBRLR",
        "BFBBFFFLRL",
        "FFFBFFFRRR",
        "BBFFBFFRLL",
        "FFBFBBFRRR",
        "BBBFFBBLLL",
        "FFBBBFFRRR",
        "BFFBFBBRLR",
        "BBFBBFFRLR",
        "FFBFBBFLRL",
        "BFBBFBBLLL",
        "FBBFBFBLLL",
        "FBBFBBFLRL",
        "BFBFBBBRLR",
        "FFBFFFFLRL",
        "FBBFFFFRLR",
        "BFBBBFBRLL",
        "FFFBBBBLLR",
        "BFFFBFFLLL",
        "BFFBFFFRRR",
        "FBFFFFBRLL",
        "BFBFFFBRLR",
        "FFBBFFFLRL",
        "FFBFBFBLLL",
        "FBFBBFBRLR",
        "BBBFFFFLLR",
        "BBFFBBBRLR",
        "FFBFBFBLRR",
        "BFBBFBFLLL",
        "FFBFFBBLLR",
        "FBBFFBBLLL",
        "BFBFBFFRRR",
        "BBFBBBFRLL",
        "BFBFFBFLRR",
        "BBBFFFFRLR",
        "BFFBFBFLRR",
        "FFFBFFFRLR",
        "FFBBFFBLRL",
        "FFBFFBBLRL",
        "FFBBBBFLLL",
        "FBBBFBBLRR",
        "FFBFBFFRRR",
        "BFFBFBFRRL",
        "FFBBBFFLLL",
        "BFFBBFFLLL",
        "BFBFFFBLRR",
        "FFBBBBFRRR",
        "BBFFBBFRLR",
        "FFFBBFFLRL",
        "BFFBBFFRRL",
        "FFFBBBFLLL",
        "FBBFFBBLRR",
        "BFFFBFFLLR",
        "BBFBFBFLRL",
        "BBFFFFFLRL",
        "FBBBFFBLLR",
        "FBFFFFFLLL",
        "BBFFBBFLRL",
        "FBBBBBFLLR",
        "BBBFBFFLLL",
        "FBBBBBFRRL",
        "BFBBFFBRLR",
        "FFBBBBBLLR",
        "FFBFBFFLRL",
        "BBFBBBBLRR",
        "BBFFBBBRRR",
        "FBFBFBFRLL",
        "FBBBFFFRLL",
        "BFFBBBBLLR",
        "BFFFFFBLLR",
        "FFBBBBFRLL",
        "BFFFFBBLRR",
        "FBBFBBBRLR",
        "BBBFFFBLRL",
        "BFFBBFBRRR",
        "BBFBBFBRRR",
        "BFFBBBBRRR",
        "BBFFBBFLLR",
        "FBFFBBBLLR",
        "BFBBBBFRLL",
        "BBFBFBFLLL",
        "BFBBFFBRRR",
        "FBFBFBFRLR",
        "FFFBFFBLRL",
        "FBFBBFFRLR",
        "BFBFFBBRLL",
        "BBFBFFFRLL",
        "FFFBBBFRLR",
        "FFBBBFBRRR",
        "BBFBFBBRLR",
        "FBFBFBFLRR",
        "BBFFFBBRLL",
        "BBFFBFBLLL",
        "FFFFBFBRRR",
        "BBFFBBBLRL",
        "BBBFFFFLLL",
        "FFBBBFBRLR",
        "FBFFFBFRLR",
        "BFBFBBBLLR",
        "FFFBBFBRLL",
        "BFFFFBBLLR",
        "BFBBBFBLLL",
        "BBFFBFFLRR",
        "FBFFFBFRLL",
        "BBFFFBFLRR",
        "FFBBFFBLRR",
        "FBFFBFFLRL",
        "FFFBBFFRRL",
        "BFFBFFFLRL",
        "FFFFBFBLRL",
        "FBFBBBBRLR",
        "FFBBFBBRLL",
        "FFFBBBFLRL",
        "FFBFBBFLRR",
        "BFBBBBFLRL",
        "FBFBBFBLRL",
        "FBBFFBBRRL",
        "FBFFBBFRRL",
        "BFFBBFFRRR",
        "FBFBFFFRRR",
        "FFBBFBFLLL",
        "FBBFFFBLRL",
        "BFBBBBFLRR",
        "FBFBBFFLRL",
        "BBFBBFBLLL",
        "BBBFFBFRLR",
        "FBBBFFFLRR",
        "BBFFFBFRLR",
        "BBBFFBFLLL",
        "FBBFFFBRRR",
        "FFFBBBFLRR",
        "FFBFBBBRLL",
        "FBFBBFFRRL",
        "BFBBFBFRLL",
        "FBFFBBBLLL",
        "FFFFBBBLLL",
        "FFBBBBBRRL",
        "BBBFBFFLRL",
        "FBBBBFBLRL",
        "BBFFBBBLLL",
        "BFFBFFFLLR",
        "FBBBBBBLLL",
        "FFFBFBBRLL",
        "FFBFBBFLLL",
        "FFBFBBBRLR",
        "BFBBBFBLRR",
        "FBBBBBBLRL",
        "BFBBFBBRRL",
        "BBFBBBFLLR",
        "FBFFBFBRLR",
        "FFBFBBBLRL",
        "FFBBFFFRRR",
        "FBBBBFFLLL",
        "FFBFFBFRLL",
        "FFBFFBBLRR",
        "BFBFFBFRLL",
        "FFFFBFBRLL",
        "FBFFBFBLLR",
        "FFFBFBBLLL",
        "FBBBBBFRRR",
        "FFFBBBBRLL",
        "FBFFBFBLRL",
        "BBFBFBFRRL",
        "FFFBFFBLRR",
        "BFFBFFBRRR",
        "FBFFFFFLRL",
        "FBBFBFFLRR",
        "FBFFBFBLRR",
        "FBFFFBBLLL",
        "FFBBFBBLLL",
        "BFFFFBFLLL",
        "FBBBFBFLRL",
        "BBFBFFBLLR",
        "BBBFFBBRRL",
        "FBFBFFFRLR",
        "FBFFBFBRRR",
        "FBBBFFFRRL",
        "BFBFFBBRLR",
        "FBBFFFFRRL",
        "BBFFFFFRLR",
        "FFFBFBFRLR",
        "FBBFFFFLRL",
        "BBFFBBBLLR",
        "BFFFBFBLLL",
        "FFFBFFFLLR",
        "FBFBBFBRRL",
        "BFBBBFFLLR",
        "BFBFBBFLLR",
        "BFBFBFFLRL",
        "FBFFBBFLLR",
        "FFFBFFFRLL",
        "FBFBFBFLLL",
        "FBFFFBFLLR",
        "FFFBFBBRRR",
        "BFFFBBBRRR",
        "FBBFBFFLLL",
        "BBFFFBBRRL",
        "FBBBFBBLLR",
        "FBFBBFBRRR",
        "BFFFBBFLRR",
        "BBFBFFFRRR",
        "FBBBBBBLRR",
        "BFBFBBFRRR",
        "BBFBBBBRLR",
        "FFBBFFBLLR",
        "FFBFBFFRLL",
        "BBFFBFFRRL",
        "BBFFBFBLRR",
        "FBBBFFBLLL",
        "FBFBFFFLLL",
        "BBBFBFFRRR",
        "BBFFFFBLLR",
        "BBBFBFBLLR",
        "BFBFBFFLLL",
        "BFBFFFBLLL",
        "BBFBFFFRLR",
        "FBFBBBFRLR",
        "FBFBBBFLRR",
        "FBFBBBBLLR",
        "FBBBFBFLLL",
        "FFBBFFBRRR",
        "FBBFBBFLLL",
        "FBFBFFBRRL",
        "BFBFBBBRRR",
        "BFFBBBBLRL",
        "FFFBBBBLLL",
        "BFBBBFFLLL",
        "FBBFFBFRLR",
        "FFFBFFFLRL",
        "BBFFFBBLLR",
        "BBBFFBFRRR",
        "BBFBBBBRRL",
        "FFBFFFFRLR",
        "FFBBBFBLLL",
        "FFBFFFFLLL",
        "BFFFFBFLRR",
        "BFFFBFFRRL",
        "BFBBFBBLRL",
        "FBFBBBFLLR",
        "FFFBFBBLRR",
        "FFBFFBBRRL",
        "FFBBFBBLRR",
        "FFBFFFBRRL",
        "BBBFFFBLRR",
        "BBFBFBBRLL",
        "BFBBBBBLLL",
        "FFBBBBBRLL",
        "FBFBFBBLLR",
        "BFBFFFFLRR",
        "FBBBBBBRLL",
        "FBFFFBBRLL",
        "FBBBBFBLLL",
        "BFFBFFFLLL",
        "BBFBFBFRLR",
        "BFBBBFFLRR",
        "BBFBFFFLRR",
        "BFFBBBFLRL",
        "BFBFFBFLRL",
        "FFFBFBBRRL",
        "BBBFFBBLLR",
        "FBFBBBBLLL",
        "FBBFBBFLLR",
        "FBFFBFBRRL",
        "BFBBFFFLRR",
        "FBFBBBBRRL",
        "FBBBBFBLLR",
        "FBFFFFFLRR",
        "FFBBBFFRLR",
        "BFBBBBBRRL",
        "BBBFFFBLLR",
        "BFFBBBFLRR",
        "FFBBBBBLRR",
        "FBBFFBFRLL",
        "BFBFBFFRRL",
        "FFFBFBFRLL",
        "FFBFFFBRRR",
        "FFFFBBFRLR",
        "BBFFFBFLLL",
        "FFFFBFFRLL",
        "FBBBFBFRRL",
        "FBBBFFFRRR",
        "FFFFBBFRRR",
        "FFBFFFBLRL",
        "BBFBBBFRRR",
        "BFFBFFFRLR",
        "FFFBFBFRRR",
        "FFFFBBBRLL",
        "FBFBFFBLRR",
        "BBFFBFFLLL",
        "BBBFFBBLRL",
        "BBFFBFBLRL",
        "FFBBFBBRRR",
        "BFFBFBBRLL",
        "BFBBBBBRLL",
        "BBFFBBBLRR",
        "FFBBBFBLRR",
        "FBBFBBBLLR",
        "BFBBBBBRRR",
        "BBFBBFFLRR",
        "BBFFFBBLLL",
        "BFBBFFFRRR",
        "BFBBFFBLLL",
        "BFBFBBBLRL",
        "BFBFFBBLRR",
        "BBFFFBBLRL",
        "FBBFFBBRLL",
        "FBFFBFFRLR",
        "FBFFBFFLLR",
        "BFFFBBFLRL",
        "FFBFFBFRRR",
        "BFBFFBFRRR",
        "BFFFFBBRLR",
        "FBBFFBFRRL",
        "FFFBFBBLRL",
        "BFFFBBBRLL",
        "FBFFBFFRRL",
        "BBBFBBFLLL",
        "BBBFFBFLRL",
        "FBFFBBFLLL",
        "FFFFBBFLRR",
        "BFFFFBFRLR",
        "BFBBBFBRRR",
        "BBFBBFFLRL",
        "BFBBFBFRRL",
        "BFFBFBFRLL",
        "FBFFBFFLLL",
        "BFFFBBFRLR",
        "FFBBFBFLRR",
        "BFBBFBBRLL",
        "FFFBBBBLRL",
        "BFBBBFBRRL",
        "FBFFFFFRLR",
        "FFBFBFBLLR",
        "BFBBFBBLLR",
        "BBBFFBFLRR",
        "BFFBBBBRRL",
        "FBBBBFFRLR",
        "FBFFFBFRRL",
        "FFBFFFFRLL",
        "FFBBFBBLLR",
        "BBFBBBBLLR",
        "BFBFBFFRLR",
        "BBBFFFBRLR",
        "BFFBFBBLLL",
        "BFBFBBFRRL",
        "FFFBFFBRRR",
        "FFFBFBBLLR",
        "FFBBFFBRLR",
        "FFFBBBBRLR",
        "FBBFFBFLRR",
        "BFFBFFBLLL",
        "FBFBBBBRRR",
        "FBFFFBBLRL",
        "BBBFBFFRRL",
        "FBFFBBBLRR",
        "BFBBFFFRLL",
        "BFBFFFFRRL",
        "FBFBFFFRLL",
        "BFFFBFBLLR",
        "FBBFBBBLRL",
        "FBBFFFBLLL",
        "FBFBBBFRRR",
        "FFBFFBFLRR",
        "BBFBFBBRRL",
        "BBFBBBFLRL",
        "BBFFFFFRLL",
        "FFBFFBBRLR",
        "BBFBBBBLLL",
        "BBFBFFFLRL",
        "FBFBBFBRLL",
        "FBFFBBBRRR",
        "FBFBFBFRRL",
        "BFFFFFBRRR",
        "FFBFBBBLLL",
        "FFBBFBFLRL",
        "BFFBBBFRLL",
        "FBBBFFBRRR",
        "BFFBFBFLRL",
        "BBFFFBBLRR",
        "BBFFBFBRRL",
        "FBFBFBFLLR",
        "FFBFBFBRRL",
        "BFFFFBBRLL",
        "BBFFFFBRRL",
        "FFBBFFBRLL",
        "BFFBBFBRLL",
        "FBBBFFBRLL",
        "BFBFFFBLLR",
        "FFFBFBFRRL",
        "BBFFBFBRRR",
        "BFFFFFFLLR",
        "FBBFBFFRRL",
        "BFFBBBBLLL",
        "FFBBFFFLLL",
        "FBBFFBBLLR",
        "FFFFBBFRLL",
        "FBFBFFFRRL",
        "BFFFBBFRRR",
        "BBFBFFBRRL",
        "BFFBFBBRRR",
        "BFBFFBFRLR",
        "BBFBFFBLRR",
        "BFBBFBBRRR",
        "BFBFFFFRLL",
        "BBFFBBBRLL",
        "FFBFFFFLRR",
        "BBBFBFFLLR",
        "FFFBBFBRLR",
        "BFBBFFFLLL",
        "FBBBFBBRLR",
        "FFBFFBFRLR",
        "FBFBBFBLLL",
        "FBBBFBFRRR",
        "FBFFFBBRRR",
        "BFFBFBFRRR",
        "FFBBBBFRRL",
        "BBFBFBBLLL",
        "BBFFBFBLLR",
        "BBBFFFFRRL",
        "FFBFFFFRRR",
        "BBFFBBFRRR",
        "FFBBBFBLRL",
        "FBBFBFBLRL",
        "FBBBFBFRLR",
        "BFFBFBFRLR",
        "BFFFBFBRRR",
        "FFBBFFFRLL",
        "FBFFBFBRLL",
        "FFBBBFFLRR",
        "FBBBFBBRRL",
        "BBFBBBBLRL",
        "BFFFBBBLLR",
        "BFBBBBBLRL",
        "BFFFFBFLRL",
        "BFFFFFFRLL",
        "FFFFBBBRRL",
        "FFBFBBFRRL",
        "BBBFFBFRRL",
        "FBBFBFFLRL",
        "BBBFBFBLRL",
        "FBBFBBFRRL",
        "FBFBFFBLLL",
        "FBBBBBBLLR",
        "BBBFFFBRRL",
        "FFBFBBFLLR",
        "BFBFFBBRRL",
        "FFBFFFBLRR",
        "BBFBFBFRRR",
        "FBBBBFBLRR",
        "FFBBFBFRRR",
        "BFBBFBFLRL",
        "BBFFBFFLLR",
        "FFFFBBBLLR",
        "FFFBBFBLLL",
        "FFBBFFFRRL",
        "BFFBBBFRRL",
        "BFBBBBBLLR",
        "BBBFFFFRRR",
        "FBFFBBBRLR",
        "FBFFBFFRRR",
        "FFFBFBFLRL",
        "FBFFBFFRLL",
        "FBBBFFFLLL",
        "BFFFFFBLRR",
        "BFBBFBFRLR",
        "FBFBBBBRLL",
        "FBFBBBFLRL",
        "FFFBBFBLRR",
        "BBFBBBFRLR",
        "FBFFFFBLLR",
        "BBBFFBFRLL",
        "FBBFBFFRLR",
        "BFFFBBFRRL",
        "BFFFFBBRRR",
        "FBFFBFBLLL",
        "BFBFFFBRRR",
        "BFBFBBFLRR",
        "FBFFBBBLRL",
        "FFBFBBFRLL",
        "FBBBFBFRLL",
        "FFBFFBBRRR",
        "FBBFFBBRLR",
        "BFFBFBBLRL",
        "FFBFBFFRRL",
        "FBBFFFFLRR",
        "BBFBBFFRRR",
        "BFBBFFFRRL",
        "FBBFFFBRLL",
        "FFBBBBFRLR",
        "BBFFBBFRRL",
        "BFBBFBFRRR",
        "BFFBBFFLLR",
        "BBFFBBFLRR",
        "BBFBBFFLLR",
        "BFFFBBFLLL",
        "FBBBFFBRRL",
        "BBFFFFFLRR",
        "BFFFBBBRLR",
        "BFBFBBFLRL",
        "BBFBBFBLRR",
        "FFBFFFFRRL",
        "BFFBBBFLLL",
        "FBFBFBBRLR",
        "FFBBFFFLLR",
        "FFFBFFFRRL",
        "BBFBBFBRRL",
        "BBFFFFFLLR",
        "FFFFBFBRLR",
        "BFFFFBFRLL",
        "BBBFBFFRLR",
        "BFFFFBBLRL",
        "BFFBBFFLRR",
        "FBFFFBBLRR",
        "BFFFBBBLLL",
        "BFFBBBFLLR",
        "BFFBBFBRRL",
        "FBBFBFBRRR",
        "FBFBFFFLLR",
        "FFFBBFFRLR",
        "FFBFBFFLLR",
        "FBBFFBFRRR",
        "FBFFBBFRLR",
        "FBBFFFBRLR",
        "BFBFFFBRRL",
        "BFFFFBFLLR",
        "BBBFFFFLRR",
        "BFBFBFFRLL",
        "FBFBFBBRRL",
        "FFBFBFFLLL",
        "BBFFFFBRRR",
        "FBFBBFFRLL",
        "BBFFFFFLLL",
        "FFFFBBBLRL",
        "BFFFFBBLLL",
        "FFFBFBFLLL",
        "FFFFBFBLLR",
        "FFFBBFBLLR",
        "FFFBBFBLRL",
        "FFFFBFFRRR",
        "BBFFFFBLRL",
        "BFBBBBFLLR",
        "BBFBFBBLLR",
        "BFFFFFFRLR",
        "FBBFFFFRLL",
        "FFFBFFBLLL",
        "BBBFFBBRRR",
        "BBFBFBBLRR",
        "FBBBBFFLRL",
        "FFBBBFFLLR",
        "FBBBBFFRLL",
        "BBFBFBFLRR",
        "BFFBFBBLLR",
        "BBBFFFFRLL",
        "FBFBFFBRRR",
        "FBFBFFBLLR",
        "FFFBBFBRRL",
        "FBBBBBBRLR",
        "FBFFFFBRRL",
        "FFBBFBBRRL",
        "BBBFFFBRLL",
        "FFFBFFBLLR",
        "BFBFBBFLLL",
        "BFFBFBFLLR",
        "BBFFBBFRLL",
        "BFFFBBFLLR",
        "BFBBFBFLRR",
        "FBBFFBFLLR",
        "BBFBBFFRLL",
        "FFBBBBBRRR",
        "FFFBBFFLLR",
        "BBBFFBFLLR",
        "BBFBFFBLLL",
        "FFBBFBBRLR",
        "FBBBBBFRLL",
        "BBFBFFBRLR",
        "BBFFBFBRLL",
        "BFFBFFBLLR",
        "FBFFBFFLRR",
        "FBFFFBFLRL",
        "BFBBFFFLLR",
        "FBBFBFFRRR",
        "FBFBFBBLRR",
        "BFBBBBFRRR",
        "BFBFFFFLRL",
        "FFFBFFFLLL",
        "FFFBBBFRLL",
        "FFFBBBFRRR",
        "FBBBBBFLRL",
        "BBFFFFBLRR",
        "BFFBBBFRLR",
        "BFFBFFBRLR",
        "FBBBBFBRLR",
        "FFBFBBBLRR",
        "BBFBFFFRRL",
        "BFFBFBBLRR",
        "BFFBBFFLRL",
        "BFBFFFFRRR",
        "FBBBFBBLRL",
        "BFBBBFFRRL",
        "FBFBBBFLLL",
        "BFFBBBBRLL",
        "FBBFFFBLLR",
        "BBBFBFFLRR",
        "FBFFBBFLRR",
        "BFBBFFBLLR",
        "BFFBBFFRLR",
        "BBFFBBBRRL",
        "BFFBBFBLLR",
        "FBBBBBFLLL",
        "BBFFFBFLLR",
        "BFBBBFFLRL",
        "FFBBFBBLRL",
        "BBFBFBFLLR",
        "BBFFFBFRLL",
        "BFFFFFBRRL",
        "BBBFBFBRLR",
        "FFFBBBBRRR",
        "FBFBBFFRRR",
        "BFFBFFFRRL",
        "BFFFBFFRLR",
        "BFFFBFBLRR",
        "BBFBBBFLLL",
        "BFBFFFFLLL",
        "BBFBBFBLLR",
        "BFFFBBFRLL",
        "FFFBFFBRLL",
        "FBFFFFBLLL",
        "BFBFFFFLLR",
        "BFBFFBBLLR",
        "FBFFBBFRRR",
        "FBFBBBFRRL",
        "BFFBBFBRLR",
        "BBFFFFBLLL",
        "BFBBBFFRLL",
        "BBBFFBBRLL",
        "FBFFFFBRLR",
        "BFBFFBBLLL",
        "FBFBFBBRLL",
        "FFBFFFBRLL",
        "FFFBFFBRLR",
        "FBFFFBFLLL",
        "FFFFBFFRLR",
        "BBFBBFFLLL",
        "FFFFBFBLRR",
        "BFBFFFBLRL",
        "FBBBBFFRRR",
        "BBBFFFBRRR",
        "FBBFBFFRLL",
        "FFBFFBFRRL",
        "BBFBBFBRLR",
        "FFBFBFFRLR",
        "BFBFBFBRRL",
        "FFFBFBBRLR",
        "BFBBBFFRRR",
        "FBFFFFBLRR",
        "FFBBBFBLLR",
        "FBBBFFBLRL",
        "FFFBBFFRLL",
        "FBBFFBBLRL",
        "BFFBBFBLRL",
        "FBBBBFFLRR",
        "BFFFFFFLRR",
        "FBFBBBBLRL",
        "FFBBFFFRLR",
        "BFFFFBFRRL",
        "BBBFBFBRRR",
        "FFFBBFBRRR",
        "FFFFBBBRLR",
        "FFFFBBFLRL",
        "FBBFBBFRLL",
        "BBBFBFFRLL",
        "BFBFFFFRLR",
        "BFBFBFBRRR",
        "FBFFFFFLLR",
        "BFFBFFFLRR",
        "FFBFBFBRLR",
        "BFBBBFBLLR",
        "BBFBFBFRLL",
        "BFFFFFFRRR",
        "BBBFBFBRRL",
        "FBFBBFFLRR",
        "BFBBFFBRRL",
        "BFBFFFBRLL",
        "BFBFFBFLLL",
        "BBBFBFBLRR",
        "BBFBBBBRLL",
        "FFBFFBBLLL",
        "BBFBBFBLRL",
        "FBFBBFFLLL",
        "FBBFFBFLRL",
        "BBFFFBBRRR",
        "FFBBFBFRLL",
        "BFFFFFBRLR",
        "FFBFFBFLLL",
        "BBFFBBFLLL",
        "BFFBFFBLRR",
        "FFBBFFBLLL",
        "FFFBFBFLLR",
        "FBFFBBBRRL",
        "FBFFFBFLRR",
        "BBFFFFFRRR",
        "FFBFBFBRRR",
        "FBBBFBBRRR",
        "FBFBFFBRLL",
        "FBBBBFBRRL",
        "BBFFFFBRLR",
        "FFBFBFFLRR",
        "FFBFFFBLLR",
        "BBBFFBBLRR",
        "BFFFFFBRLL",
        "BFFFFBFRRR",
        "FFBBBBBLRL",
        "FFFFBBBLRR",
        "FBBFBFBLRR",
        "FFBFFBFLLR",
        "FBBFBFBRLL",
        "BFBBFBBLRR",
        "BFBBBFBLRL",
        "BBFBBFFRRL",
        "BFBFBFBLRL",
        "FBFFFFBLRL",
        "FBFBBFBLLR",
        "FFFFBFFRRL",
        "FFFBBFFRRR",
        "FBFFFFFRLL",
        "FBFBBFBLRR",
        "FFBFFFBLLL",
        "BFFFFFFRRL",
        "FBBFBBBRRL",
        "BFFFFFBLLL",
        "FFBBFFBRRL",
        "FBFFFBBRLR",
        "BFBFBFFLRR",
        "FBBBBFFRRL",
        "FBBFFFBLRR",
        "FBFFFBFRRR",
        "FBBBFBFLLR",
        "FFFBBBBRRL",
        "FBBBBFFLLR",
        "BFBFFBBLRL",
        "FBFBFBBRRR",
        "BBFBFFFLLL",
        "FFBBFFFLRR",
        "BFFFFBBRRL",
        "FBBBBBBRRL",
        "BFBFBBFRLR",
        "BFBFBFBLLR",
        "FBBFBFFLLR",
        "BFFFFFFLLL",
        "BBBFBFBLLL",
        "BFBBFFFRLR",
        "FBFFBBBRLL",
        "BFFBFFBRLL",
        "BFFBBFFRLL",
        "FFBBBFFLRL",
        "FBBFBBBLLL",
        "FFFFBFBRRL",
        "BFBFBBBRRL",
        "BFFBBBBLRR",
        "BBFBFBBLRL",
        "FFBBBBFLRL",
        "BFBBBBFRRL",
        "BFFFBFBRLR",
        "FFFBBBFLLR",
        "FBFBFFBLRL",
        "FFBBBFFRRL",
        "FBBBFFBRLR",
        "FFBFFFFLLR",
        "FFBFBFBLRL",
        "BBBFFBBRLR",
        "FBFBBBBLRR",
        "BFBFBFBRLL",
        "FFBBBBBLLL",
        "FBFBFFFLRL",
        "FFFBFFBRRL",
        "FBFFFFFRRR",
        "BFBBBFFRLR",
        "FBFBFBFLRL",
        "BFBFBFFLLR",
        "FBFBBBFRLL",
        "BFBBBBBLRR",
        "FBBFBBFRRR",
        "BFFFBFFRLL",
        "FFFBFBFLRR",
        "FFFBBFFLRR",
        "BFBFBFBRLR",
        "FBBBFFFLLR",
        "FBFFFFBRRR",
        "FBBFFFFLLR",
        "BFFBFFFRLL",
        "FBBBFBBRLL",
        "FBFBFBBLLL",
        "BFFBFBFLLL",
        "FFFBBBFRRL",
        "FFFFBBFRRL",
        "FBFBBFFLLR",
        "BBFBFFBRRR",
        "BFBBFFBLRL",
        "FFFFBFBLLL",
        "BFBFFBFLLR",
        "BFFBBFBLLL",
        "FBBBBBBRRR",
        "FBBFFFBRRL",
        "BFFFBBBRRL",
        "BBFBBBBRRR",
        "BBFFFBFRRL",
        "BBFBFFFLLR",
        "BFFFBFFRRR",
        "BFBFBBBLLL",
        "BBFFFBFRRR",
        "BBFFBFBRLR",
        "FBFBFBBLRL",
        "BFBBFFBLRR",
        "BBFBFFBRLL",
        "BFFBBBBRLR",
        "FBBFBBFRLR",
        "FFBFBBBLLR",
        "BFBFFBBRRR",
        "FBBBFFFLRL",
        "BBBFFFFLRL",
        "BFBBBBFRLR",
        "BFFFBFBRRL",
        "FBFFFFFRRL",
        "BBFFFFFRRL",
        "FBFFBBFLRL",
        "FBBFBBBRRR",
        "BFBBBBBRLR",
        "BBFFBFFRLR",
        "FBBFFBBRRR",
        "BBBFFFBLLL",
        "FFBBBFFRLL",
        "BBFFFBBRLR",
        "FFBBBBFLLR",
        "FFBBBFBRLL",
        "BFFFBBBLRR",
        "FFFBBBBLRR",
        "FFBFBBBRRR",
        "FFBFBBBRRL",
        "BBFBBBFLRR",
        "FBBFBBBLRR",
        "FBFBFFBRLR",
        "FBFFBBFRLL",
        "BBFFFBFLRL",
        "FBBFBFBRLR",
        "BFBBBBFLLL",
        "FBBFBBFLRR",
        "FBFFFBBRRL",
        "BBFFBFFLRL",
        "BFFBBBFRRR",
        "BFFBBFBLRR",
        "FBFFFBBLLR",
        "BFFFBFBRLL",
        "FFBFBFBRLL",
        "BBFBBFBRLL",
        "BFFBFFBRRL",
        "FFFFBBFLLR",
        "BFBFBBBRLL",
        "FFFFBBBRRR",
        "FFBBBFBRRL",
        "BFFBFBBRRL",
        "BFBFBFBLLL",
        "FFBFFFBRLR",
        "FFBBFBFRLR",
        "FBBBFFBLRR",
        "BBFFBFFRRR",
        "BFFFBFFLRL",
        "BFBBFBFLLR",
        "BBFBFFBLRL",
        "FBBBFBBLLL",
        "FFBBBBBRLR",
        "BFBBFBBRLR",
        "FBBBBFBRLL",
        "FFBBBBFLRR",
        "BFFFFFBLRL",
        "BBFBFBBRRR",
    ];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT).unwrap(), 820);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run(&PUZZLE_INPUT).unwrap(), 944);
    }
}
