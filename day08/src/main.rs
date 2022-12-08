mod input;

fn calc(input_data: &'static str) -> usize {
    let grid: Vec<Vec<usize>> = input_data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let height = grid.len() - 1;
    let width = grid[0].len() - 1;
    let mut count = 0;

    grid.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(item_index, item)| {
            let is_side =
                vec![0, height].contains(&line_index) || vec![0, width].contains(&item_index);

            let see_up = line_index > 0 && {
                let mut found_larger = false;

                for i in 0..line_index {
                    if !found_larger {
                        found_larger = grid[i][item_index] >= *item;
                    }
                }

                !found_larger
            };
            let see_down = line_index < height && {
                let mut found_larger = false;

                for i in (line_index + 1)..(height + 1) {
                    if !found_larger {
                        found_larger = grid[i][item_index] >= *item;
                    }
                }

                !found_larger
            };
            let see_left = item_index > 0 && {
                let mut found_larger = false;

                for i in 0..item_index {
                    if !found_larger {
                        found_larger = grid[line_index][i] >= *item;
                    }
                }

                !found_larger
            };
            let see_right = item_index < width && {
                let mut found_larger = false;

                for i in (item_index + 1)..(width + 1) {
                    if !found_larger {
                        found_larger = grid[line_index][i] >= *item;
                    }
                }

                !found_larger
            };

            let visible_middle = see_up || see_down || see_left || see_right;

            if is_side || visible_middle {
                count += 1;
            }
        })
    });

    count
}

fn calc_p2(input_data: &'static str) -> usize {
    let grid: Vec<Vec<usize>> = input_data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let height = grid.len() - 1;
    let width = grid[0].len() - 1;
    let mut scenic_score = vec![];

    grid.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(item_index, item)| {
            let is_side =
                vec![0, height].contains(&line_index) || vec![0, width].contains(&item_index);

            let see_up: usize = if line_index > 0 {
                let mut found_larger = false;
                let mut count = 0;
                let mut i = line_index - 1;

                loop {
                    if !found_larger {
                        count += 1;
                        found_larger = grid[i][item_index] >= *item;
                    }

                    if i == 0 {
                        break;
                    }

                    i -= 1;
                }

                count
            } else {
                1
            };
            let see_down = if line_index < height {
                let mut found_larger = false;
                let mut count = 0;

                for i in (line_index + 1)..(height + 1) {
                    if !found_larger {
                        count += 1;
                        found_larger = grid[i][item_index] >= *item;
                    }
                }

                count
            } else {
                1
            };
            let see_left = if item_index > 0 {
                let mut found_larger = false;
                let mut count = 0;
                let mut i = item_index - 1;

                loop {
                    if !found_larger {
                        count += 1;
                        found_larger = grid[line_index][i] >= *item;
                    }

                    if i == 0 {
                        break;
                    }

                    i -= 1;
                }

                count
            } else {
                1
            };
            let see_right = if item_index < width {
                let mut found_larger = false;
                let mut count = 0;

                for i in (item_index + 1)..(width + 1) {
                    if !found_larger {
                        count += 1;
                        found_larger = grid[line_index][i] >= *item;
                    }
                }

                count
            } else {
                1
            };

            if !is_side {
                scenic_score.push(see_up * see_down * see_left * see_right)
            }
        })
    });

    scenic_score.iter().max().unwrap().clone()
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
                "30373
25512
65332
33549
35390
"
            ),
            21
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "30373
25512
65332
33549
35390
"
            ),
            8
        )
    }
}
