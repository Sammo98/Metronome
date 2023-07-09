use nom;
use nom::IResult;
use nom::character::complete::{digit1, u16};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::separated_list0;

#[derive(Debug)]
pub enum InputType<'a> {
    TempoChange(u16),
    TimeSignatureChange(Vec<&'a str>),
    DownbeatToggle(&'a str),
    Quit(&'a str),
    Help(&'a str)
}

pub fn parse_input(input:&str) -> IResult<&str, InputType> {
    alt((
        map(parse_time_signature, InputType::TimeSignatureChange),
        map(parse_bpm, InputType::TempoChange),
        map(parse_downbeat_toggle, InputType::DownbeatToggle),
        map(parse_help, InputType::Help),
        map(parse_quit, InputType::Quit),
    ))(input)

}

fn parse_time_signature(i: &str) -> IResult<&str, Vec<&str>> {
    // Expected input for a time signature change is "ts 5 4 3 2 1"
    // "ts" followed by a space separated string of numbers
    let (time_signature, _ ) = tag("ts ")(i)?;
    separated_list0(tag(" "), digit1)(time_signature)

}

fn parse_quit(i: &str) -> IResult<&str, &str> {
    alt((
        tag("q"),
        tag("quit"),
        tag("exit")
    ))(i)
}

fn parse_bpm(i: &str) -> IResult<&str, u16> {
    let (tempo, _) = tag("bpm ")(i)?;
    u16(tempo)
}

fn parse_downbeat_toggle(i: &str) -> IResult<&str, &str> {
    alt((
        tag("db"),
        tag("downbeat")
    ))(i)
}

fn parse_help(i:&str) -> IResult<&str, &str> {
    alt((
        tag("h"),
        tag("help")
    ))(i)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1() {
        let x = parse_input("q");
        println!("{x:?}");
    }
}
