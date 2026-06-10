use nom::IResult;
use nom::bytes::complete::take_until;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::Hash;

fn read_lines(filename: &str) -> eyre::Result<Vec<String>> {
    let mut result = vec![];

    for line in fs::read_to_string(filename)?.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    r: usize,
    c: usize,
}

struct SWNEIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    line_row: usize,
    line_column: usize,
}

impl<'a, T: AsRef<str>> SWNEIter<'a, T> {
    fn build(matrix: &'a [T]) -> Option<Self> {
        if matrix.len() <= 0 {
            return None;
        };
        let first_row: &str = matrix[0].as_ref();
        if first_row.len() <= 0 {
            return None;
        };

        Some(SWNEIter {
            matrix,
            n_rows: matrix.len(),
            n_columns: first_row.len(),
            line_row: 0,
            line_column: 0,
        })
    }
}

impl<'a, T: AsRef<str>> Iterator for SWNEIter<'a, T> {
    type Item = Vec<Coord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_column >= self.n_columns {
            return None;
        }

        let mut coords = vec![];
        let mut r = self.line_row;
        let mut c = self.line_column;
        loop {
            coords.push(Coord { r, c });

            if r == 0 || c == self.n_columns - 1 {
                break;
            }

            r -= 1;
            c += 1;
        }

        if self.line_row < self.n_rows - 1 {
            self.line_row += 1;
        } else {
            self.line_column += 1;
        }

        Some(coords)
    }
}

struct SENWIter<'a, T>
where
    T: AsRef<str>,
{
    matrix: &'a [T],
    n_rows: usize,
    n_columns: usize,
    line_row: usize,
    line_column: usize,
}

impl<'a, T: AsRef<str>> SENWIter<'a, T> {
    fn build(matrix: &'a [T]) -> Option<Self> {
        if matrix.len() <= 0 {
            return None;
        };
        let first_row: &str = matrix[0].as_ref();
        if first_row.len() <= 0 {
            return None;
        };

        Some(SENWIter {
            matrix,
            n_rows: matrix.len(),
            n_columns: first_row.len(),
            line_row: 0,
            line_column: 0,
        })
    }
}

impl<'a, T: AsRef<str>> Iterator for SENWIter<'a, T> {
    type Item = Vec<Coord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_column >= self.n_columns {
            return None;
        }

        let mut coords = vec![];
        let mut r = self.line_row;
        let mut c = self.line_column;
        loop {
            coords.push(Coord {
                r,
                c: self.n_columns - 1 - c,
            });

            if r == 0 || c == self.n_columns - 1 {
                break;
            }

            r -= 1;
            c += 1;
        }

        if self.line_row < self.n_rows - 1 {
            self.line_row += 1;
        } else {
            self.line_column += 1;
        }

        Some(coords)
    }
}

trait Matrix<'a, T: AsRef<str>> {
    fn iter_sw_ne(self) -> Option<SWNEIter<'a, T>>;
    fn iter_se_nw(self) -> Option<SENWIter<'a, T>>;
    fn coords_to_string(self, coords: &[Coord]) -> String;
}

impl<'a, T: AsRef<str>> Matrix<'a, T> for &'a [T] {
    fn iter_sw_ne(self) -> Option<SWNEIter<'a, T>> {
        SWNEIter::build(self)
    }

    fn iter_se_nw(self) -> Option<SENWIter<'a, T>> {
        SENWIter::build(self)
    }

    fn coords_to_string(self, coords: &[Coord]) -> String {
        let mut result = "".to_string();
        for coord in coords {
            let row: &str = self[coord.r].as_ref();
            let c = row.chars().nth(coord.c).unwrap();
            result.push(c);
        }
        result
    }
}

fn parse_mas(input: &str) -> IResult<&str, usize> {
    let (input, taken) = take_until("MAS")(input)?;
    Ok((&input[3..], taken.len() + 3))
}

fn parse_sam(input: &str) -> IResult<&str, usize> {
    let (input, taken) = take_until("SAM")(input)?;
    Ok((&input[3..], taken.len() + 3))
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() -> eyre::Result<()> {
    // My solution is to iterate diagonally from south-west to north-east and diagonally from
    // south-east to north-west, and search for "MAS" and "SAM". For each of these found, I record
    // the coordinates of the "A" in the matrix. Then I count the coordinates in common in the
    // SW-NE list and SE-NW list. This corresponds to the number of crossing "MAS"/"SAM".

    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let matrix = read_lines(&args[1])?;
    let mut sw_ne_centers: Vec<Coord> = vec![];
    for coords in matrix.iter_sw_ne().unwrap() {
        let line = matrix.coords_to_string(&coords);
        let mut input = &line[..];
        let mut i: usize = 0;
        while let Ok((new_input, n)) = parse_mas(input) {
            input = new_input;
            i += n;
            sw_ne_centers.push(coords[i - 2]);
        }

        input = &line[..];
        i = 0;
        while let Ok((new_input, n)) = parse_sam(input) {
            input = new_input;
            i += n;
            sw_ne_centers.push(coords[i - 2]);
        }
    }

    let mut se_nw_centers: Vec<Coord> = vec![];
    for coords in matrix.iter_se_nw().unwrap() {
        let line = matrix.coords_to_string(&coords);
        let mut input = &line[..];
        let mut i: usize = 0;
        while let Ok((new_input, n)) = parse_mas(input) {
            input = new_input;
            i += n;
            se_nw_centers.push(coords[i - 2]);
        }

        input = &line[..];
        i = 0;
        while let Ok((new_input, n)) = parse_sam(input) {
            input = new_input;
            i += n;
            se_nw_centers.push(coords[i - 2]);
        }
    }

    assert!(has_unique_elements(&sw_ne_centers));
    assert!(has_unique_elements(&se_nw_centers));

    let mut result = 0;
    for coord1 in &sw_ne_centers {
        for coord2 in &se_nw_centers {
            if coord1 == coord2 {
                result += 1;
                break;
            }
        }
    }

    println!("{}", result);

    Ok(())
}
