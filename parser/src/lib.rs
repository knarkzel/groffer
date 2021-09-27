use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{digit1, multispace0, space0},
    combinator::{map, not},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub enum Block<'a> {
    HorizontalRule,
    Header { level: usize, title: &'a str },
    Equation { body: &'a str },
    SourceCode { language: &'a str, code: &'a str },
    UnorderedList { items: Vec<&'a str> },
    OrderedList { items: Vec<&'a str> },
    Quote { items: Vec<&'a str> },
    Line { items: Vec<Inline> },
}

pub fn parse(input: &str) -> Vec<Block> {
    let (_, ast) = parse_block(input).expect("Failure occured when parsing!");
    ast.into_iter()
        .filter(|block| match block {
            Block::Line { items } => items.len() > 0,
            _ => true,
        })
        .collect::<Vec<_>>()
}

fn parse_block(input: &str) -> IResult<&str, Vec<Block>> {
    many1(alt((
        header,
        horizontal_rule,
        unordered_list,
        ordered_list,
        source_code,
        equation,
        quote,
        parse_text,
    )))(input)
}

fn quote(input: &str) -> IResult<&str, Block> {
    let trash = tuple((multispace0, tag(">"), space0));
    let quotes = delimited(trash, take_until("\n"), multispace0);
    let (input, items) = many1(quotes)(input)?;
    Ok((input, Block::Quote { items }))
}

fn header(input: &str) -> IResult<&str, Block> {
    let mut parser = separated_pair(many1(tag("#")), space0, take_until("\n"));
    let (input, (hashtags, title)) = parser(input)?;
    let level = hashtags.len();
    Ok((input, Block::Header { level, title }))
}

fn ordered_list(input: &str) -> IResult<&str, Block> {
    let list_tag = delimited(
        tuple((digit1, alt((tag("."), tag(")"))), space0)),
        take_until("\n"),
        take(1usize),
    );
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Block::OrderedList { items }))
}

fn unordered_list(input: &str) -> IResult<&str, Block> {
    let list_tag = delimited(
        pair(alt((tag("-"), tag("*"))), space0),
        take_until("\n"),
        take(1usize),
    );
    let (input, items) = many1(list_tag)(input)?;
    Ok((input, Block::UnorderedList { items }))
}

fn horizontal_rule(input: &str) -> IResult<&str, Block> {
    let (input, _) = delimited(multispace0, tag("---"), multispace0)(input)?;
    Ok((input, Block::HorizontalRule))
}

fn equation(input: &str) -> IResult<&str, Block> {
    let start = tuple((multispace0, tag("$$$"), multispace0));
    let source = take_until("$$$");
    let end = pair(tag("$$$"), multispace0);
    let (input, body) = delimited(start, source, end)(input)?;
    Ok((input, Block::Equation { body }))
}

fn source_code(input: &str) -> IResult<&str, Block> {
    let start = pair(multispace0, tag("```"));
    let highlight = terminated(take_until("\n"), multispace0);
    let source = take_until("```");
    let end = pair(tag("```"), multispace0);
    let (input, (language, code)) = delimited(start, pair(highlight, source), end)(input)?;
    Ok((input, Block::SourceCode { language, code }))
}

#[derive(Debug)]
pub enum Inline {
    Plain { line: String },
    Italic { line: String },
    Bold { line: String },
    Pre { line: String },
    Math { line: String },
    Link { name: String, url: String },
    Image { name: String, url: String },
}

fn parse_text(input: &str) -> IResult<&str, Block> {
    let (input, items) = terminated(many0(parse_inline), tag("\n"))(input)?;
    Ok((input, Block::Line { items }))
}

fn parse_inline(input: &str) -> IResult<&str, Inline> {
    alt((image, link, pre, math, bold, italics, plain))(input)
}

fn plain(input: &str) -> IResult<&str, Inline> {
    let (input, line) = map(
        many1(preceded(
            not(alt((
                tag("$"),
                tag("!["),
                tag("["),
                tag("`"),
                tag("*"),
                tag("\n"),
            ))),
            take(1u8),
        )),
        |letters| letters.join(""),
    )(input)?;
    Ok((input, Inline::Plain { line }))
}

fn math(input: &str) -> IResult<&str, Inline> {
    let (input, line) = delimited(tag("$"), take_until("$"), tag("$"))(input)?;
    Ok((
        input,
        Inline::Math {
            line: line.to_string(),
        },
    ))
}

fn link(input: &str) -> IResult<&str, Inline> {
    let (input, (name, url)) = link_image(input)?;
    Ok((
        input,
        Inline::Link {
            name: name.to_string(),
            url: url.to_string(),
        },
    ))
}

fn image(input: &str) -> IResult<&str, Inline> {
    let (input, (name, url)) = preceded(tag("!"), link_image)(input)?;
    Ok((
        input,
        Inline::Image {
            name: name.to_string(),
            url: url.to_string(),
        },
    ))
}

fn link_image(input: &str) -> IResult<&str, (&str, &str)> {
    let link = delimited(tag("["), take_until("]"), tag("]"));
    let href = delimited(tag("("), take_until(")"), tag(")"));
    let (input, part) = pair(link, href)(input)?;
    Ok((input, part))
}

fn pre(input: &str) -> IResult<&str, Inline> {
    let (input, line) = delimited(tag("`"), take_until("`"), tag("`"))(input)?;
    Ok((
        input,
        Inline::Pre {
            line: line.to_string(),
        },
    ))
}

fn italics(input: &str) -> IResult<&str, Inline> {
    let (input, line) = delimited(tag("*"), take_until("*"), tag("*"))(input)?;
    Ok((
        input,
        Inline::Italic {
            line: line.to_string(),
        },
    ))
}

fn bold(input: &str) -> IResult<&str, Inline> {
    let (input, line) = delimited(tag("**"), take_until("**"), tag("**"))(input)?;
    Ok((
        input,
        Inline::Bold {
            line: line.to_string(),
        },
    ))
}
