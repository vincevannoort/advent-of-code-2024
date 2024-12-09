use itertools::Itertools;

advent_of_code::solution!(9);

fn construct_disk(input: &str) -> (Vec<String>, usize) {
    let mut iterator = input
        .chars()
        .filter_map(|c| c.to_string().parse::<u64>().ok())
        .tuples();

    let size = iterator.clone().count();

    let values = iterator
        .by_ref()
        .enumerate()
        .flat_map(|(i, (files, free_space))| {
            [
                vec![i.to_string(); files as usize],
                vec![".".to_string(); free_space as usize],
            ]
            .concat()
        })
        .collect_vec();

    // TODO: find a way we don't need to do this...
    let (a,) = iterator.into_buffer().collect_tuple().unwrap();
    ([values, vec![size.to_string(); a as usize]].concat(), size)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut values, file_groups) = construct_disk(input);

    let last_item = values.len() - 1;

    loop {
        let (free_pos, _) = values.iter().find_position(|c| *c == ".").unwrap();
        let (space_pos, _) = values.iter().rev().find_position(|c| *c != ".").unwrap();
        let space_pos = last_item - space_pos;

        if free_pos > space_pos {
            break;
        }
        values.swap(free_pos, space_pos);
    }

    let result: usize = values
        .into_iter()
        .take_while(|c| c != ".")
        .enumerate()
        .map(|(i, c)| i * (c.parse::<usize>().unwrap()))
        .sum();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
