use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Markdown<'a> {
    Header { level: usize, title: &'a str },
    Paragraph { body: &'a str },
}

fn parse(input: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((header, paragraph)))(input)
}

fn header(input: &str) -> IResult<&str, Markdown> {
    let mut parser = tuple((many1(tag("#")), many0(tag(" ")), take_until("\n")));
    let (input, (hashtags, _, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Markdown::Header { level, title }))
}

fn paragraph(input: &str) -> IResult<&str, Markdown> {
    let mut parser = tuple((tag("\n\n"), is_not("\n\n"), tag("\n\n")));
    let (input, (_, body, _)) = parser(input)?;
    Ok((input, Markdown::Paragraph { body }))
}

fn main() {
    color_backtrace::install();
    let input = include_str!("../examples/basic.md");
    let trimmed = format!("{}\n", input);
    let output = parse(&trimmed).unwrap();
    println!("{:#?}", output);
}
