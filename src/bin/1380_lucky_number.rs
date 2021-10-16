#![allow(unused)]

struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut mins = vec![i32::MAX; rows];
        let mut maxs = vec![i32::MIN; cols];

        for (idx, row) in matrix.iter().enumerate() {
            mins[idx] = *row.iter().min().unwrap();
        }

        for col in 0..matrix[0].len() {
            maxs[col] = matrix.iter().map(|row| row[col]).max().unwrap();
        }

        for min in &mins {
            for max in &maxs {
                if max == min {
                    return vec![*max];
                }
            }
        }

        vec![]
    }
}

fn main() {}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::lucky_numbers(vec![
            vec![3, 7, 8],
            vec![9, 11, 13],
            vec![15, 16, 17],
        ]),
        vec![15]
    );
}
