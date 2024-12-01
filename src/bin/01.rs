advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(first, second)| {
            (
                first.trim().parse::<u32>().unwrap(),
                second.trim().parse::<u32>().unwrap(),
            )
        })
        .collect();

    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(right_list)
        .map(|(left_item, right_item)| left_item.abs_diff(right_item))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
