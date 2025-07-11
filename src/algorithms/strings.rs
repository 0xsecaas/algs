pub fn find_the_difference(s1: String, s2: String) -> char {
    for c in s1.chars() {
        let count_in_s1 = s1.matches(c).count();
        let count_in_s2 = s2.matches(c).count();
        if count_in_s1 > count_in_s2 {
            return c;
        }
    }
    unreachable!();
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
mod tests {
    use super::{find_the_difference, merge_alternately};

    #[test]
    fn test_1() {
        let s1: String = String::from("abcd");
        let s2 = String::from("abcde");

        //assert_eq!(find_the_difference(s1, s2), 'e');
    }

    #[test]
    fn test_merge() {
        let s1: String = "HloTe".into();
        let s2 = "el hre".to_string();
        assert_eq!(merge_alternately(s1, s2), String::from("Hello There"));
    }
}
