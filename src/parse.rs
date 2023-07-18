use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u16, u8};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug)]
pub enum InputType {
    TempoChange(u16),
    TimeSignatureChange(Vec<(u8, u8)>),
    DownbeatToggle(()),
    Quit(()),
    Help(()),
}

pub fn parse_individual_time_signature(input: &str) -> IResult<&str, (u8, u8)> {
    separated_pair(u8, char('/'), u8)(input)
}

pub fn parse_input(input: &str) -> IResult<&str, InputType> {
    alt((
        map(parse_time_signature, InputType::TimeSignatureChange),
        map(parse_bpm, InputType::TempoChange),
        map(parse_downbeat_toggle, InputType::DownbeatToggle),
        map(parse_help, InputType::Help),
        map(parse_quit, InputType::Quit),
    ))(input)
}

fn parse_time_signature(i: &str) -> IResult<&str, Vec<(u8, u8)>> {
    // Expected input for a time signature change is "ts 5 4 3 2 1"
    // "ts" followed by a space separated string of numbers
    let (time_signature, _) = tag("ts ")(i)?;
    separated_list0(tag(" "), parse_individual_time_signature)(time_signature)
}

fn parse_quit(i: &str) -> IResult<&str, ()> {
    map(alt((tag("q"), tag("quit"), tag("exit"))), |_: &str| ())(i)
}

fn parse_bpm(i: &str) -> IResult<&str, u16> {
    preceded(tag("bpm "), u16)(i)
}

fn parse_downbeat_toggle(i: &str) -> IResult<&str, ()> {
    map(alt((tag("db"), tag("downbeat"))), |_: &str| ())(i)
}

fn parse_help(i: &str) -> IResult<&str, ()> {
    map(alt((tag("h"), tag("help"))), |_: &str| ())(i)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1() {
        let x = 100 << 1;
        println!("{x:?}");
    }
}
