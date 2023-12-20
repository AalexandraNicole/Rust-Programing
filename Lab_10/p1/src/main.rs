use std::collections::HashMap;
use std::io::{self, Write};

struct Cache {
    data: HashMap<u64, bool>, // Key: Number, Value: Whether it's prime or not
    order: Vec<u64>,          // Maintain order of insertion
    capacity: usize,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Cache {
            data: HashMap::new(),
            order: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn insert(&mut self, num: u64, is_prime: bool) {
        if self.order.len() >= self.capacity {
            let removed = self.order.remove(0);
            self.data.remove(&removed);
        }

        self.order.push(num);
        self.data.insert(num, is_prime);
    }

    fn get(&self, num: &u64) -> Option<&bool> {
        self.data.get(num)
    }
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut cache = Cache::new(10);

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let num: u64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        if let Some(&is_prime) = cache.get(&num) {
            println!("Result from cache: {} is prime: {}", num, is_prime);
        } else {
            let is_prime = is_prime(num);
            println!("{} is prime: {}", num, is_prime);
            cache.insert(num, is_prime);
        }
    }
}
