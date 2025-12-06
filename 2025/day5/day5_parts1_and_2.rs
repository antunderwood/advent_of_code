use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IngredientRange {
    start: i64,
    end: i64,
}

impl IngredientRange {
    fn parse(line: &str) -> Self {
        let trimmed = line.trim();
        let (start, end) = trimmed
            .split_once('-')
            .unwrap_or_else(|| panic!("Invalid range line: {trimmed}"));

        let start = start.parse().expect("Invalid start of range");
        let end = end.parse().expect("Invalid end of range");

        Self { start, end }
    }
}

struct IngredientsData {
    ingredient_ranges: Vec<IngredientRange>,
    available_ingredients: Vec<i64>,
    fresh_ingredients: Option<Vec<i64>>,
}

impl IngredientsData {
    fn parse(input_string: &str) -> Self {
        let (range_block, available_block) = input_string
            .split_once("\n\n")
            .unwrap_or_else(|| panic!("Input must contain a blank line separating the two sections"));

        let ingredient_ranges = range_block
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(IngredientRange::parse)
            .collect::<Vec<IngredientRange>>();

        let available_ingredients = available_block
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().parse().expect("Invalid ingredient id"))
            .collect::<Vec<i64>>();

        IngredientsData {
            ingredient_ranges,
            available_ingredients,
            fresh_ingredients: None,
        }
    }

    fn merge_ranges(&mut self) {
        let mut ranges = self.ingredient_ranges.clone();
        ranges.sort_by_key(|r| r.start);

        let mut merged_ranges: Vec<IngredientRange> = Vec::new();
        for range in ranges {
            if merged_ranges.is_empty() || merged_ranges.last().unwrap().end < range.start {
                merged_ranges.push(range);
            } else {
                let last_range = merged_ranges.last_mut().unwrap();
                last_range.end = last_range.end.max(range.end);
            }
        }
        self.ingredient_ranges = merged_ranges;
    }

    fn check_fresh_ingredients(&mut self) {
        let mut fresh_ingredients: Vec<i64> = Vec::new();
        for ingredient in &self.available_ingredients {
            for range in &self.ingredient_ranges {
                if *ingredient >= range.start && *ingredient <= range.end {
                    fresh_ingredients.push(*ingredient);
                    break; // Once matched, don't check further ranges
                }
            }
        }
        self.fresh_ingredients = Some(fresh_ingredients);
    }

}



fn main() {
    let raw = fs::read_to_string("2025/day5/data.txt")
        .expect("Failed to read 2025/day5/data.txt");
    let mut ingredients_data = IngredientsData::parse(&raw);
    ingredients_data.merge_ranges();
    ingredients_data.check_fresh_ingredients();
    let fresh_count = ingredients_data
        .fresh_ingredients
        .as_ref()
        .map(|fresh| fresh.len())
        .unwrap_or(0);
    println!("Number of fresh ingredients: {fresh_count}");
    let mut sum = 0;
    for range in ingredients_data.ingredient_ranges {
        sum += range.end - range.start + 1;
    }
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example_input() -> &'static str {
        indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "}
    }
    #[test]
    fn test_parse_input_example() {
        let ingredients_data = IngredientsData::parse(example_input());
        assert_eq!(ingredients_data.ingredient_ranges.len(), 4);
        assert_eq!(ingredients_data.ingredient_ranges[0].start, 3);
        assert_eq!(ingredients_data.ingredient_ranges[0].end, 5);
        assert_eq!(ingredients_data.ingredient_ranges[1].start, 10);
        assert_eq!(ingredients_data.ingredient_ranges[1].end, 14);
        assert_eq!(ingredients_data.ingredient_ranges[2].start, 16);
        assert_eq!(ingredients_data.ingredient_ranges[2].end, 20);
        assert_eq!(ingredients_data.ingredient_ranges[3].start, 12);
        assert_eq!(ingredients_data.ingredient_ranges[3].end, 18);
        assert_eq!(ingredients_data.available_ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_check_fresh_ingredients_example() {
        let mut ingredients_data = IngredientsData::parse(example_input());
        ingredients_data.merge_ranges();
        ingredients_data.check_fresh_ingredients();
        assert_eq!(ingredients_data.fresh_ingredients.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_find_fresh_ingredient_ranges_example() {
        let mut ingredients_data = IngredientsData::parse(example_input());
        ingredients_data.merge_ranges();
        let mut sum = 0;
        for range in ingredients_data.ingredient_ranges {
            sum += range.end - range.start + 1;
        }
        println!("Sum: {sum}");
        assert_eq!(sum, 14);
    }

    // test merge_ranges //
    fn parse_range(s: &str) -> (u32, u32) {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .expect("missing start")
            .parse::<u32>()
            .expect("invalid start");
        let end = parts
            .next()
            .expect("missing end")
            .parse::<u32>()
            .expect("invalid end");
        (start, end)
    }

    fn parse_ranges(input: &[&str]) -> Vec<(u32, u32)> {
        input.iter().map(|s| parse_range(s)).collect()
    }

    fn merge_ranges(ranges: &[(u32, u32)]) -> Vec<(u32, u32)> {
        let mut sorted_ranges = ranges.to_vec();
        sorted_ranges.sort_by_key(|r| r.0);
        let mut merged_ranges: Vec<(u32, u32)> = Vec::new();
        for range in sorted_ranges {
            if merged_ranges.is_empty() || merged_ranges.last().unwrap().1 + 1 < range.0 {
                merged_ranges.push((range.0, range.1));
            } else {
                let last_range = merged_ranges.last_mut().unwrap();
                last_range.1 = last_range.1.max(range.1);
            }
        }
        merged_ranges
    }
    #[test]
    fn merge_simple_overlaps_and_disjoint() {
        let input = parse_ranges(&["1-5", "3-10", "20-25", "22-30", "40-45"]);
        let expected = vec![(1, 10), (20, 30), (40, 45)];

        let result = merge_ranges(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn merge_nested_overlapping_and_duplicates() {
        let input = parse_ranges(&[
            "5-15",
            "1-3",
            "2-6",
            "8-12",
            "30-40",
            "35-45",
            "50-60",
            "50-60",
        ]);
        let expected = vec![(1, 15), (30, 45), (50, 60)];

        let result = merge_ranges(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn merge_adjoining_ranges() {
        // Assuming adjacency (end + 1 == next_start) counts as merge.
        //
        // Input: ["1-5", "6-10", "11-20", "25-30", "30-35"]
        // Expected: ["1-20", "25-35"]
        let input = parse_ranges(&["1-5", "6-10", "11-20", "25-30", "30-35"]);
        let expected = vec![(1, 20), (25, 35)];

        let result = merge_ranges(&input);

        assert_eq!(result, expected);
    }


}
