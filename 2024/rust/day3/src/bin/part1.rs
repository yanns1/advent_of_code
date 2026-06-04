use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::u32;
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

fn parse_until_m(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_until("m")(input)?;
    Ok((input, ()))
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let mut input = input;
    let mut pairs: Vec<(u32, u32)> = vec![];
    loop {
        if let Ok((new_input, _)) = parse_until_m(input) {
            input = new_input;
        } else {
            break;
        }

        if let Ok((new_input, (x, y))) = parse_mul(input) {
            if n_digits(x) > 3 || n_digits(y) > 3 {
                input = new_input;
                continue;
            }
            pairs.push((x, y));
            input = new_input;
        } else {
            let (new_input, _) = char('m')(input)?;
            input = new_input;
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
