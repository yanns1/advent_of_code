use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::anychar;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::combinator::value;
use nom::multi::many_till;
use nom::multi::many0;
use nom::sequence::preceded;
use std::env;
use std::fs;

fn n_digits(mut n: u32) -> u32 {
    let mut result = 1;
    while n >= 10 {
        n /= 10;
        result += 1;
    }
    result
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (_, x, _, y, _)) = (tag("mul("), u32, char(','), u32, char(')')).parse(input)?;
    Ok((input, (x, y)))
}

fn parse_do_dont(input: &str) -> IResult<&str, bool> {
    let (input, enabled) =
        alt((value(false, tag("don't")), value(true, tag("do")))).parse(input)?;
    Ok((input, enabled))
}

fn parse(mut input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    // This one was tricky to do with nom. We need to discard everything before matching a mul, do
    // or don't instruction, while maintaining a state (enabled/disabled).
    //
    // My strategy is to discard everything until a 'm' or 'd' is found, then try `parse_mul` or
    // `parse_do_dont` respectively. The use of a while loop, however, forces me to reassign the
    // remaining input to a variable outside the loop; I cannot just shadow a same `input`
    // variable.
    //
    // A "cleaner" way would be to decouple the parsing of instructions from their interpretation.
    // I could put mul, do and don'ts instructions in a list, then do a second pass to process the
    // enabled mul instructions.

    let mut pairs: Vec<(u32, u32)> = vec![];
    let mut enabled = true;

    while !input.is_empty() {
        let (new_input, _) = take_till(|c| c == 'm' || c == 'd')(input)?;
        input = new_input;
        if let Some(c) = input.chars().nth(0) {
            if c == 'm' {
                if let Ok((new_input, (x, y))) = parse_mul(input) {
                    if !enabled || n_digits(x) > 3 || n_digits(y) > 3 {
                        input = new_input;
                        continue;
                    }
                    pairs.push((x, y));
                    input = new_input;
                } else {
                    input = &new_input[1..];
                }
            }
            if c == 'd' {
                if let Ok((new_input, new_enabled)) = parse_do_dont(input) {
                    enabled = new_enabled;
                    input = new_input;
                } else {
                    input = &new_input[1..];
                }
            }
        }
    }

    Ok((input, pairs))
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let input = fs::read_to_string(&args[1])?;

    let (_, pairs) = parse(&input[..]).map_err(|e| e.to_owned())?;
    let result = pairs.iter().fold(0, |acc, (x, y)| acc + x * y);
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::n_digits;

    #[test]
    fn test_n_digits() {
        for i in 0..10 {
            assert_eq!(n_digits(i), 1);
        }
        for i in 10..100 {
            assert_eq!(n_digits(i), 2);
        }
        for i in 100..1000 {
            assert_eq!(n_digits(i), 3);
        }
    }
}
