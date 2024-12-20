use core::panic;

use aoc_2024::{log_output, parse_string};

fn run_program(
    mut register_a: u32,
    mut register_b: u32,
    mut register_c: u32,
    program: &Vec<u32>,
) -> Option<Vec<u32>> {
    let mut output = vec![];
    let mut pointer = 0;

    while pointer < program.len() {
        let opcode = program[pointer];
        let literal_operand = program[pointer + 1];

        let operand = match literal_operand {
            0..=3 | 7 => literal_operand,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => panic!("Invalid operand"),
        };

        match opcode {
            0 => register_a /= 1 << operand,
            1 => register_b ^= literal_operand,
            2 => register_b = operand % 8,
            3 => {
                if register_a != 0 {
                    pointer = literal_operand as usize;
                    continue;
                }
            }
            4 => register_b ^= register_c,
            5 => output.push(operand % 8),
            6 => register_b = register_a / (1 << operand),
            7 => register_c = register_a / (1 << operand),
            _ => panic!("Invalid opcode"),
        };

        pointer += 2;
    }

    Some(output)
}

fn part1(puzzle: &str) -> String {
    let puzzle: Vec<Vec<u32>> = parse_string(puzzle, vec![',']);
    let register_a = puzzle[0][0];
    let register_b = puzzle[1][0];
    let register_c = puzzle[2][0];
    let program = puzzle[3].clone();

    let output = run_program(register_a, register_b, register_c, &program).unwrap();

    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug)]
struct Input {
    opr: Vec<i32>,
}

struct ProgramState {
    r_a: i32,
    r_b: i32,
    r_c: i32,
    p: usize,
    oprs: Vec<i32>,
}

impl ProgramState {
    fn new(opr: Vec<i32>, a: i32) -> Self {
        ProgramState {
            r_a: a,
            r_b: 0,
            r_c: 0,
            p: 0,
            oprs: opr,
        }
    }

    fn dv(&self, opand: i32) -> i32 {
        2_i32.pow(self.get_combo(opand) as u32)
    }

    fn get_combo(&self, opand: i32) -> i32 {
        match opand {
            4 => self.r_a,
            5 => self.r_b,
            6 => self.r_c,
            _ => opand,
        }
    }

    // Operation handlers
    fn handle_adv(&mut self, opand: i32) {
        self.r_a /= self.dv(opand);
    }

    fn handle_bxl(&mut self, opand: i32) {
        self.r_b ^= opand;
    }

    fn handle_bst(&mut self, opand: i32) {
        self.r_b = self.get_combo(opand) & 7;
    }

    fn handle_jnz(&mut self, opand: i32) -> bool {
        if self.r_a != 0 {
            self.p = opand as usize;
            true
        } else {
            false
        }
    }

    fn handle_bxc(&mut self) {
        self.r_b ^= self.r_c;
    }

    fn handle_out(&self, opand: i32) -> i32 {
        self.get_combo(opand) & 7
    }

    fn handle_bdv(&mut self, opand: i32) {
        self.r_b = self.r_a / self.dv(opand);
    }

    fn handle_cdv(&mut self, opand: i32) {
        self.r_c = self.r_a / self.dv(opand);
    }
}

fn run(input: Input) -> i64 {
    let mut res: i64 = 0;
    let mut len = input.opr.len() - 1;

    loop {
        res *= 8;
        let curr_target: String = input.opr[len..]
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        loop {
            let curr = try_values(&input.opr, res as i32);
            if curr == curr_target {
                break;
            }
            res += 1;
        }

        if len == 0 {
            break;
        }
        len -= 1;
    }
    res
}

fn try_values(opr: &[i32], a: i32) -> String {
    let mut state = ProgramState::new(opr.to_vec(), a);
    let mut res = Vec::new();

    while state.p < state.oprs.len() - 1 {
        let opcode = state.oprs[state.p];
        let opand = state.oprs[state.p + 1];

        let mut increment_p = true;

        match opcode {
            0 => state.handle_adv(opand),
            1 => state.handle_bxl(opand),
            2 => state.handle_bst(opand),
            3 => increment_p = !state.handle_jnz(opand),
            4 => state.handle_bxc(),
            5 => res.push(state.handle_out(opand)),
            6 => state.handle_bdv(opand),
            7 => state.handle_cdv(opand),
            _ => panic!("Invalid opcode"),
        }

        if increment_p {
            state.p += 2;
        }

        if state.p >= state.oprs.len() - 1 {
            break;
        }
    }

    // Convert result to string
    res.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(puzzle: &str) -> i64 {
    let puzzle: Vec<Vec<i32>> = parse_string(puzzle, vec![',']);

    run(Input {
        opr: puzzle[3].clone(),
    })
}

fn main() {
    log_output(1, || part1(include_str!("data.txt")));
    log_output(2, || part2(include_str!("data.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("729\n0\n0\n0,1,5,4,3,0"), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2024\n0\n0\n0,3,5,4,3,0"), 117440);
    }
}
