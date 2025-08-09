use num_bigint::{BigUint};
use num_traits::{One};

/// Naive recursive complexity is O(2ⁿ)
pub fn fibo_1(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibo_1(n - 1) + fibo_1(n - 2),
    }
}

/// Use cache to avoid recomputation; complexity O(N)
/// Still uses recursion but only N times.
pub fn fibo_2(n: u32, cache: &mut Vec<u128>) -> u128 {
    if cache[n as usize] != u128::MAX {
        return cache[n as usize];
    }
    cache[n as usize] = fibo_2(n - 1, cache) + fibo_2(n - 2, cache);
    cache[n as usize]
}

/// Avoid recursion overhead entirely; complexity O(N) -- extremely fast
pub fn fibo_3(n: u64) -> u128 {
    if n < 2 {
        return n as u128;
    }

    let mut prev = 0u128;
    let mut curr = 1u128;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

pub fn fibo_3_large(n: u64) -> BigUint {
    if n < 2 {
        return BigUint::from(n);
    }

    let mut prev = BigUint::ZERO;
    let mut curr = BigUint::one();
    for _ in 2..n {
        let next = &prev + &curr;
        prev = curr;
        curr = next;
    }
    curr
}

/// Constant-Time (Matrix Exponential or Closed Form)
/// Fast doubling -- complexity O(log n)
/// Avoid floating point errors
pub fn fibo_4(n: u64) -> BigUint {
    fn fib_pair(n: u64) -> (BigUint, BigUint) {
        if n == 0 {
            (BigUint::ZERO, BigUint::one())
        } else {
            let (a, b) = fib_pair(n >> 1);
            let two = BigUint::from(2u8);
            let c = &a * (&b * &two - &a);
            let d = &a * &a + &b * &b;
            if n & 1 == 0 {
                (c, d)
            } else {
                (d.clone(), &c + &d)
            }
        }
    }
    fib_pair(n).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(fibo_1(0), 0);
        assert_eq!(fibo_1(1), 1);

        assert_eq!(fibo_1(7), 13);
    }

    #[test]
    fn test_2() {
        fn helper(n: u32) -> u128 {
            if n < 2 {
                return n as u128;
            }
            let mut cache = vec![u128::MAX; (n + 1) as usize];
            cache[0] = 0;
            cache[1] = 1;
            fibo_2(n, &mut cache)
        }
        assert_eq!(helper(0), 0);
        assert_eq!(helper(1), 1);
        assert_eq!(helper(7), 13);
    }

    #[test]
    fn test_3() {
        assert_eq!(fibo_3(100), 354224848179261915075);
    }

    #[test]
    fn test_4() {
        assert_eq!(fibo_4(100), BigUint::from(354224848179261915075 as u128));
        fibo_4(1_000_000);
    }
}
