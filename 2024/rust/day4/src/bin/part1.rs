use nom::IResult;
use nom::bytes::complete::take_until;
use std::env;
use std::fs;

fn read_lines(filename: &str) -> eyre::Result<Vec<String>> {
    let mut result = vec![];

    for line in fs::read_to_string(filename)?.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

struct LeftRightIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    cur_row: usize,
    cur_column: usize,
}

impl<'a, T: AsRef<str>> Iterator for LeftRightIter<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row >= self.n_rows {
            return None;
        }

        let result = if self.cur_column == self.n_columns {
            '\n'
        } else {
            let row: &str = self.matrix[self.cur_row].as_ref();
            row.chars().nth(self.cur_column).unwrap()
        };

        if self.cur_column < self.n_columns {
            self.cur_column += 1;
        } else {
            self.cur_column = 0;
            self.cur_row += 1;
        }

        Some(result)
    }
}

struct TopDownIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    cur_row: usize,
    cur_column: usize,
}

impl<'a, T: AsRef<str>> Iterator for TopDownIter<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_column >= self.n_columns {
            return None;
        }

        let result = if self.cur_row == self.n_rows {
            '\n'
        } else {
            let row: &str = self.matrix[self.cur_row].as_ref();
            row.chars().nth(self.cur_column).unwrap()
        };

        if self.cur_row < self.n_rows {
            self.cur_row += 1;
        } else {
            self.cur_row = 0;
            self.cur_column += 1;
        }

        Some(result)
    }
}

struct SWNEIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    cur_row: usize,
    cur_column: usize,
    end_of_line: bool,
    line_row: usize,
    line_column: usize,
}

impl<'a, T: AsRef<str>> Iterator for SWNEIter<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_column >= self.n_columns {
            return None;
        }

        let result = if self.end_of_line {
            '\n'
        } else {
            let row: &str = self.matrix[self.cur_row].as_ref();
            row.chars().nth(self.cur_column).unwrap()
        };

        if self.end_of_line {
            self.end_of_line = false;

            if self.line_row < self.n_rows - 1 {
                self.line_row += 1;
            } else {
                self.line_column += 1;
            }

            self.cur_row = self.line_row;
            self.cur_column = self.line_column;
        } else if self.cur_row == 0 || self.cur_column == self.n_columns - 1 {
            self.end_of_line = true;
        } else {
            self.cur_row -= 1;
            self.cur_column += 1;
        }

        Some(result)
    }
}

struct SENWIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    cur_row: usize,
    cur_column: usize,
    end_of_line: bool,
    line_row: usize,
    line_column: usize,
}

impl<'a, T: AsRef<str>> Iterator for SENWIter<'a, T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_column >= self.n_columns {
            return None;
        }

        let result = if self.end_of_line {
            '\n'
        } else {
            let row: &str = self.matrix[self.cur_row].as_ref();
            row.chars()
                .nth(self.n_columns - 1 - self.cur_column)
                .unwrap()
        };

        if self.end_of_line {
            self.end_of_line = false;

            if self.line_row < self.n_rows - 1 {
                self.line_row += 1;
            } else {
                self.line_column += 1;
            }

            self.cur_row = self.line_row;
            self.cur_column = self.line_column;
        } else if self.cur_row == 0 || self.cur_column == self.n_columns - 1 {
            self.end_of_line = true;
        } else {
            self.cur_row -= 1;
            self.cur_column += 1;
        }

        Some(result)
    }
}

trait Matrix<'a, T: AsRef<str>> {
    fn iter_left_right(self) -> LeftRightIter<'a, T>;
    fn iter_top_down(self) -> TopDownIter<'a, T>;
    fn iter_sw_ne(self) -> SWNEIter<'a, T>;
    fn iter_se_nw(self) -> SENWIter<'a, T>;
}

