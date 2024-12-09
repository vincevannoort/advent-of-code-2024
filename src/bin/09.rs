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

fn get_file_groups(values: &[String], size: usize) -> Vec<(String, usize, usize, usize)> {
    let last_item = values.len() - 1;

    (0..=size)
        .map(|i| {
            let start_position = values
                .iter()
                .find_position(|c| **c == i.to_string())
                .unwrap()
                .0;
            let end_position = last_item
                - values
                    .iter()
                    .rev()
                    .find_position(|c| **c == i.to_string())
                    .unwrap()
                    .0;
            (
                i.to_string(),
                start_position,
                end_position,
                (end_position - start_position) + 1,
            )
        })
        .collect_vec()
}

fn display_disk(values: &[String]) {
    values.iter().for_each(|v| print!("{},", v));
    println!();
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut values, file_groups) = construct_disk(input);
    let groups = get_file_groups(&values, file_groups);

    // loop over groups starting from the back
    for (group_value, group_start, group_end, group_size) in groups.iter().rev() {
        // find free fitting space
        let free_fitting_space = values
            .iter()
            .tuple_windows()
            .enumerate()
            .filter(|(i, (a, b))| *a != "." && *b == ".")
            .find(|(free_space_start, _)| {
                let free_space_lenght = values
                    .iter()
                    .skip(*free_space_start + 1)
                    .take_while(|c| *c == ".")
                    .count();

                *group_size <= free_space_lenght
            })
            // if we found something, it means we found a (N, .) combination, and thus we need to add 1 to the index
            .map(|(free_space_start, _)| free_space_start + 1);

        // continue if we did not find one
        let Some(free_space_start) = free_fitting_space else {
            continue;
        };

        // continue if the free space occurs later than the group itself
        if free_space_start > *group_start {
            continue;
        }

        let file_group = *group_start..=*group_end;
        file_group.into_iter().enumerate().for_each(|(i, file)| {
            values.swap(free_space_start + i, file);
        });
    }

    let result: usize = values
        .into_iter()
        .enumerate()
        .filter(|(i, c)| c != ".")
        .map(|(i, c)| i * (c.parse::<usize>().unwrap()))
        .sum();

    Some(result as u64)
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

    #[test]
    fn test_part_two_test1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3317));
    }

    #[test]
    fn test_part_two_test2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_test3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_test4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(813));
    }

    #[test]
    fn test_part_two_test5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(4));
    }
}
