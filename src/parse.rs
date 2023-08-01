use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u16, u8};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

pub enum InputType {
    TempoChange(u16),
    TimeSignatureChange(Vec<(u8, u8)>),
    StartStop,
    DownbeatToggle,
    Quit,
    Help,
}

impl InputType {
    pub fn parse(input: &str) -> IResult<&str, InputType> {
        alt((
            parse_time_signature,
            parse_bpm,
            parse_downbeat_toggle,
            parse_help,
            parse_quit,
            parse_start_stop
        ))(input)
    }
}

fn parse_individual_time_signature(i: &str) -> IResult<&str, (u8, u8)> {
    separated_pair(u8, char('/'), u8)(i)
}

fn parse_time_signatures_to_vec(i: &str) -> IResult<&str, Vec<(u8, u8)>> {
    separated_list0(tag(" "), parse_individual_time_signature)(i)
}

fn parse_time_signature(i: &str) -> IResult<&str, InputType> {
    map(
        preceded(
            alt((tag("ts "), tag("time signature "))),
            parse_time_signatures_to_vec
        ),
        InputType::TimeSignatureChange,
    )(i)
}

fn parse_bpm(i: &str) -> IResult<&str, InputType> {
    map(
        preceded(
            alt((tag("bpm "), tag("tempo "))),
            u16
        ),
        InputType::TempoChange
    )(i)
}

fn parse_downbeat_toggle(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("db"), tag("downbeat"))), |_: &str| {
        InputType::DownbeatToggle
    })(i)
}

fn parse_help(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("h"), tag("help"))), |_: &str| InputType::Help)(i)
}

fn parse_quit(i: &str) -> IResult<&str, InputType> {
    map(alt((tag("q"), tag("quit"), tag("exit"))), |_: &str| {
        InputType::Quit
    })(i)
}

fn parse_start_stop(i: &str) -> IResult<&str, InputType> {
    map(
        tag(""), 
        |_: &str| InputType::StartStop 
    )
    (i)
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_variant(actual:&InputType, expected:&InputType) {
        assert_eq!(std::mem::discriminant(actual), std::mem::discriminant(expected));
    }

    #[test]
    fn test_parse_help(){
        let actual = parse_help("h").unwrap().1;
        let expected = InputType::Help;
        assert_variant(&actual, &expected);

        let actual = parse_help("help").unwrap().1;
        assert_variant(&actual, &expected);
    }

    #[test] 
    fn test_parse_quit(){

        let actual = parse_quit("q").unwrap().1;
        let expected = InputType::Quit;
        assert_variant(&actual, &expected);

        let actual = parse_quit("quit").unwrap().1;
        assert_variant(&actual, &expected);

        let actual = parse_quit("quit").unwrap().1;
        assert_variant(&actual, &expected);
    }

    #[test]
    fn test_parse_downbeat_toggle() {
        let actual = parse_downbeat_toggle("db").unwrap().1;
        let expected = InputType::DownbeatToggle;
        assert_variant(&actual, &expected);

        let actual = parse_downbeat_toggle("downbeat").unwrap().1;
        assert_variant(&actual, &expected);
    }

    #[test]
    fn test_parse_time_signature() {
        let actual = parse_time_signature("ts 4/4").unwrap().1;
        let expected = InputType::TimeSignatureChange(vec![]);
        assert_variant(&actual, &expected);

        let actual = parse_time_signature("time signature 4/4 3/4").unwrap().1;
        assert_variant(&actual, &expected);
    }

    #[test]
    fn test_parse_bpm() {
        let actual = parse_bpm("bpm 200").unwrap().1;
        let expected = InputType::TempoChange(0_u16);
        assert_variant(&actual, &expected);

        let actual = parse_bpm("tempo 200").unwrap().1;
        assert_variant(&actual, &expected);
    }

    #[test]
    fn test_parse_individual_time_signature() {
        let actual = parse_individual_time_signature("3/4").unwrap().1;
        assert_eq!(actual, (3_u8, 4_u8));
    }

    #[test]
    fn test_parse_multiple_time_signature() {
        let actual = parse_time_signatures_to_vec("3/4 4/4 5/4").unwrap().1;
        let expected = vec![
            (3_u8, 4_u8),
            (4_u8, 4_u8),
            (5_u8, 4_u8),
        ];
        assert_eq!(actual, expected);
    }


}
