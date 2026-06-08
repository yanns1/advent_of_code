use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, ((before, after), _)) =
        (separated_pair(u32, char('|'), u32), newline).parse(input)?;
    Ok((input, (before, after)))
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, (page_numbers, _)) = (separated_list1(char(','), u32), newline).parse(input)?;
    Ok((input, page_numbers))
}

fn parse(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (input, rules) = many1(parse_rule).parse(input)?;
    let (input, _) = newline(input)?;
    let (input, updates) = many1(parse_update).parse(input)?;
    Ok((input, (rules, updates)))
}

fn is_correctly_ordered(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (n, page_number) in update.iter().enumerate() {
        if let Some(afters) = rules.get(page_number) {
            for i in 0..n {
                for j in 0..afters.len() {
                    if update[i] == afters[j] {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn order_update(update: &mut [u32], rules: &HashMap<u32, Vec<u32>>) {
    update.sort_by(|&n, &m| {
        if let Some(afters) = rules.get(&n) {
            if afters.iter().find(|&&i| i == m).is_some() {
                return Ordering::Less;
            }
        }

        Ordering::Greater
    })
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() >= 2,
        "Should provide the path of the input file as first argument."
    );

    let input = fs::read_to_string(&args[1])?;

    let (_, (rules, mut updates)) = parse(&input[..]).map_err(|e| e.to_owned())?;

    // Make a hash map out of rules where page numbers are keys and for a given key, the value is
    // the set of page numbers that the key must go before.
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (before, after) in rules {
        let afters = rules_map.entry(before).or_insert(vec![]);
        afters.push(after);
    }

    // Iterate over incorrectly ordered updates, order them, and sum their middle page numbers.
    let mut result = 0;
    for update in updates
        .iter_mut()
        .filter(|update| !is_correctly_ordered(update, &rules_map))
    {
        order_update(update, &rules_map);
        result += update[update.len() / 2];
    }
    println!("{}", result);

    Ok(())
}
