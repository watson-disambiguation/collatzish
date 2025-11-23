use std::{collections::HashMap, io};

enum Command {
    Loop,
    Path(u64),
    Add(u64),
    Quit,
    Counts,
}

fn scan_commands() -> Option<Command> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    if let Err(_) = stdin.read_line(&mut buffer) {
        return None;
    }
    let cleaned_input = buffer.strip_suffix("\n")?;
    if cleaned_input.eq("l") {
        Some(Command::Loop)
    }
    else if cleaned_input.eq("q") {
        Some(Command::Quit)
    }
    else if cleaned_input.eq("c") {
        Some(Command::Counts)
    }
    else if cleaned_input.starts_with("p") {
        let value: u64 = match cleaned_input.strip_prefix("p")?.parse() {
            Ok(n) => n,
            Err(_) => { return None; },
        };
        Some(Command::Path(value))
    }
    else if cleaned_input.starts_with("a") {
        let value: u64 = match cleaned_input.strip_prefix("a")?.parse() {
            Ok(n) => n,
            Err(_) => { return None; },
        };
        Some(Command::Add(value))
    }
    else {
        None
    }
}

fn scan_input_unsigned_integer(label: &str, default_value: Option<u64>) -> u64 {
    print!("Input {}.",label);
    if let Some(default) = default_value {
        print!(" Default value is {}.\n",default);
    }
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        if let Err(_) = stdin.read_line(&mut buffer) {
            println!("Error reading input, please try again.");
            continue;
        }
        let number_text = match buffer.strip_suffix("\n") {
            Some(s) => s,
            None => {
                println!("Error reading input, please try again.");
                continue;
            },
        };
        // empty string, so use default value
        if let Some(default) = default_value {
            if number_text.is_empty() {
                return default;
            }
        }
        match number_text.parse::<u64>() {
            Ok(n) => return n,
            Err(_) => println!("Unable to parse input, please try again."),
        }
    }
    
}

fn console_loop(calculator: &mut CollatzishMemoized) {
    println!("Input commands:");
    loop {
        if let Some(command) = scan_commands() {
            match command {
                Command::Counts => counts(calculator),
                Command::Quit => return,
                _ => println!("Command not implemented"),
            };
        }
        else {
            println!("Invalid Command, please try again");
        }
    }
}

fn counts(calculator: &mut CollatzishMemoized) {
    let mut counts = HashMap::new();
    for final_loop in calculator.final_loop_map.keys() {
        counts.insert(final_loop.clone(), 0u64);
    }
    let filtered_iter = calculator.number_path_map
        .iter()
        .map(|(k,v)| v.clone().final_loop);
    let total = calculator.number_path_map.iter().count();
    for final_loop in filtered_iter {
        if let Some(count) = counts.get(&final_loop) {
            counts.insert(final_loop, count+1);
        }
    }
    for (final_loop,count) in counts.iter() {
        print!("{} : {}/{} ",final_loop.clone(),count.clone(),total);
    }
    print!("\n");
}

fn main() {
    let mult = scan_input_unsigned_integer("the multiplicative factor", Some(3));
    let add = scan_input_unsigned_integer("the additive factor", Some(1));
    let limit = scan_input_unsigned_integer("the maximum starting number", Some(10000));
    let mut calculator = CollatzishMemoized::new(mult, add);
    calculator.add_all_paths(limit);
    console_loop(&mut calculator);
    
}

#[derive(Debug, Clone)]
struct NumberPath {
    start: u64,
    next: u64,
    final_loop: u64,
    length_to_loop: u64,
}

#[derive(Debug, Clone)]
struct FinalLoop {
    min_value: u64,
    loop_length: u64,
}

struct CollatzishMemoized {
    number_path_map: HashMap<u64,NumberPath>,
    final_loop_map: HashMap<u64,FinalLoop>,
    mult: u64,
    add: u64,
}

impl CollatzishMemoized {
    fn new(mult: u64, add: u64) -> Self {
        Self { 
            number_path_map: HashMap::new(), 
            final_loop_map: HashMap::new(), 
            mult, 
            add 
        }
    }

