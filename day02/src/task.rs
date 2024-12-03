#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError
}

pub type Result<T> = std::result::Result<T, Error>;

fn is_safe(row: &[i32]) -> std::result::Result<(), usize> {
    let sign = (row[0] - row[1]).signum();
    if sign == 0 { return Err(0); }
    let wrong = row.windows(2).position(|pair| {
        let diff = pair[0] - pair[1];
        diff.signum() != sign || diff.abs() > 3
    });
    if let Some(pos) = wrong {
        Err(pos)
    } else {
        Ok(())
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let rows = lines.iter().map(|line| {
        line.as_ref().split_whitespace()
            .map(|num| num.parse::<i32>().map_err(|_| Error::ParseError))
            .collect::<Result<Vec<i32>>>()
    }).collect::<Result<Vec<Vec<i32>>>>()?;
    let count = rows.iter().filter(|row| {
        is_safe(row).is_ok()
    }).count() as u32;
    Ok(count)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let rows = lines.iter().map(|line| {
        line.as_ref().split_whitespace()
            .map(|num| num.parse::<i32>().map_err(|_| Error::ParseError))
            .collect::<Result<Vec<i32>>>()
    }).collect::<Result<Vec<Vec<i32>>>>()?;
    let exclude_pos = |row: &[i32], pos: usize|
        is_safe(&[&row[..pos], &row[pos + 1..]].concat()).is_ok();
    let count = rows.iter().filter(|row| {
        if let Err(pos) =  is_safe(row) {
            exclude_pos(row, pos) || exclude_pos(row, pos + 1) || (pos == 1 && exclude_pos(row, pos - 1))
        } else { true }
    }).count() as u32;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
8 9 7 6 5";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(2), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(5), result);
    }
}
