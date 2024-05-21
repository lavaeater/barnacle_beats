use crate::beats::data::{Condition, Effect, Fact, Rule, Story, StoryBeat, StringHashSet};
use nom::character::complete::alphanumeric1;
use nom::error::Error;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alpha1, char, space0, space1},
    combinator::{all_consuming, map, opt},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    alt((
        map(
            tuple((
                tag("IntEquals("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                nom::character::complete::i32,
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::IntEquals {
                fact_name: fact_name.to_string(),
                expected_value,
            },
        ),
        map(
            tuple((
                tag("StringEquals("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                delimited(char('"'), take_until("\""), char('"')),
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::StringEquals {
                fact_name: fact_name.to_string(),
                expected_value: expected_value.to_string(),
            },
        ),
        map(
            tuple((
                tag("BoolEquals("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                alt((tag("true"), tag("false"))),
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::BoolEquals {
                fact_name: fact_name.to_string(),
                expected_value: expected_value == "true",
            },
        ),
        map(
            tuple((
                tag("IntMoreThan("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                nom::character::complete::i32,
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::IntMoreThan {
                fact_name: fact_name.to_string(),
                expected_value,
            },
        ),
        map(
            tuple((
                tag("IntLessThan("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                nom::character::complete::i32,
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::IntLessThan {
                fact_name: fact_name.to_string(),
                expected_value,
            },
        ),
        map(
            tuple((
                tag("ListContains("),
                alphanumeric1::<&str, Error<&str>>,
                tag(", "),
                delimited(char('"'), take_until("\""), char('"')),
                tag(")"),
            )),
            |(_, fact_name, _, expected_value, _)| Condition::ListContains {
                fact_name: fact_name.to_string(),
                expected_value: expected_value.to_string(),
            },
        ),
    ))(input)
}

fn parse_effect(input: &str) -> IResult<&str, Effect> {
    let (input, (_, fact_type, fact_name, fact_value)) = tuple((
        tag("- Effect: SetFact "),
        alphanumeric1,
        space1,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))(input)?;

    let fact = match fact_type {
        "Int" => Fact::Int(fact_name.to_string(), fact_value.parse().unwrap()),
        "String" => Fact::String(fact_name.to_string(), fact_value.to_string()),
        "Bool" => Fact::Bool(fact_name.to_string(), fact_value.parse().unwrap()),
        "StringList" => Fact::StringList(fact_name.to_string(), {
            let mut set = StringHashSet::new();
            set.insert(fact_value.to_string());
            set
        }),
        _ => unimplemented!(),
    };

    Ok((input, Effect::SetFact(fact)))
}


fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (_, _, name, _, conditions)) = tuple((
        tag("- Rule: "),
        space0,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        many1(preceded(
            tuple((space1, tag("- Condition: "))),
            parse_condition,
        )),
    ))(input)?;

    Ok((
        input,
        Rule {
            name: name.to_string(),
            conditions,
        },
    ))
}

fn parse_story_beat(input: &str) -> IResult<&str, StoryBeat> {
    let parse_rule = |input| Ok(("", Rule::new("rule_name".to_string(), vec![]))); // Placeholder for parse_rule

    let (input, (_, _, name, _, rules, effects)) = tuple((
        tag("## StoryBeat: "),
        space0,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        many1(preceded(|input| space1(input), parse_rule)), // Wrap space1 in a closure
        many1(preceded(|input| space1(input), parse_effect)), // Wrap space1 in a closure
    ))(input)?;

    Ok((
        input,
        StoryBeat {
            name: name.to_string(),
            rules,
            effects,
            finished: false,
        },
    ))
}

pub fn parse_story(input: &str) -> IResult<&str, Story> {
    let (input, (_, _, name, _, beats)) = tuple((
        tag("# Story: "),
        space0,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        many1(preceded(space1, parse_story_beat)), // Removed tuple combinator
    ))(input)?;

    Ok((
        input,
        Story {
            name: name.to_string(),
            beats,
            active_beat_index: 0,
        },
    ))
}

// Example usage
fn main() {
    let input = r#"
# Story: MyStory

## StoryBeat: Beat1
- Rule: Rule1
    - Condition: IntEquals(score, 42)
    - Condition: StringEquals(player, "Alice")
    - Condition: BoolEquals(is_alive, true)

## StoryBeat: Beat2
- Rule: Rule2
    - Condition: IntMoreThan(score, 50)
"#;

    match all_consuming(parse_story)(input) {
        Ok((_, story)) => println!("{:#?}", story),
        Err(e) => eprintln!("Error parsing story: {:?}", e),
    }
}
