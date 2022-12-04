mod input;

fn fill_vec(input: &'static str) -> Vec<usize> {
    let parsed: Vec<usize> = input
        .split("-")
        .map(|g| g.parse::<usize>().unwrap())
        .collect();

    let mut i = 0;
    let mut start = vec![];

    loop {
        i += 1;

        if parsed[0] <= i {
            start.push(i)
        }

        if parsed[1] <= i {
            break;
        }
    }

    start
}

fn calc(input_data: &'static str) -> usize {
    input_data
        .lines()
        .filter(|line| {
            let lines: Vec<&str> = line.split(",").collect();
            let group_1 = fill_vec(lines[0]);
            let group_2 = fill_vec(lines[1]);

            group_1
                .iter()
                .filter(|number| group_2.contains(number))
                .count()
                == group_1.len()
                || group_2
                    .iter()
                    .filter(|number| group_1.contains(number))
                    .count()
                    == group_2.len()
        })
        .count()
}

fn calc_p2(input_data: &'static str) -> usize {
    input_data
        .lines()
        .filter(|line| {
            let lines: Vec<&str> = line.split(",").collect();
            let group_1 = fill_vec(lines[0]);
            let group_2 = fill_vec(lines[1]);

            group_1
                .iter()
                .filter(|number| group_2.contains(number))
                .count()
                > 0
                || group_2
                    .iter()
                    .filter(|number| group_1.contains(number))
                    .count()
                    > 0
        })
        .count()
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
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
            ),
            2
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
            ),
            4
        )
    }
}
