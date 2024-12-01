use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in lines {
        let nums = line.as_ref().split_whitespace().collect::<Vec<_>>();
        list1.push(nums[0].parse::<i32>().map_err(|_| Error::ParseError)?);
        list2.push(nums[1].parse::<i32>().map_err(|_| Error::ParseError)?);
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let result = list1.iter().zip(list2.iter()).map(|(a, b)| (a - b).abs()).sum();
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in lines {
        let nums = line.as_ref().split_whitespace().collect::<Vec<_>>();
        list1.push(nums[0].parse::<i32>().map_err(|_| Error::ParseError)?);
        list2.push(nums[1].parse::<i32>().map_err(|_| Error::ParseError)?);
    }
    let mut freq = HashMap::new();
    list2.iter().for_each(|&num| *freq.entry(num).or_insert(0) += 1);
    let mut result = 0;
    for x in list1 {
        result += x * *freq.get(&x).unwrap_or(&0);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(11), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(31), result);
    }
}
