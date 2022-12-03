mod input;

fn calc(input_data: &'static str) -> usize {
    input_data
        .lines()
        .map(|line| {
            let split = line.split_once(' ').unwrap();

            (match split.1 {
                "X" => 1, // rock
                "Y" => 2, // paper
                "Z" => 3, // scissors
                &_ => 0,
            }) + (match split.0 {
                "A" => match split.1 {
                    "Y" => 6,
                    "X" => 3,
                    &_ => 0,
                }, // rock
                "B" => match split.1 {
                    "Z" => 6,
                    "Y" => 3,
                    &_ => 0,
                }, // paper
                "C" => match split.1 {
                    "X" => 6,
                    "Z" => 3,
                    &_ => 0,
                }, // scissors
                &_ => 0,
            })
        })
        .sum()
}

fn calc_p2(input_data: &'static str) -> usize {
    input_data
        .lines()
        .map(|line| {
            let split = line.split_once(' ').unwrap();

            match split.1 {
                "Y" => {
                    (match split.0 {
                        "A" => 1, // rock
                        "B" => 2, // paper
                        "C" => 3, // scissors
                        &_ => 0,
                    }) + 3
                } // draw
                "Z" => {
                    (match split.0 {
                        "A" => 2, // rock
                        "B" => 3, // paper
                        "C" => 1, // scissors
                        &_ => 0,
                    }) + 6
                } // win
                "X" => match split.0 {
                    "A" => 3, // rock
                    "B" => 1, // paper
                    "C" => 2, // scissors
                    &_ => 0,
                }, // lose
                &_ => 0,
            }
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
                "A Y
B X
C Z
"
            ),
            15
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "A Y
B X
C Z
"
            ),
            12
        )
    }
}
