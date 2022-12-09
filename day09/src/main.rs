mod input;

use std::collections::HashSet;

fn calc(input_data: &'static str, length: usize) -> isize {
    let mut tails = vec![(500, 500); 10];
    let mut tail_history: HashSet<(isize, isize)> = HashSet::new();
    tail_history.insert(tails[0]);

    input_data.lines().for_each(|line| {
        let mut split = line.split(" ");
        let direction = split.nth(0).unwrap();
        let distance = split.nth(0).unwrap().parse::<isize>().unwrap();

        for _ in 0..distance {
            match direction {
                "U" => tails[0] = (tails[0].0 + 1, tails[0].1),
                "D" => tails[0] = (tails[0].0 - 1, tails[0].1),
                "R" => tails[0] = (tails[0].0, tails[0].1 + 1),
                "L" => tails[0] = (tails[0].0, tails[0].1 - 1),
                _ => {}
            }

            for i in 1..=(length) {
                let previous = tails[i - 1];
                let tail = tails[i];
                let v_dist: isize = previous.0 - tail.0;
                let h_dist: isize = previous.1 - tail.1;
                let is_diag = v_dist.abs() > 0 && h_dist.abs() > 0;

                if is_diag {
                    let is_beside = v_dist.abs() == 1 && h_dist.abs() == 1;
                    if !is_beside {
                        if v_dist.abs() == 1 {
                            tails[i] = (tail.0 + (v_dist), tail.1 + (h_dist / 2));
                        } else if h_dist.abs() == 1 {
                            tails[i] = (tail.0 + (v_dist / 2), tail.1 + (h_dist));
                        } else {
                            tails[i] = (tail.0 + (v_dist / 2), tail.1 + (h_dist / 2));
                        }
                    }
                } else if v_dist > 1 || v_dist < -1 {
                    tails[i] = (tail.0 + (v_dist / 2), tail.1);
                } else if h_dist > 1 || h_dist < -1 {
                    tails[i] = (tail.0, tail.1 + (h_dist / 2));
                }

                if i == length {
                    tail_history.insert(tails[i]);
                }
            }
        }
    });

    tail_history.len().try_into().unwrap()
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data, 1));
    println!("Found P2 {:?}", calc(input_data, 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
                1
            ),
            13
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
                9
            ),
            36
        )
    }
}
