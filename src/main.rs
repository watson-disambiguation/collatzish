use std::{collections::HashMap};

fn main() {
    let mut calculator = CollatzishMemoized::new(3, 5);
    let limit = 100000;
    calculator.add_all_paths(limit);
    let mut counts = HashMap::new();
    for final_loop in calculator.final_loop_map.keys() {
        counts.insert(final_loop.clone(), 0u64);
        println!("{:?}",calculator.get_path(*final_loop));
    }
    let filtered_iter = calculator.number_path_map
        .iter()
        .filter(|(k,v)| **k <= limit)
        .map(|(k,v)| v.clone().final_loop);
    for final_loop in filtered_iter {
        if let Some(count) = counts.get(&final_loop) {
            counts.insert(final_loop, count+1);
        }
    }
    println!("{:#?}",counts);
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


