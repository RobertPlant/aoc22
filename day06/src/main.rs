mod input;

use std::collections::{HashSet, VecDeque};

fn calc(input_data: &'static str, concurrent: usize) -> usize {
    let mut current = VecDeque::new();
    let mut i = 0;

    input_data.chars().find(|&char| {
        if current.len() == concurrent {
            current.pop_front();
        }
        current.push_back(char);

        let set: HashSet<_> = current.iter().collect();

        i += 1;

        set.len() == concurrent
    });

    i
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data, 4));
    println!("Found P2 {:?}", calc(input_data, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calc("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(calc("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(calc("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(calc("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(calc("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(calc("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(calc("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(calc("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(calc("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(calc("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
