use super::*;
use BigNumberEnum::*;

enum BigNumberEnum {
    BigNum(Box<BigNumberEnum>, Box<BigNumberEnum>),
    U8 { index: usize },
}

impl BigNumberEnum {
    pub fn new(size: u8, index: usize) -> Self {
        if size.count_ones() != 1 {
            panic!("Only power of 2 supported");
        }
        if size < 8 {
            panic!("Min size should be 8");
        }
        if size == 8 {
            return U8 { index };
        }
        let index2 = index + size as usize / 2;
        BigNum(
            Box::new(BigNumberEnum::new(size / 2, index)),
            Box::new(BigNumberEnum::new(size / 2, index2)),
        )
    }

    pub fn get_index(&self) -> usize {
        match self {
            BigNum(sub_num_1, _) => sub_num_1.get_index(),
            U8 { index } => *index,
        }
    }

    pub fn get_size(&self) -> u8 {
        match self {
            BigNum(sub_num_1, _) => 2 * sub_num_1.get_size(),
            U8 { .. } => 8,
        }
    }

    pub fn get_value(&self, memory: &[u8]) -> u64 {
        match self {
            BigNum(sub_num_1, sub_num_2) => {
                let size = sub_num_1.get_size();
                let value_1 = sub_num_1.get_value(memory);
                let value_2 = sub_num_2.get_value(memory);
                let value = (value_1 << size) | value_2;
                value
            }
            U8 { index } => memory[*index] as u64,
        }
    }
}
