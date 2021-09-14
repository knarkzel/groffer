use nom::{
    branch::alt,
    bytes::complete::{take_until, tag, take},
    character::complete::{digit1, multispace0, space0},
    multi::{many0, many1},
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Markdown<'a> {
    PlainText { body: &'a str },
    Header { level: usize, title: &'a str },
    UnorderedList { items: Vec<&'a str> },
    OrderedList { items: Vec<&'a str> },
    SourceCode { language: &'a str, code: &'a str },
    // Paragraph { items: Vec<Markdown<'a>> },
}

fn parse(input: &str) -> IResult<&str, Vec<Markdown>> {
    many0(alt((
        header,
        unordered_list,
        ordered_list,
        source_code,
        plain_text,
    )))(input)
}

fn header(input: &str) -> IResult<&str, Markdown> {
    let mut parser = separated_pair(many1(tag("#")), space0, take_until("\n"));
    let (input, (hashtags, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Markdown::Header { level, title }))
}

fn ordered_list(input: &str) -> IResult<&str, Markdown> {
    let list_tag = delimited(
        tuple((digit1, tag("."), space0)),
        take_until("\n"),
        take(1usize),
    );
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Markdown::OrderedList { items }))
}

fn unordered_list(input: &str) -> IResult<&str, Markdown> {
    let list_tag = delimited(pair(tag("-"), space0), take_until("\n"), take(1usize));
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Markdown::UnorderedList { items }))
}

fn plain_text(input: &str) -> IResult<&str, Markdown> {
    let (input, body) = delimited(multispace0, take_until("\n\n"), multispace0)(input)?;
    Ok((input, Markdown::PlainText { body }))
}

fn source_code(input: &str) -> IResult<&str, Markdown> {
    let start = pair(multispace0, tag("```"));
    let highlight = terminated(take_until("\n"), multispace0);
    let source = take_until("```");
    let end = pair(tag("```"), multispace0);
    let (input, (language, code)) = delimited(start, pair(highlight, source), end)(input)?;
    Ok((input, Markdown::SourceCode { language, code }))
}

fn main() {
    color_backtrace::install();
    let input = include_str!("../examples/basic.md");
    // let trimmed = format!("{}", input);
    // let trimmed = "\n\nnaoisdnasiodandonasiasndasodasndo\n\n";
    let output = parse(&input);
    println!("{:#?}", output);
}
