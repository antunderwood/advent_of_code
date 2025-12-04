use std::collections::HashMap;
use std::fs;

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

    fn set(&mut self, x: i32, y: i32, value: char) {
        self.cells.insert((x, y), value);
    }

    fn get_adjacent_values(&self, x: i32, y: i32) -> Vec<char> {
        let mut values = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
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

    fn accessible_rolls(&self) -> Vec<(i32, i32)> {
        let mut rolls = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) == Some('@') && self.number_adjacent_rolls(x, y) < 4 {
                    rolls.push((x, y));
                }
            }
        }
        rolls
    }

    fn remove_accessible_round(&mut self) -> bool {
        let to_remove = self.accessible_rolls();
        if to_remove.is_empty() {
            return false;
        }
        for (x, y) in to_remove {
            self.set(x, y, '.');
        }
        true
    }

    fn recursively_remove_accessible(&mut self) {
        if self.remove_accessible_round() {
            self.recursively_remove_accessible();
        }
    }
}

fn main() {
    let raw = fs::read_to_string("2025/day4/data.txt").unwrap();
    let mut grid = Grid::from_str(&raw);

    let initial_rolls = raw.chars().filter(|&c| c == '@').count();
    grid.recursively_remove_accessible();

    let remaining_rolls = grid.cells.values().filter(|value| **value == '@').count();

    let removed_rolls = initial_rolls - remaining_rolls;
    println!("Removed rolls: {removed_rolls}");
}
 

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
        assert_eq!(
            grid.accessible_rolls().len(),
            13,
            "Should find exactly 13 accessible rolls"
        );
    }

    #[test]
    fn test_accessible_rolls_positions() {
        let grid = Grid::from_str(sample_input());
        // Positions of accessible rolls '@' with <4 adjacent '@', from spec
        let accessible_pos = [
            (2, 0),
            (3, 0),
            (5, 0),
            (6, 0),
            (8, 0),
            (0, 1),
            (6, 2),
            (0, 4),
            (9, 4),
            (0, 7),
            (0, 9),
            (2, 9),
            (8, 9),
        ];
        let expected: HashSet<_> = accessible_pos.into_iter().collect();
        let actual: HashSet<_> = grid.accessible_rolls().into_iter().collect();
        assert_eq!(expected, actual, "Accessible positions should match spec");
    }

    #[test]
    fn test_part2_recursive_removal_exhausts_accessible_rolls() {
        let mut grid = Grid::from_str(sample_input());
        grid.recursively_remove_accessible();
        assert!(
            grid.accessible_rolls().is_empty(),
            "No accessible rolls should remain after recursive removal"
        );
    }
    #[test]
    fn test_part2_full_removal_count_matches_example() {
        let raw = sample_input();
        let mut grid = Grid::from_str(raw);
        let initial_rolls = raw.chars().filter(|&c| c == '@').count();
        grid.recursively_remove_accessible();
        // Based on the new spec, we should remove 43 rolls in total.
        let remaining_rolls = grid.cells.values().filter(|&&c| c == '@').count();
        let removed_rolls = initial_rolls - remaining_rolls;
        assert_eq!(removed_rolls, 43, "Should remove 43 rolls after full recursive removal");
    }

    #[test]
    fn test_part2_remaining_grid_after_all_removals() {
        let mut grid = Grid::from_str(sample_input());
        grid.recursively_remove_accessible();
        // The remaining grid after all removals
        // The final 10x10 shape should match this (from the example):
        let expected = [
            "..........",
            "..........",
            "..........",
            "....@@....",
            "...@@@@...",
            "...@@@@@..",
            "...@.@.@@.",
            "...@@.@@@.",
            "...@@@@@..",
            "....@@@...",
        ];
        for (y, line) in expected.iter().enumerate() {
            for (x, expected_ch) in line.chars().enumerate() {
                let Some(val) = grid.get(x as i32, y as i32) else {
                    continue;
                };
                assert_eq!(val, expected_ch, "Mismatch at ({x},{y})");
            }
        }
    }
}
