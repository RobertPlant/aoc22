mod input;

fn calc(input_data: &'static str, top: usize) -> usize {
    let mut scores = Vec::new();
    let mut max = 0;

    input_data
        .lines()
        .for_each(|line| match line.parse::<usize>() {
            Ok(value) => max += value,
            Err(_) => {
                scores.push(max);
                max = 0
            }
        });

    scores.push(max);
    scores.sort();
    scores.reverse();

    scores.split_at(top).0.iter().sum()
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data, 1));
    println!("Found P2 {:?}", calc(input_data, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
                1
            ),
            24000
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
                3
            ),
            45000
        )
    }
}
