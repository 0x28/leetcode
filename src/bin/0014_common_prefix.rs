#![allow(unused)]

struct Solution;

impl Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();

        for p in 0.. {
            let ch = match strs.first().unwrap().as_bytes().get(p) {
                None => break,
                Some(c) => c,
            };

            if strs.iter().all(|s| s.as_bytes().get(p) == Some(ch)) {
                prefix.push(*ch as char);
            } else {
                break;
            }
        }

        prefix
    }
}

fn main() {}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned()
        ]),
        "fl".to_owned()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "ab".to_owned(),
            "a".to_owned(),
            "abc".to_owned()
        ]),
        "a".to_owned()
    );
}
