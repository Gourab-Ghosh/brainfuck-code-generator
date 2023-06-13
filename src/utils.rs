use super::*;

pub fn get_smallest_prime_factor(number: u8) -> u8 {
    for prime in PRIMES {
        if number % prime == 0 {
            return prime;
        }
    }
    unreachable!()
}

pub fn is_prime(number: u8) -> bool {
    PRIMES.contains(&number)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Stack {
    start_index: usize,
    end_index: usize,
}

impl Stack {
    pub fn new(start_index: usize, length: usize) -> Stack {
        Stack {
            start_index,
            end_index: start_index + length,
        }
    }

    pub fn get_start_index(&self) -> usize {
        self.start_index
    }

    pub fn get_end_index(&self) -> usize {
        self.end_index
    }
}
