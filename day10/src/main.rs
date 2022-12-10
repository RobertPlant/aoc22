mod input;

enum Function {
    NOOP,
    ADDX,
}

fn update_sprite(sprites: &Vec<char>, cpu: &Vec<i32>) -> char {
    let last_value = cpu.last().unwrap().clone();
    let length = ((sprites.len() % 40) + 1) as i32;

    if vec![last_value, last_value + 1, last_value + 2].contains(&length) {
        '#'
    } else {
        '.'
    }
}

fn run(instructions: &Vec<(Function, i32)>) -> (Vec<i32>, String) {
    let mut cpu = Vec::new();
    cpu.push(1 as i32);
    let mut sprites_vec = Vec::new();

    for i in 0..instructions.len() {
        let (instruction, parameter) = &instructions[i];

        match instruction {
            Function::NOOP => {
                let last_value = cpu.last().unwrap().clone();

                sprites_vec.push(update_sprite(&sprites_vec, &cpu));
                cpu.push(last_value);
            }
            Function::ADDX => {
                let last_value = cpu.last().unwrap().clone();

                sprites_vec.push(update_sprite(&sprites_vec, &cpu));
                cpu.push(last_value);

                sprites_vec.push(update_sprite(&sprites_vec, &cpu));
                cpu.push(last_value + parameter);
            }
        }
    }

    let lines: Vec<&[char]> = sprites_vec.chunks(40).collect();

    let mut sprites = String::new();
    for line in lines {
        sprites += &line.iter().collect::<String>();
        sprites += &String::from("\n");
    }

    (cpu, sprites)
}

fn calc(input_data: &'static str) -> (i32, String) {
    let instructions = input_data
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();

            (
                match parts[0] {
                    "addx" => Function::ADDX,
                    _ => Function::NOOP,
                },
                if parts.len() > 1 {
                    parts[1].parse::<i32>().unwrap()
                } else {
                    0
                },
            )
        })
        .collect();

    let (cpu, sprites) = run(&instructions);

    let mut signal_strengths = 0;
    for cycle in vec![20, 60, 100, 140, 180, 220] {
        let signal_strength = cycle as i32 * cpu[cycle - 1];

        signal_strengths += signal_strength;
    }

    (signal_strengths, sprites)
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data).0);
    let p2 = calc(input_data).1;
    println!("Found P2\n");
    p2.lines().for_each(|line| println!("{:?}", line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
            )
            .0,
            13140
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
            )
            .1,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        )
    }
}
