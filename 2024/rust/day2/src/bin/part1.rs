use nom::IResult;
use nom::Parser;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::character::complete::u32;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use std::env;
use std::fs;

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let parse_record = terminated(separated_list1(space1, u32), newline);
    let (input, records) = many1(parse_record).parse(input)?;
    Ok((input, records))
}

fn record_is_safe(record: &[u32]) -> bool {
    // The idea is to iterator over overlapping pairs within the record, and for each pair, check
    // that the distance is between 1 and 3, and check that the variation keeps being the same
    // (always increasing or always decreasing).
    let mut increasing: Option<bool> = None;
    for pair in record.windows(2) {
        let diff = pair[1].abs_diff(pair[0]);

        if diff < 1 || diff > 3 {
            return false;
        }

        let new_increasing = pair[0] < pair[1];
        if let Some(increasing) = increasing {
            if new_increasing != increasing {
                return false;
            }
        }
        increasing = Some(new_increasing)
    }

    true
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let input = fs::read_to_string(&args[1])?;

    let (_, records) = parse(&input[..]).map_err(|e| e.to_owned())?;

    println!(
        "{}",
        records
            .iter()
            .filter(|record| record_is_safe(record))
            .count()
    );

    Ok(())
}
