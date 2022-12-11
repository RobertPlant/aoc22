mod input;

#[derive(Debug, Clone)]
struct Monkey {
    operation_method: char,
    operation_value: usize,
    test_value: usize,
    true_monkey: usize,
    false_monkey: usize,
    items: Vec<usize>,
    inspection_count: usize,
}

impl Monkey {
    fn operation(item: usize, method: char, value: usize) -> usize {
        let normalised_value = if value == 0 { item } else { value };
        match method {
            '+' => item + normalised_value,
            _ => item * normalised_value,
        }
    }

    fn test(item: usize, test: usize) -> bool {
        item % test == 0
    }
}

fn parse(input_data: &'static str) -> Vec<Monkey> {
    let mut monkies = Vec::new();
    input_data.split_terminator("\n\n").for_each(|line| {
        let lines: Vec<&str> = line.split("\n").collect();

        let items = lines[1]
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        let operation_split: Vec<String> = lines[2]
            .trim()
            .replace("Operation: new = old ", "")
            .split(" ")
            .map(|item| item.to_string())
            .collect();

        let method = operation_split[0].chars().nth(0).unwrap();
        let value = if operation_split[1].parse::<usize>().is_ok() {
            operation_split[1].parse::<usize>().unwrap()
        } else {
            0
        };

        let test = lines[3]
            .trim()
            .replace("Test: divisible by ", "")
            .parse::<usize>()
            .unwrap();
        let true_monkey = lines[4]
            .trim()
            .replace("If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let false_monkey = lines[5]
            .trim()
            .replace("If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        monkies.push(Monkey {
            operation_method: method,
            operation_value: value,
            test_value: test,
            true_monkey,
            false_monkey,
            items,
            inspection_count: 0,
        });
    });

    monkies
}

fn calc(input_data: &'static str) -> usize {
    let mut monkies = parse(input_data);
    let rounds = 20;

    for _ in 0..rounds {
        monkies
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(monkey_key, monkey)| {
                monkies[monkey_key]
                    .items
                    .clone()
                    .into_iter()
                    .for_each(|item| {
                        let updated_value = Monkey::operation(
                            item,
                            monkey.operation_method,
                            monkey.operation_value,
                        ) / 3;

                        let monkey_target = if Monkey::test(updated_value, monkey.test_value) {
                            monkey.true_monkey
                        } else {
                            monkey.false_monkey
                        };

                        monkies[monkey_key].inspection_count += 1;
                        monkies[monkey_key].items.remove(0);
                        monkies[monkey_target].items.push(updated_value);
                    });
            });
    }

    let mut inspection_count: Vec<usize> = monkies
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();

    inspection_count.sort();

    inspection_count.iter().rev().take(2).product()
}

fn calc_p2(input_data: &'static str) -> usize {
    let mut monkies = parse(input_data);
    let lcm: usize = monkies.iter().map(|monkey| monkey.test_value).product();
    let rounds = 10000;

    for _ in 0..rounds {
        monkies
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(monkey_key, monkey)| {
                monkies[monkey_key]
                    .items
                    .clone()
                    .into_iter()
                    .for_each(|item| {
                        let normalised_value = if monkey.operation_value == 0 {
                            item
                        } else {
                            monkey.operation_value
                        };
                        let updated_value = match monkey.operation_method {
                            '+' => item + normalised_value,
                            _ => item * normalised_value,
                        } % lcm;

                        let test_passes = Monkey::test(updated_value, monkey.test_value);

                        let monkey_target = if test_passes {
                            monkey.true_monkey
                        } else {
                            monkey.false_monkey
                        };

                        monkies[monkey_key].inspection_count += 1;
                        monkies[monkey_key].items.remove(0);
                        monkies[monkey_target].items.push(updated_value);
                    });
            });
    }

    let mut inspection_count: Vec<usize> = monkies
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();

    inspection_count.sort();

    inspection_count.iter().rev().take(2).product()
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
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
            ),
            10605
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
            ),
            2713310158
        )
    }
}
