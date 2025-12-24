use std::{fmt, io};

use thiserror::Error;

use color::*;
mod color
{
    use std::fmt;

    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const BLUE: &str = "\x1b[34m";
    pub const YELLOW: &str = "\x1b[33m";

    pub fn emit_red<T: fmt::Debug>(value: T)
    {
        let output = format!("{value:?}");
        println!("{RED}{}{RESET}", output.trim_matches('"'));
    }

    pub fn emit_green<T: fmt::Debug>(value: T)
    {
        let output = format!("{value:?}");
        println!("{GREEN}{}{RESET}", output.trim_matches('"'));
    }

    pub fn emit_blue<T: fmt::Debug>(value: T)
    {
        let output = format!("{value:?}");
        println!("{BLUE}{}{RESET}", output.trim_matches('"'));
    }

    pub fn emit_yellow<T: fmt::Debug>(value: T)
    {
        let output = format!("{value:?}");
        println!("{YELLOW}{}{RESET}", output.trim_matches('"'));
    }
}

#[derive(Debug, Error)]
enum GlintErr
{
    #[error(transparent)]
    IoErr(#[from] io::Error),

    #[error(transparent)]
    RegexErr(#[from] regex::Error),

    #[error(transparent)]
    SourceErr(#[from] SourceErr),
}

#[derive(Debug, Error)]
enum SourceErr
{
    #[error("invalid syntax")]
    InvalidSyntax
    {
        filepath: String, sourceline: String
    },
}

// #[derive(Debug, Error)]
// pub enum LexErr
// {
//     #[error("invalid token {0}")]
//     InvalidToken(char),

//     #[error("unexpected character {0}")]
//     UnexpectedCharacter(char),

//     #[error("unexpected end of character stream")]
//     EndOfStream,
// }

use regex::{Error, Regex};

fn init() -> Result<(), GlintErr>
{
    println!("Hello, world!");

    let filepath = std::env::args().nth(1).unwrap();
    let regex = Regex::new(r"( *)(a|b)")?;

    for line in std::fs::read_to_string(&filepath)?.lines()
    {
        emit_green(format!("{:?}", regex.find(line)));
        let captures = regex.captures(line).ok_or(GlintErr::SourceErr(
            SourceErr::InvalidSyntax {
                filepath: filepath.to_string(),
                sourceline: line.to_string(),
            },
        ))?;
        let (_full, [indent, content]) = captures.extract();
        emit_blue(format!("    Spaces: `{indent:?}` Source: `{content:?}`"));
    }

    Ok(())
}

fn main()
{
    if let Err(err) = init()
    {
        emit_red(err);
    }
}
