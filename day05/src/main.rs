mod input;

fn calc(input_data: &'static str) -> String {
    let split: Vec<&str> = input_data.split("\n\n").collect();
    let stacks_string = split[0];
    let movements = split[1].lines();
    let line_of_stacks = stacks_string.lines().next_back().unwrap();
    let number_of_stacks = line_of_stacks
        .chars()
        .next_back()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; number_of_stacks];
    let mut current_stack = 0;
    let mut final_result = String::from("");

    line_of_stacks.chars().enumerate().for_each(|char| {
        if char.1.to_string().parse::<usize>().is_ok() {
            stacks_string.lines().rev().for_each(|line| {
                let box_char = line.chars().nth(char.0);

                if box_char.is_some() {
                    let box_char_is_number = line
                        .chars()
                        .nth(char.0)
                        .unwrap()
                        .to_string()
                        .parse::<usize>();

                    if box_char_is_number.is_err() && box_char.unwrap() != ' ' {
                        stacks[current_stack].push(box_char.unwrap().to_string())
                    }
                }
            });

            current_stack += 1;
        }
    });

    movements.for_each(|movement| {
        let mut movement_split = movement.split(" ");
        let number_of_boxes_to_move = movement_split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = movement_split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = movement_split.nth(1).unwrap().parse::<usize>().unwrap();

        for _ in 0..number_of_boxes_to_move {
            let movement_item = stacks[from - 1].pop().unwrap();

            stacks[to - 1].push(movement_item);
        }
    });

    stacks
        .iter()
        .for_each(|stack| final_result.push(stack[stack.len() - 1].chars().last().unwrap()));

    final_result
}

fn calc_p2(input_data: &'static str) -> String {
    let split: Vec<&str> = input_data.split("\n\n").collect();
    let stacks_string = split[0];
    let movements = split[1].lines();
    let line_of_stacks = stacks_string.lines().next_back().unwrap();
    let number_of_stacks = line_of_stacks
        .chars()
        .next_back()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; number_of_stacks];
    let mut current_stack = 0;
    let mut final_result = String::from("");

    line_of_stacks.chars().enumerate().for_each(|char| {
        if char.1.to_string().parse::<usize>().is_ok() {
            stacks_string.lines().rev().for_each(|line| {
                let box_char = line.chars().nth(char.0);

                if box_char.is_some() {
                    let box_char_is_number = line
                        .chars()
                        .nth(char.0)
                        .unwrap()
                        .to_string()
                        .parse::<usize>();

                    if box_char_is_number.is_err() && box_char.unwrap() != ' ' {
                        stacks[current_stack].push(box_char.unwrap().to_string())
                    }
                }
            });

            current_stack += 1;
        }
    });

    movements.for_each(|movement| {
        let mut movement_split = movement.split(" ");
        let number_of_boxes_to_move = movement_split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = movement_split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = movement_split.nth(1).unwrap().parse::<usize>().unwrap();

        let mut movement_item = vec![];
        for _ in 0..number_of_boxes_to_move {
            movement_item.push(stacks[from - 1].pop().unwrap());
        }

        movement_item.iter().rev().for_each(|item| {
            stacks[to - 1].push(item.to_string());
        });
    });

    stacks
        .iter()
        .for_each(|stack| final_result.push(stack[stack.len() - 1].chars().last().unwrap()));

    final_result
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
                "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
            ),
            "CMZ"
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
            ),
            "MCD"
        )
    }
}
