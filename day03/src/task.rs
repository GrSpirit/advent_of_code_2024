use regex::Regex;

#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut result = 0;
    let input: String = lines.iter().map(|l| l.as_ref()).collect::<Vec<&str>>().concat();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [xs, ys]) in re.captures_iter(&input).map(|c| c.extract()) {
        let x = xs.parse::<i32>().map_err(|_| Error::ParseError)?;
        let y = ys.parse::<i32>().map_err(|_| Error::ParseError)?;
        result += x * y;
    }
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut result = 0;
    let mut is_mul_enabled = true;
    let input: String = lines.iter().map(|l| l.as_ref()).collect::<Vec<&str>>().concat();
    let re = Regex::new(r"(?<mul>mul\((\d+),(\d+)\))|(?<disable>don't\(\))|(?<enable>do\(\))").unwrap();
    for cap in re.captures_iter(&input) {
        if cap.name("disable").is_some() {
            is_mul_enabled = false;
        } else if cap.name("enable").is_some() {
            is_mul_enabled = true;
        } else if cap.name("mul").is_some() && is_mul_enabled {
            let x = cap[2].parse::<i32>().map_err(|_| Error::ParseError)?;
            let y = cap[3].parse::<i32>().map_err(|_| Error::ParseError)?;
            result += x * y;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const DATA2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn task1_test() {
        let lines = DATA1.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(161), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA2.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(48), result);
    }
}
