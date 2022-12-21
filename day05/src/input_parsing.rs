pub fn split_lines(input_string: &str) -> Vec<&str>{
    let rough_split: Vec<&str> = input_string.trim().lines().collect();
    rough_split.iter().map(|current_line| current_line.trim()).collect()
}

pub fn split_lines_on_slice_match<'a>(input_strings: Vec<&'a str>, split_on: &str) -> Vec<[&'a str;2]>{
    let mut outputs: Vec<[&str;2]> = Vec::new();
    for current_string in input_strings{
        let split_vec: Vec<&str> = current_string.split(split_on).collect();
        outputs.push([split_vec[0],split_vec[1]]);
    }
    outputs
}

