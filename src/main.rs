use nom::{
    bytes::complete::{tag, take_until},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Header<'a> {
    level: usize,
    title: &'a str,
}

fn header(input: &str) -> IResult<&str, Header> {
    let mut parser = tuple((
        many1(tag("#")),
        many0(tag(" ")),
        take_until("\n"),
    ));
    let (input, (hashtags, _, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Header { level, title }))
}

#[derive(Debug)]
struct Paragraph<'a> {
    body: &'a str,
}

fn paragraph(input: &str) -> IResult<&str, Paragraph> {
    let mut parser = tuple((
        tag("\n\n"),
        take_until("\n\n"),
        many1(tag("\n")),
    ));
    let (input, (_, body, _)) = parser(input)?;
    Ok((input, Paragraph { body }))
}

fn main() {
    let mut parse = tuple((
        header, paragraph
    ));
    let output = parse("### This is a header\n\nThis is a paragraph\nI like paragraph\n\n");
    println!("{:?}", output);
}
