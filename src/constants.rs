#[cfg(debug_assertions)]
pub const OPTIMISE_CODE: bool = false;
#[cfg(not(debug_assertions))]
pub const OPTIMISE_CODE: bool = true;

pub const PRIMES: [u8; 54] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
];
pub const BAD_PATTERNS: [&str; 5] = ["+-", "-+", "<>", "><", "[]"];
// pub const WORDWRAP_THRESHOLD: usize = 65;
pub const WORDWRAP_THRESHOLD: usize = usize::MAX;
