pub fn xor(s1: String, s2: String) -> char {
    let mut res: u8 = 0;

    for c in s1.bytes() {
        res ^= c;
    }
    for c in s2.bytes() {
        res ^= c;
    }

    res as char
}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::with_capacity(word1.len() + word2.len());

    let mut iter1 = word1.chars();
    let mut iter2 = word2.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => break,
            (None, Some(c)) => result.push(c),
            (Some(c), None) => result.push(c),
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
        }
    }
    result
}

#[cfg(test)]
mod units {
    use super::{merge_alternately, xor};

    #[test]
    fn test_xor() {
        assert_eq!(xor(String::from("abcd"), String::from("abcde")), 'e');
        assert_eq!(xor(String::new(), String::from("y")), 'y');
        assert_eq!(xor(String::from("x"), String::new()), 'x');
    }

    #[test]
    fn test_merge() {
        let s1: String = "HloTe".into();
        let s2 = "el hre".to_string();
        assert_eq!(merge_alternately(s1, s2), String::from("Hello There"));
    }
}
