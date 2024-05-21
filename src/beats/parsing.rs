use bevy::utils::HashSet;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list0},
    sequence::{preceded, tuple},
    IResult,
};
use crate::beats::data::{Condition, Effect, Fact, Rule, Story, StoryBeat, StringHashSet};

fn parse_effect(input: &str) -> IResult<&str, Effect> {
    let (input, (_, fact_type, _, fact_name, _, fact_value)) = tuple((
        tag("- Set "),
        alt((tag("Int"), tag("String"), tag("Bool"), tag("StringList"))),
        tag(" "),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        tag(" to "),
        take_while(|c: char| c.is_alphanumeric() || c == ' ' || c == ',' || c == '_'),
    ))(input)?;

    let fact = match fact_type {
        "Int" => {
            let value = fact_value.parse::<i32>().unwrap();
            Fact::Int(fact_name.to_string(), value)
        }
        "String" => Fact::String(fact_name.to_string(), fact_value.to_string()),
        "Bool" => {
            let value = fact_value.parse::<bool>().unwrap();
            Fact::Bool(fact_name.to_string(), value)
        }
        "StringList" => {
            let values = fact_value
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<HashSet<_>>();
            Fact::StringList(fact_name.to_string(), StringHashSet(values))
        }
        _ => unreachable!(),
    };

    Ok((input, Effect::SetFact(fact)))
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, (fact_name, _, operator, _, expected_value)) = tuple((
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        alt((
            tag("=="),
            tag(">"),
            tag("<"),
            tag("contains"),
            tag("equals"),
        )),
        space0,
        take_while(|c: char| c.is_alphanumeric() || c == '_' || c == ' '),
    ))(input)?;

    let condition = match operator {
        "==" => {
            if let Ok(value) = expected_value.parse::<i32>() {
                Condition::IntEquals {
                    fact_name: fact_name.to_string(),
                    expected_value: value,
                }
            } else if let Ok(value) = expected_value.parse::<bool>() {
                Condition::BoolEquals {
                    fact_name: fact_name.to_string(),
                    expected_value: value,
                }
            } else {
                Condition::StringEquals {
                    fact_name: fact_name.to_string(),
                    expected_value: expected_value.to_string(),
                }
            }
        }
        ">" => Condition::IntMoreThan {
            fact_name: fact_name.to_string(),
            expected_value: expected_value.parse::<i32>().unwrap(),
        },
        "<" => Condition::IntLessThan {
            fact_name: fact_name.to_string(),
            expected_value: expected_value.parse::<i32>().unwrap(),
        },
        "contains" => Condition::ListContains {
            fact_name: fact_name.to_string(),
            expected_value: expected_value.to_string(),
        },
        "equals" => Condition::StringEquals {
            fact_name: fact_name.to_string(),
            expected_value: expected_value.to_string(),
        },
        _ => unreachable!(),
    };

    Ok((input, condition))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (_, name, _, conditions)) = tuple((
        tag("### Rule: "),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        many1(preceded(space1, parse_condition)),
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
    let (input, (_, _, name, _, rules, _, effects)) = tuple((
        tag("## StoryBeat: "),
        space0,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        space0,
        many1(preceded(space1, parse_rule)),
        space0,
        many1(preceded(space1, parse_effect)),
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
        many1(preceded(space1, parse_story_beat)),
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