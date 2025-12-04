use std::collections::HashMap;
use std::fs;

/// Simple grid wrapper that lets us address cells via cartesian coordinates.
#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    cells: HashMap<(i32, i32), char>,
}

impl Grid {
    fn from_str(raw: &str) -> Self {
        let mut cells = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in raw.lines().enumerate() {
            let y = y as i32;
            if width == 0 {
                width = line.len() as i32;
            }
            height = y + 1;

            for (x, value) in line.chars().enumerate() {
                cells.insert((x as i32, y), value);
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        self.cells.get(&(x, y)).copied()
    }

    fn get_adjacent_values(&self, x: i32, y: i32) -> Vec<char> {
        let mut values = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                if let Some(val) = self.get(x + dx, y + dy) {
                    values.push(val);
                }
            }
        }
        values
    }
    fn number_adjacent_rolls(&self, x: i32, y: i32) -> i32 {
        let adjacent_values = self.get_adjacent_values(x, y);
        let mut number_rolls = 0;
        for val in adjacent_values {
            if val == '@' {
                number_rolls += 1;
            }
        }
        number_rolls
    }
}

fn main() {
    let raw = fs::read_to_string("2025/day4/data.txt").unwrap();
    let grid = Grid::from_str(&raw);

    let mut count_accessible = 0;

    for x in 0..(grid.width as i32) {
        for y in 0..(grid.height as i32) {
            if grid.get(x, y) == Some('@') && grid.number_adjacent_rolls(x, y) < 4 {
                    count_accessible += 1;
            }
        }
    }
    println!("Count of accessible rolls: {count_accessible}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
    }

    #[test]
    fn test_number_adjacent_rolls_some_positions() {
        let grid = Grid::from_str(sample_input());

        // At (0,0), adjacents are: (0,1),(1,0),(1,1)
        assert_eq!(grid.number_adjacent_rolls(0, 0), 2); // Only (1,0) and (1,1) are '@'

        // At (2,0): Should have 3 adjacent rolls at (1,1), (1,2) and (3,0)
        assert_eq!(grid.number_adjacent_rolls(2, 0), 3);

        // At (1,1): Surrounded by more rolls
        assert_eq!(grid.number_adjacent_rolls(1, 1), 6);
    }

    #[test]
    fn test_accessible_rolls_count_matches_spec() {
        let grid = Grid::from_str(sample_input());
        let mut count_accessible = 0;

        for x in 0..(grid.width as i32) {
            for y in 0..(grid.height as i32) {
                if grid.get(x, y) == Some('@') && grid.number_adjacent_rolls(x, y) < 4 {
                        count_accessible += 1;
                }
            }
        }
        assert_eq!(count_accessible, 13, "Should find exactly 13 accessible rolls");
    }

    #[test]
    fn test_accessible_rolls_positions() {
        let grid = Grid::from_str(sample_input());
        // Positions of accessible rolls '@' with <4 adjacent '@', from spec
        let accessible_pos = [
            (2, 0), (3, 0), (5, 0), (6, 0), (8, 0),
            (0, 1),
            (6, 2),
            (0, 4), (9, 4),
            (0, 7),
            (0, 9), (2, 9), (8, 9),
        ];
        // Some positions from above (by spec) - let's just test a few
        for &(x, y) in accessible_pos.iter() {
            assert!(grid.number_adjacent_rolls(x, y) < 4, "Roll at ({x},{y}) should be accessible");
        }
    }
}
