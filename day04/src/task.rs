use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub type Result<T> = std::result::Result<T, Error>;

fn search_word(grid: &Vec<Vec<char>>, word: &str, dir: Direction, ii: usize, jj: usize) -> bool {
    let Some(c) = word.chars().next() else { return true; };
    match dir {
        Direction::Up => ii > 0 && grid[ii - 1][jj] == c && search_word(grid, &word[1..], Direction::Up, ii - 1, jj),
        Direction::Down => ii < grid.len() - 1 && grid[ii + 1][jj] == c && search_word(grid, &word[1..], Direction::Down, ii + 1, jj),
        Direction::Left => jj > 0 && grid[ii][jj - 1] == c && search_word(grid, &word[1..], Direction::Left, ii, jj - 1),
        Direction::Right => jj < grid[0].len() - 1 && grid[ii][jj + 1] == c && search_word(grid, &word[1..], Direction::Right, ii, jj + 1),
        Direction::UpLeft => ii > 0 && jj > 0 && grid[ii - 1][jj - 1] == c && search_word(grid, &word[1..], Direction::UpLeft, ii - 1, jj - 1),
        Direction::UpRight => ii > 0 && jj < grid[0].len() - 1 && grid[ii - 1][jj + 1] == c && search_word(grid, &word[1..], Direction::UpRight, ii - 1, jj + 1),
        Direction::DownLeft => ii < grid.len() - 1 && jj > 0 && grid[ii + 1][jj - 1] == c && search_word(grid, &word[1..], Direction::DownLeft, ii + 1, jj - 1),
        Direction::DownRight => ii < grid.len() - 1 && jj < grid[0].len() - 1 && grid[ii + 1][jj + 1] == c && search_word(grid, &word[1..], Direction::DownRight, ii + 1, jj + 1),
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let grid = lines.into_iter().map(|line| line.as_ref().chars().collect::<Vec<_>>()).collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                for dir in Direction::iter() {
                    if search_word(&grid, "MAS", dir, i, j) {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let grid = lines.into_iter().map(|line| line.as_ref().chars().collect::<Vec<_>>()).collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 'A' {
                let first = (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S') || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M');
                let second = (grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S') || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M');
                if first && second {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(18), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(9), result);
    }
}
