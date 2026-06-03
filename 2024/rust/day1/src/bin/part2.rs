use std::collections::HashMap;
use nom::IResult;
use nom::Parser;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::character::complete::u32;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use std::env;
use std::fs;

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let parse_line = terminated(separated_pair(u32, space1, u32), newline);
    let (input, pairs) = many1(parse_line).parse(input)?;
    Ok((input, pairs))
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let input = fs::read_to_string(&args[1])?;

    let (_, pairs) = parse(&input).map_err(|e| e.to_owned())?;

    let (left, right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    // The idea is to make a hash map out of the second list, and simply lookup the numbers from the
    // first list to get the multiplication factor.
    let mut hm = HashMap::new();
    for n in right {
        let mult_factor = hm.entry(n).or_insert(0);
        *mult_factor += 1;
    }

    let similarity_score = left.iter().fold(0, |acc, x| acc + x * hm.get(&x).unwrap_or(&0));
    println!("{}", similarity_score);

    Ok(())
}
