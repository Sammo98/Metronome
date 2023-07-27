use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u16, u8};
use nom::combinator::{map, map_parser};
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug)]
pub enum InputType {
    TempoChange(u16),
    TimeSignatureChange(Vec<(u8, u8)>),
    DownbeatToggle,
    Quit,
    Help,
}

impl InputType {
    pub fn parse(input:&str) -> IResult<&str, InputType> {
        alt((
            parse_time_signature,
            parse_bpm,
            parse_downbeat_toggle,
            parse_help,
            parse_quit,
        ))(input)
    }
}

fn parse_individual_time_signature(i: &str) -> IResult<&str, (u8, u8)> {
    separated_pair(u8, char('/'), u8)(i)
}

fn parse_time_signature(i: &str) -> IResult<&str, InputType> {
    map(map_parser(tag("ts "), separated_list0(tag(" "), parse_individual_time_signature)), InputType::TimeSignatureChange)(i)
}


fn parse_bpm(i: &str) -> IResult<&str, InputType> {
    map(preceded(tag("bpm "), u16), InputType::TempoChange)(i)
}

fn parse_downbeat_toggle(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("db"), tag("downbeat"))), |_: &str| InputType::DownbeatToggle)(i)
}

fn parse_help(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("h"), tag("help"))), |_: &str| InputType::Help)(i)
}

fn parse_quit(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("q"), tag("quit"), tag("exit"))), |_: &str| InputType::Quit)(i)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
    }
}