    fn calc_next(&self, x: u64) -> u64 {
        if x % 2 == 0 {
            return x / 2;
        }
        else {
            return self.mult * x + self.add;
        }
    }

    fn add_path(&mut self, starting_num: u64) -> NumberPath {
        if let Some(path) = self.number_path_map.get(&starting_num) {
            return path.clone();
        }
        let mut path_list = vec![starting_num];
        let mut new_loop = false;
        // keep going until we reach a known value of some kind
        let base_path = loop {
            let curr_num = path_list.last().expect("Should never be empty.").clone();
            let next_num = self.calc_next(curr_num);
            if let Some(path) = self.number_path_map.get(&next_num) {
                break path.clone();
            }
            // need to check if a loop has been found
            if path_list.contains(&next_num) {
                // we found a loop, so we need to add the loop
                path_list = path_list.into_iter().take_while(|x| *x != next_num).collect();
                break self.add_loop(next_num);
            }
            path_list.push(next_num);
        };
        let mut curr_dist_from_loop = base_path.length_to_loop;
        for num in path_list.into_iter().rev() {
            curr_dist_from_loop += 1;
            self.number_path_map.insert(num, 
                NumberPath { 
                    start: num, 
                    next: self.calc_next(num), 
                    final_loop: base_path.final_loop, 
                    length_to_loop: curr_dist_from_loop 
                }
            );
            
        }
        return NumberPath { 
            start: starting_num, 
            next: self.calc_next(starting_num), 
            final_loop: base_path.final_loop, 
            length_to_loop: curr_dist_from_loop 
        };
    }

    // we have found the first item of a new loop, so we need to go set all of its elements
    // to mark them as a part of a loop. Returns the path for the number you entered the loop from
    fn add_loop(&mut self, starting_num: u64) -> NumberPath {
        let mut loop_list = vec![starting_num];
        let mut min_num = starting_num;
        let mut curr_num = self.calc_next(starting_num);
        let mut loop_length = 1u64;
        while curr_num != starting_num {     
            loop_length += 1;
            if curr_num < min_num {
                min_num = curr_num;
            }
            loop_list.push(curr_num);
            curr_num = self.calc_next(curr_num);
        }
        self.final_loop_map.insert(min_num, FinalLoop { min_value: min_num, loop_length });
        for num in loop_list.into_iter() {
            self.number_path_map.insert(
                num, 
                NumberPath { 
                    start: num, 
                    next: self.calc_next(num), 
                    final_loop: min_num, 
                    length_to_loop: 0 
                }
            );
        }
        NumberPath {
            start: starting_num,
            next: self.calc_next(starting_num),
            final_loop: min_num,
            length_to_loop: 0,
        }
    }

    fn add_all_paths(&mut self, limit: u64) {
        for i in 1..=limit {
            self.add_path(i);
        }
    }

    fn create_path_list(&self, number_path: NumberPath) -> Vec<u64> {
        let mut path_list = Vec::new();
        let mut curr_number_path = number_path;
        while curr_number_path.length_to_loop > 0 {
            path_list.push(curr_number_path.start);
            curr_number_path = self.number_path_map
                .get(&curr_number_path.next)
                .expect("If path start exists, whole path should exist")
                .clone();
        }
        let loop_start = curr_number_path.clone();
        while loop_start.start != curr_number_path.next {
            path_list.push(curr_number_path.start);
            curr_number_path = self.number_path_map
                .get(&curr_number_path.next)
                .expect("If path start exists, whole path should exist")
                .clone();
        }
        path_list.push(curr_number_path.start);
        return path_list;
    }

    fn get_path(&self, starting_num: u64) -> Option<Vec<u64>> {
        let number_path = self.number_path_map.get(&starting_num)?.clone();
        return Some(self.create_path_list(number_path));
    }

    // calcs the path if it doesn't already exist
    fn get_add_path(&mut self, starting_num: u64) -> Vec<u64> {
        let number_path = self.add_path(starting_num);
        return self.create_path_list(number_path);
    }

}


