use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until},
    character::complete::{multispace0, space0},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Markdown<'a> {
    PlainText { body: &'a str },
    UnorderedList { items: Vec<&'a str> },
    Header { level: usize, title: &'a str },
}

fn parse(input: &str) -> IResult<&str, Vec<Markdown>> {
    many0(alt((header, unordered_list, plain_text)))(input)
}

fn header(input: &str) -> IResult<&str, Markdown> {
    let mut parser = tuple((many1(tag("#")), space0, is_not("\n")));
    let (input, (hashtags, _, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Markdown::Header { level, title }))
}

// fn paragraph(input: &str) -> IResult<&str, Markdown> {
//     let parser = separated_list0(unordered_list, plain_text);
//     let (input, items) = delimited(multispace0, parser, multispace0)(input)?;
//     Ok((input, Markdown::Paragraph { items }))
// }

fn unordered_list(input: &str) -> IResult<&str, Markdown> {
    let list_tag = delimited(
        tag("-"),
        preceded(many0(tag(" ")), is_not("\n")),
        take(1usize),
    );
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