impl<'a, T: AsRef<str>> Matrix<'a, T> for &'a [T] {
    fn iter_left_right(self) -> LeftRightIter<'a, T> {
        assert!(self.len() > 0);
        let first_row: &str = self[0].as_ref();
        assert!(first_row.len() > 0);

        LeftRightIter {
            matrix: self,
            n_rows: self.len(),
            n_columns: first_row.len(),
            cur_row: 0,
            cur_column: 0,
        }
    }

    fn iter_top_down(self) -> TopDownIter<'a, T> {
        assert!(self.len() > 0);
        let first_row: &str = self[0].as_ref();
        assert!(first_row.len() > 0);

        TopDownIter {
            matrix: self,
            n_rows: self.len(),
            n_columns: first_row.len(),
            cur_row: 0,
            cur_column: 0,
        }
    }

    fn iter_sw_ne(self) -> SWNEIter<'a, T> {
        assert!(self.len() > 0);
        let first_row: &str = self[0].as_ref();
        assert!(first_row.len() > 0);

        SWNEIter {
            matrix: self,
            n_rows: self.len(),
            n_columns: first_row.len(),
            cur_row: 0,
            cur_column: 0,
            end_of_line: false,
            line_row: 0,
            line_column: 0,
        }
    }

    fn iter_se_nw(self) -> SENWIter<'a, T> {
        assert!(self.len() > 0);
        let first_row: &str = self[0].as_ref();
        assert!(first_row.len() > 0);

        SENWIter {
            matrix: self,
            n_rows: self.len(),
            n_columns: first_row.len(),
            cur_row: 0,
            cur_column: 0,
            end_of_line: false,
            line_row: 0,
            line_column: 0,
        }
    }
}

struct Lines<I: Iterator<Item = char>> {
    it: I,
}

impl<I: Iterator<Item = char>> Iterator for Lines<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = "".to_string();
        for c in &mut self.it {
            if c == '\n' {
                if s.is_empty() {
                    return None;
                } else {
                    return Some(s);
                }
            }
            s.push(c)
        }
        if s.is_empty() {
            return None;
        } else {
            return Some(s);
        }
    }
}

fn lines<I: Iterator<Item = char>>(it: I) -> Lines<I> {
    Lines { it }
}

fn parse_xmas(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_until("XMAS")(input)?;
    Ok((&input[4..], ()))
}

fn parse_samx(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_until("SAMX")(input)?;
    Ok((&input[4..], ()))
}

fn count_xmas(input: &str) -> u32 {
    let mut count = 0;
    let mut input = input;
    while let Ok((new_input, _)) = parse_xmas(input) {
        count += 1;
        input = new_input;
    }
    count
}

fn count_samx(input: &str) -> u32 {
    let mut count = 0;
    let mut input = input;
    while let Ok((new_input, _)) = parse_samx(input) {
        count += 1;
        input = new_input;
    }
    count
}

fn main() -> eyre::Result<()> {
    // This problem is interesting. My idea to solve it is the following. We always need to find
    // the sequence "XMAS", but from different views into the matrix. The first view is the usual
    // left to right. "XMAS" in reverse also counts, so we need to do right to left; that is the
    // second view. Both views are horizontal views. Then there are vertical views: top to bottom
    // and bottom to top. Finally, there are diagonal views, of which there are four: SW to NE, NE
    // to SW, SE to NW and NW to SE.
    //
    // The thing is I do not want to make a modified copy of the matrix for each view. I only want
    // to change the way I iterate over the matrix. Instead of doing the eight views, I only do
    // four and search for "XMAS" and "SAMX" for each. This is likely faster that way, because I
    // allocate a new String for each line of a view. So for one view, I allocate the entire matrix
    // another time. Better to do it four times than eight. I do not see why it would not be
    // possible to never allocate. However, that means parsing input from an iterator, which nom
    // cannot do. Also, it was not clear how to write an iterator adapter that returns an iterator
    // over lines, where the lines are themselves iterators as we do not want to allocate memory.

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let matrix = read_lines(&args[1])?;

    let mut result = 0;
    for line in lines(matrix.iter_left_right()) {
        result += count_xmas(&line);
        result += count_samx(&line);
    }
    for line in lines(matrix.iter_top_down()) {
        result += count_xmas(&line);
        result += count_samx(&line);
    }
    for line in lines(matrix.iter_sw_ne()) {
        result += count_xmas(&line);
        result += count_samx(&line);
    }
    for line in lines(matrix.iter_se_nw()) {
        result += count_xmas(&line);
        result += count_samx(&line);
    }
    println!("{}", result);

    Ok(())
}
