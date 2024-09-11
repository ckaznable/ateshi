use std::mem;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn lcm_of_multiple(numbers: &[u64]) -> Option<u64> {
    if numbers.is_empty() {
        return None;
    }

    if numbers.len() == 1 {
        return Some(numbers[0]);
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        if num != 0 {
            result = lcm(result, num);
        }
    }

    Some(result)
}
