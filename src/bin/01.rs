use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(first, second)| {
            (
                first.trim().parse::<u32>().unwrap(),
                second.trim().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(right_list)
        .map(|(left_item, right_item)| left_item.abs_diff(right_item))
        .sum();

    Some(result)
}

fn count_frequencies(input: Vec<u32>) -> HashMap<u32, u32> {
    // https://stackoverflow.com/questions/70234024/is-there-a-rust-function-which-counts-frequencies-in-a-vec
    input.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(*val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_input(input);

    let left_hashmap = count_frequencies(left_list);
    let right_hashmap = count_frequencies(right_list);

    let result = left_hashmap
        .iter()
        .filter_map(|(left_key, left_frequency)| {
            let right_frequency = right_hashmap.get(left_key)?;
            let similarity_score = left_key * right_frequency;
            Some(left_frequency * similarity_score)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
