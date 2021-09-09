use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until},
    character::complete::{digit1, multispace0, space0},
    multi::{many0, many1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
enum Markdown<'a> {
    PlainText { body: &'a str },
    UnorderedList { items: Vec<&'a str> },
    OrderedList { items: Vec<&'a str> },
    Header { level: usize, title: &'a str },
}

fn parse(input: &str) -> IResult<&str, Vec<Markdown>> {
    many0(alt((header, unordered_list, ordered_list, plain_text)))(input)
}

fn header(input: &str) -> IResult<&str, Markdown> {
    let mut parser = separated_pair(many1(tag("#")), space0, is_not("\n"));
    let (input, (hashtags, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Markdown::Header { level, title }))
}

fn ordered_list(input: &str) -> IResult<&str, Markdown> {
    let list_tag = delimited(
        tuple((digit1, tag("."), space0)),
        is_not("\n"),
        take(1usize),
    );
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Markdown::OrderedList { items }))
}

fn unordered_list(input: &str) -> IResult<&str, Markdown> {
    let list_tag = delimited(pair(tag("-"), space0), is_not("\n"), take(1usize));
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Markdown::UnorderedList { items }))
}

fn plain_text(input: &str) -> IResult<&str, Markdown> {
    let (input, body) = delimited(multispace0, take_until("\n\n"), multispace0)(input)?;
    Ok((input, Markdown::PlainText { body }))
}

fn main() {
    color_backtrace::install();
    let input = include_str!("../examples/basic.md");
    // let trimmed = format!("{}", input);
    // let trimmed = "\n\nnaoisdnasiodandonasiasndasodasndo\n\n";
    let output = parse(&input);
    println!("{:#?}", output);
}
