#![feature(iter_array_chunks)]

mod input;

const SCORES: &'static str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn calc(input_data: &'static str) -> usize {
    input_data
        .lines()
        .map(|line| {
            let split = line.split_at(line.len() / 2);

            split
                .0
                .chars()
                .into_iter()
                .map(|char| match split.1.find(char) {
                    Some(c) => SCORES.find(split.1.chars().nth(c).unwrap()).unwrap(),
                    None => 0,
                })
                .max()
                .unwrap()
        })
        .sum()
}

fn calc_p2(input_data: &'static str) -> usize {
    input_data
        .lines()
        .array_chunks::<3>()
        .map(|chunks| {
            chunks[0]
                .chars()
                .into_iter()
                .map(|char| match chunks[1].find(char) {
                    Some(char_index) => {
                        match chunks[2].find(chunks[1].chars().nth(char_index).unwrap()) {
                            Some(c) => SCORES.find(chunks[2].chars().nth(c).unwrap()).unwrap(),
                            _ => 0,
                        }
                    }
                    _ => 0,
                })
                .max()
                .unwrap()
        })
        .sum()
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data));
    println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
            ),
            157
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
            ),
            70
        )
    }
}
