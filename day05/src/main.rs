use std::error::Error;

use input_parsing::split_lines;
use regex::Regex;


mod input_parsing;

fn main() {
        
    // [N]         [C]     [Z]            
    // [Q] [G]     [V]     [S]         [V]
    // [L] [C]     [M]     [T]     [W] [L]
    // [S] [H]     [L]     [C] [D] [H] [S]
    // [C] [V] [F] [D]     [D] [B] [Q] [F]
    // [Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
    // [P] [P] [C] [W] [W] [F] [W] [J] [C]
    // [T] [L] [D] [G] [P] [P] [V] [N] [R]
    //  1   2   3   4   5   6   7   8   9 

    let mut stacks: Vec<Vec<char>> = vec![
        vec!['T','P','Z','C','S','L','Q','N'],
        vec!['L','P','T','V','H','C','G'],
        vec!['D','C','Z','F'],
        vec!['G','W','T','D','L','M','V','C'],
        vec!['P','W','C'],
        vec!['P','F','J','D','C','T','S','Z'],
        vec!['V','W','G','B','D'],
        vec!['N','J','S','Q','H','W'],
        vec!['R','C','Q','F','S','L','V']
    ];

    let directions: &str = 
    "move 6 from 2 to 1
    move 4 from 6 to 3
    move 1 from 6 to 5
    move 8 from 3 to 8
    move 13 from 8 to 2
    move 2 from 7 to 6
    move 10 from 1 to 6
    move 3 from 2 to 8
    move 5 from 4 to 2
    move 15 from 6 to 5
    move 1 from 1 to 4
    move 2 from 7 to 3
    move 2 from 4 to 2
    move 12 from 5 to 1
    move 4 from 8 to 9
    move 15 from 1 to 3
    move 10 from 9 to 7
    move 1 from 5 to 1
    move 1 from 4 to 8
    move 3 from 7 to 6
    move 8 from 2 to 6
    move 1 from 9 to 8
    move 5 from 2 to 3
    move 1 from 4 to 1
    move 16 from 3 to 1
    move 2 from 2 to 7
    move 13 from 1 to 6
    move 1 from 2 to 4
    move 2 from 2 to 9
    move 1 from 4 to 7
    move 2 from 8 to 2
    move 2 from 2 to 9
    move 1 from 6 to 8
    move 2 from 3 to 8
    move 2 from 1 to 9
    move 1 from 3 to 9
    move 1 from 3 to 2
    move 5 from 5 to 1
    move 2 from 9 to 3
    move 1 from 2 to 3
    move 2 from 1 to 3
    move 3 from 3 to 2
    move 1 from 5 to 7
    move 2 from 7 to 6
    move 2 from 8 to 3
    move 1 from 8 to 9
    move 6 from 3 to 4
    move 3 from 9 to 6
    move 8 from 6 to 4
    move 1 from 2 to 3
    move 1 from 2 to 6
    move 1 from 2 to 9
    move 1 from 3 to 9
    move 5 from 9 to 5
    move 7 from 7 to 4
    move 14 from 4 to 6
    move 1 from 5 to 3
    move 5 from 1 to 9
    move 4 from 5 to 4
    move 1 from 1 to 7
    move 1 from 3 to 8
    move 1 from 8 to 4
    move 4 from 9 to 7
    move 6 from 6 to 5
    move 10 from 4 to 6
    move 1 from 9 to 6
    move 1 from 4 to 3
    move 1 from 3 to 6
    move 1 from 4 to 2
    move 35 from 6 to 3
    move 1 from 2 to 3
    move 4 from 5 to 8
    move 2 from 5 to 4
    move 3 from 8 to 2
    move 2 from 4 to 8
    move 26 from 3 to 8
    move 3 from 2 to 9
    move 6 from 3 to 5
    move 3 from 5 to 7
    move 3 from 7 to 4
    move 3 from 4 to 5
    move 1 from 9 to 5
    move 6 from 5 to 1
    move 2 from 8 to 6
    move 11 from 8 to 5
    move 9 from 5 to 4
    move 1 from 9 to 7
    move 2 from 7 to 9
    move 3 from 1 to 4
    move 1 from 5 to 7
    move 8 from 6 to 1
    move 5 from 7 to 9
    move 7 from 9 to 2
    move 3 from 2 to 9
    move 3 from 7 to 1
    move 4 from 9 to 8
    move 2 from 5 to 6
    move 2 from 2 to 8
    move 2 from 6 to 9
    move 13 from 8 to 1
    move 1 from 2 to 8
    move 3 from 3 to 5
    move 1 from 9 to 8
    move 3 from 5 to 4
    move 1 from 9 to 3
    move 1 from 2 to 3
    move 4 from 8 to 2
    move 3 from 2 to 4
    move 19 from 1 to 2
    move 8 from 1 to 8
    move 1 from 4 to 3
    move 1 from 4 to 1
    move 7 from 2 to 1
    move 1 from 3 to 1
    move 2 from 3 to 1
    move 15 from 4 to 5
    move 1 from 1 to 7
    move 11 from 2 to 8
    move 2 from 2 to 9
    move 1 from 3 to 5
    move 2 from 9 to 4
    move 12 from 8 to 3
    move 16 from 5 to 1
    move 3 from 4 to 3
    move 1 from 7 to 5
    move 2 from 8 to 6
    move 1 from 5 to 4
    move 1 from 4 to 9
    move 18 from 1 to 9
    move 8 from 3 to 8
    move 9 from 8 to 2
    move 4 from 9 to 2
    move 8 from 1 to 2
    move 2 from 6 to 4
    move 17 from 2 to 1
    move 1 from 4 to 5
    move 3 from 2 to 6
    move 1 from 2 to 9
    move 2 from 6 to 1
    move 3 from 3 to 6
    move 1 from 4 to 6
    move 2 from 3 to 2
    move 16 from 9 to 5
    move 14 from 5 to 4
    move 3 from 5 to 8
    move 1 from 2 to 4
    move 4 from 8 to 6
    move 1 from 2 to 8
    move 1 from 3 to 9
    move 1 from 3 to 9
    move 2 from 9 to 1
    move 10 from 8 to 7
    move 7 from 6 to 9
    move 16 from 1 to 5
    move 7 from 4 to 3
    move 1 from 8 to 4
    move 5 from 4 to 2
    move 1 from 5 to 9
    move 5 from 9 to 1
    move 5 from 1 to 2
    move 2 from 9 to 7
    move 1 from 1 to 7
    move 1 from 6 to 8
    move 4 from 4 to 5
    move 1 from 6 to 9
    move 9 from 2 to 1
    move 11 from 5 to 6
    move 2 from 9 to 2
    move 4 from 3 to 4
    move 4 from 4 to 6
    move 1 from 3 to 4
    move 11 from 7 to 4
    move 9 from 4 to 7
    move 11 from 7 to 2
    move 2 from 3 to 5
    move 2 from 4 to 8
    move 7 from 5 to 2
    move 1 from 8 to 3
    move 1 from 5 to 1
    move 1 from 3 to 7
    move 6 from 2 to 9
    move 1 from 8 to 9
    move 6 from 9 to 2
    move 15 from 6 to 2
    move 1 from 7 to 2
    move 31 from 2 to 7
    move 22 from 7 to 3
    move 2 from 5 to 1
    move 3 from 7 to 4
    move 1 from 4 to 9
    move 3 from 4 to 3
    move 1 from 8 to 6
    move 1 from 9 to 6
    move 15 from 1 to 5
    move 1 from 9 to 5
    move 1 from 1 to 8
    move 2 from 6 to 8
    move 1 from 8 to 4
    move 1 from 4 to 6
    move 1 from 6 to 9
    move 10 from 3 to 1
    move 1 from 9 to 7
    move 2 from 7 to 8
    move 10 from 5 to 1
    move 12 from 1 to 4
    move 1 from 3 to 8
    move 11 from 4 to 8
    move 1 from 8 to 3
    move 5 from 5 to 8
    move 1 from 5 to 8
    move 6 from 8 to 6
    move 3 from 2 to 1
    move 1 from 6 to 2
    move 5 from 1 to 6
    move 3 from 1 to 4
    move 3 from 2 to 8
    move 1 from 2 to 9
    move 8 from 3 to 5
    move 2 from 1 to 3
    move 3 from 7 to 5
    move 2 from 3 to 5
    move 3 from 5 to 2
    move 1 from 7 to 9
    move 2 from 9 to 1
    move 1 from 6 to 9
    move 2 from 4 to 8
    move 5 from 6 to 5
    move 1 from 6 to 7
    move 1 from 9 to 8
    move 3 from 6 to 5
    move 7 from 8 to 9
    move 5 from 9 to 1
    move 2 from 4 to 8
    move 11 from 5 to 9
    move 3 from 2 to 3
    move 2 from 5 to 8
    move 4 from 3 to 7
    move 11 from 9 to 5
    move 3 from 7 to 5
    move 1 from 3 to 5
    move 8 from 1 to 4
    move 5 from 3 to 9
    move 15 from 5 to 4
    move 8 from 4 to 1
    move 12 from 8 to 1
    move 4 from 5 to 8
    move 12 from 4 to 5
    move 3 from 7 to 2
    move 11 from 5 to 7
    move 8 from 8 to 7
    move 7 from 9 to 8
    move 2 from 5 to 7
    move 4 from 7 to 8
    move 9 from 8 to 4
    move 11 from 4 to 5
    move 6 from 7 to 8
    move 9 from 8 to 7
    move 18 from 7 to 5
    move 1 from 8 to 1
    move 4 from 1 to 5
    move 1 from 7 to 2
    move 6 from 1 to 9
    move 1 from 2 to 4
    move 1 from 4 to 3
    move 3 from 1 to 7
    move 1 from 4 to 2
    move 3 from 2 to 5
    move 2 from 9 to 5
    move 1 from 2 to 6
    move 4 from 7 to 8
    move 1 from 6 to 2
    move 1 from 2 to 4
    move 4 from 8 to 5
    move 3 from 9 to 7
    move 1 from 9 to 5
    move 1 from 4 to 3
    move 2 from 3 to 8
    move 2 from 7 to 4
    move 28 from 5 to 8
    move 1 from 8 to 9
    move 1 from 9 to 3
    move 6 from 5 to 6
    move 5 from 5 to 2
    move 1 from 3 to 4
    move 1 from 7 to 4
    move 1 from 5 to 6
    move 16 from 8 to 3
    move 7 from 1 to 8
    move 4 from 4 to 9
    move 1 from 2 to 4
    move 3 from 2 to 3
    move 6 from 6 to 8
    move 10 from 3 to 8
    move 1 from 2 to 7
    move 1 from 6 to 7
    move 11 from 8 to 5
    move 2 from 7 to 8
    move 1 from 1 to 9
    move 5 from 9 to 5
    move 4 from 3 to 2
    move 1 from 4 to 2
    move 1 from 3 to 8
    move 3 from 8 to 2
    move 19 from 8 to 7
    move 6 from 7 to 6
    move 4 from 5 to 2
    move 9 from 7 to 5
    move 1 from 7 to 1
    move 5 from 6 to 9
    move 1 from 7 to 4
    move 1 from 6 to 7
    move 1 from 4 to 7
    move 1 from 1 to 2
    move 2 from 7 to 3
    move 6 from 5 to 9
    move 9 from 9 to 1
    move 17 from 5 to 4
    move 2 from 3 to 1
    move 13 from 4 to 7
    move 3 from 3 to 5
    move 7 from 1 to 4
    move 1 from 5 to 8
    move 2 from 5 to 2
    move 6 from 7 to 3
    move 1 from 5 to 7
    move 1 from 9 to 1
    move 2 from 3 to 2
    move 1 from 9 to 3
    move 9 from 7 to 3
    move 10 from 3 to 5
    move 8 from 4 to 2
    move 1 from 4 to 1
    move 13 from 2 to 4
    move 5 from 4 to 3
    move 1 from 5 to 9
    move 1 from 7 to 2
    move 6 from 4 to 2
    move 4 from 1 to 8
    move 3 from 4 to 6
    move 9 from 8 to 9
    move 17 from 2 to 3
    move 2 from 8 to 6
    move 1 from 4 to 3
    move 2 from 6 to 3
    move 2 from 1 to 3
    move 13 from 3 to 4
    move 8 from 9 to 8
    move 7 from 4 to 6
    move 3 from 5 to 6
    move 5 from 8 to 2
    move 9 from 6 to 1
    move 7 from 2 to 4
    move 2 from 6 to 9
    move 1 from 1 to 5
    move 18 from 3 to 8
    move 5 from 1 to 3
    move 1 from 6 to 1
    move 9 from 4 to 7
    move 11 from 8 to 7
    move 5 from 7 to 5
    move 2 from 4 to 5
    move 1 from 6 to 2
    move 13 from 7 to 8
    move 1 from 4 to 9
    move 1 from 9 to 6
    move 4 from 1 to 5
    move 1 from 7 to 6
    move 9 from 5 to 7
    move 8 from 5 to 6
    move 10 from 7 to 2
    move 1 from 5 to 7
    move 1 from 7 to 1
    move 17 from 8 to 2
    move 9 from 6 to 7
    move 6 from 7 to 1
    move 2 from 7 to 2
    move 1 from 4 to 2
    move 12 from 2 to 8
    move 7 from 1 to 2
    move 6 from 8 to 6
    move 3 from 8 to 2
    move 1 from 7 to 2
    move 2 from 3 to 4
    move 1 from 4 to 9
    move 2 from 3 to 5
    move 2 from 3 to 7
    move 1 from 4 to 6
    move 2 from 7 to 1
    move 7 from 2 to 7
    move 6 from 7 to 1
    move 1 from 5 to 2
    move 6 from 8 to 4
    move 4 from 9 to 7
    move 1 from 5 to 2
    move 3 from 8 to 1
    move 1 from 9 to 4
    move 1 from 7 to 8
    move 1 from 8 to 1
    move 4 from 7 to 8
    move 1 from 4 to 2
    move 3 from 6 to 9
    move 2 from 9 to 7
    move 1 from 9 to 3
    move 2 from 4 to 3
    move 2 from 8 to 3
    move 5 from 3 to 4
    move 4 from 6 to 2
    move 8 from 2 to 9
    move 1 from 6 to 5
    move 10 from 2 to 3
    move 2 from 8 to 3
    move 8 from 9 to 3
    move 9 from 2 to 5
    move 1 from 2 to 4
    move 1 from 2 to 3
    move 7 from 5 to 6
    move 1 from 5 to 7
    move 13 from 3 to 4
    move 2 from 7 to 8
    move 5 from 3 to 1
    move 1 from 5 to 3
    move 1 from 8 to 5
    move 1 from 2 to 8
    move 1 from 7 to 9
    move 1 from 4 to 2
    move 15 from 4 to 8
    move 6 from 4 to 7
    move 6 from 7 to 8
    move 1 from 6 to 5
    move 1 from 4 to 6
    move 1 from 9 to 6
    move 2 from 5 to 2
    move 6 from 6 to 4
    move 6 from 1 to 8
    move 6 from 4 to 9
    move 2 from 6 to 1
    move 1 from 2 to 9
    move 26 from 8 to 1
    move 4 from 3 to 7
    move 2 from 2 to 5
    move 16 from 1 to 4
    move 3 from 9 to 8
    move 3 from 8 to 7
    move 3 from 5 to 1
    move 2 from 9 to 2
    move 1 from 9 to 7
    move 1 from 9 to 1
    move 8 from 4 to 1
    move 4 from 4 to 9
    move 1 from 2 to 3
    move 1 from 3 to 7
    move 2 from 8 to 2
    move 3 from 4 to 2
    move 1 from 4 to 7
    move 9 from 7 to 5
    move 1 from 9 to 8
    move 2 from 9 to 8
    move 5 from 5 to 7
    move 1 from 9 to 5
    move 6 from 2 to 6
    move 1 from 8 to 2
    move 5 from 6 to 5
    move 1 from 7 to 4
    move 3 from 8 to 9
    move 3 from 9 to 7
    move 1 from 6 to 4
    move 2 from 4 to 1
    move 2 from 5 to 8
    move 1 from 2 to 9
    move 2 from 8 to 9
    move 3 from 9 to 3
    move 8 from 7 to 3
    move 4 from 5 to 8
    move 1 from 3 to 9
    move 3 from 5 to 8
    move 1 from 5 to 3
    move 6 from 8 to 6
    move 3 from 3 to 9
    move 5 from 3 to 2
    move 5 from 6 to 4
    move 14 from 1 to 5
    move 8 from 5 to 6
    move 2 from 3 to 2
    move 4 from 9 to 1
    move 1 from 8 to 7
    move 7 from 2 to 3
    move 6 from 3 to 7
    move 3 from 5 to 3
    move 1 from 3 to 9
    move 12 from 1 to 5
    move 1 from 9 to 7
    move 2 from 3 to 1
    move 1 from 7 to 8
    move 1 from 8 to 7
    move 2 from 3 to 6
    move 2 from 1 to 9
    move 2 from 5 to 6
    move 2 from 9 to 7
    move 9 from 7 to 3
    move 7 from 1 to 5
    move 5 from 5 to 2
    move 8 from 6 to 8
    move 5 from 8 to 9";

    // Convert directons to lines and trim whitespace -> Vec<&str>

    // Imported.

    // Use regex to capture the desired values and place them into arrays, then convert them to unsigned ints. -> Vec<[u8;3]>

    fn get_ints_from_slice(line: &str) -> [usize;3]{
        let pattern = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let matches = pattern.captures(line).unwrap();

        [matches[1].parse::<usize>().unwrap(), matches[2].parse::<usize>().unwrap(), matches[3].parse::<usize>().unwrap()]
    }

    fn get_all_ints_from_slices(lines: Vec<&str>) -> Vec<[usize;3]>{
        lines.iter().map(|line|get_ints_from_slice(line)).collect()
    }

    // Perform move operation on Stacks. -> Vec<Vec<char>>

    fn perform_move_operation_on_stacks(mut stacks: Vec<Vec<char>>, operation: [usize;3]) -> Vec<Vec<char>>{
        stacks = stacks.clone();
        let mut i: usize = 0;
        let mut temp: Option<char>;
        while i < operation[0]{
            temp = stacks[operation[1] - 1].pop();
            match temp {
                Some (value) => {
                    stacks[operation[2] - 1].push(value);
                },
                None => {}
            }
            i = i + 1;
        }
        stacks
    }

    fn perform_multigrab_move_operation_on_stacks(mut stacks: Vec<Vec<char>>, operation: [usize;3]) -> Vec<Vec<char>>{
        stacks = stacks.clone();
        let mut i: usize = 0;
        let mut temp: Option<char>;
        let mut temp_stack: Vec<char> = Vec::new();
        while i < operation[0]{
            temp = stacks[operation[1] - 1].pop();
            match temp {
                Some (value) => {
                    temp_stack.push(value);
                },
                None => {}
            }
            i = i + 1;
        }
        i = 0;
        while i < operation[0]{
            temp = temp_stack.pop();
            match temp {
                Some (value) => {
                    stacks[operation[2] - 1].push(value);
                },
                None => {}
            }
            i = i + 1;
        }
        stacks
    }

    // Perform all move operations -> Vec<Vec<char>>

    fn perform_all_move_operations_on_stacks(mut stacks: Vec<Vec<char>>, operations: Vec<[usize;3]>) -> Vec<Vec<char>>{
        stacks = stacks.clone();
        for operation in operations{
            //stacks = perform_move_operation_on_stacks(stacks, operation);
            stacks = perform_multigrab_move_operation_on_stacks(stacks, operation);
        }
        stacks
    }

    // Read top of all Stacks -> &str

    fn read_tops_of_stacks(stacks: &Vec<Vec<char>>) -> String{
        let mut output: String = String::new();
        for stack in stacks{
            let length = stack.len();
            match length {
                0 => {
                    output = format!("{}{}", output, " ".to_string());
                },
                _ => {
                    output = format!("{}{}", output, stack[length - 1].to_string());
                }
            }
        }
        output
    }

    // Print result.
    let start_tops = read_tops_of_stacks(&stacks);
    println!("{}", start_tops);
    let lines = split_lines(directions);
    let operations = get_all_ints_from_slices(lines);
    stacks = perform_all_move_operations_on_stacks(stacks, operations);
    let tops = read_tops_of_stacks(&stacks);
    println!("{}", tops);

}
