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

fn try_record_is_safe(record: &[u32]) -> Result<(), usize> {
    // The idea is to iterator over overlapping pairs within the record, and for each pair, check
    // that the distance is between 1 and 3, and check that the variation keeps being the same
    // (always increasing or always decreasing).
    let mut increasing: Option<bool> = None;
    for (n, pair) in record.windows(2).enumerate() {
        let diff = pair[1].abs_diff(pair[0]);

        if diff < 1 || diff > 3 {
            return Err(n);
        }

        let new_increasing = pair[0] < pair[1];
        if let Some(increasing) = increasing {
            if new_increasing != increasing {
                return Err(n);
            }
        }
        increasing = Some(new_increasing)
    }

    Ok(())
}

fn record_is_safe(record: &[u32]) -> bool {
    // If record is not safe on first pass, try to remove the first element of the problematic
    // pair. If after having removed the first element, the record still is not safe, try removing
    // the second element instead of the first. If the record still is not safe, there is no hope
    // for it being safe.
    //
    // Actually, it does not always work. Take for example the record "43 41 43 44 45 47 49".
    // This algorithm will try removing 41 and the second 43, in each case not giving a safe
    // record. But removing the first 43 gives a safe record. Indeed, I was not sure that an
    // economical approach to removing elements would always work. Perhaps we can do better than
    // trying to remove each element in turn (`record_is_safe2`), but I am not sure.

    if let Err(n) = try_record_is_safe(record) {
        let mut record2 = record.to_vec();
        record2.remove(n);
        if try_record_is_safe(&record2).is_err() {
            let mut record3 = record.to_vec();
            record3.remove(n + 1);
            let res = try_record_is_safe(&record3).is_ok();
            res
        } else {
            true
        }
    } else {
        true
    }
}

fn record_is_safe2(record: &[u32]) -> bool {
    if try_record_is_safe(record).is_ok() {
        return true;
    }

    for i in 0..record.len() {
        let mut new_record = record.to_vec();
        new_record.remove(i);
        if try_record_is_safe(&new_record).is_ok() {
            return true;
        }
    }

    false
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let input = fs::read_to_string(&args[1])?;

    let (_, records) = parse(&input[..]).map_err(|e| e.to_owned())?;

    // println!("{}", records.iter().filter(|record| record_is_safe(record)).count());
    println!(
        "{}",
        records
            .iter()
            .filter(|record| record_is_safe2(record))
            .count()
    );
    // for record in records {
    //     if record_is_safe(&record) != record_is_safe2(&record) {
    //         println!("{:?}", record);
    //     }
    // }

    Ok(())
}
