use std::{collections::HashMap, iter};

fn main() {
    let mut calculator = CollatzishMemoized::new(3, 5);
    calculator.add_all_paths(1000000000);
    println!("{:#?}",calculator.final_loop_map);
    //println!("{:#?}",calculator.number_path_map);
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
        let mut path_list = vec![starting_num];
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
        for i in 1..limit {
            self.add_path(i);
        }
    }

}


