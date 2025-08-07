pub fn fibo(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibo(n - 1) + fibo(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::fibo;

    #[test]
    fn test_1() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(7), 13);
    }
}

