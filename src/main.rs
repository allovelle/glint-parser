use std::{fmt, io};

use thiserror::Error;

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
    let regex = Regex::new(r"( +)(a|b)")?;

    for line in std::fs::read_to_string(&filepath)?.lines()
    {
        println!("{:?}", regex.find(line));
        let captures = regex.captures(line).ok_or(GlintErr::SourceErr(
            SourceErr::InvalidSyntax {
                filepath: filepath.to_string(),
                sourceline: line.to_string(),
            },
        ))?;
        println!("captures: {captures:?}");
        let (_full, [indent, content]) = captures.extract();
        println!(
            "Got these captures: indent: {indent:?}, content: {content:?}"
        );
    }

    Ok(())
}

fn main()
{
    report(init());
}

fn report<T: fmt::Debug>(result: Result<T, GlintErr>)
{
    println!("{:?}", result);

    use ariadne::Color;
    use ariadne::*;

    if let Err(err) = result
    {
        match err
        {
            GlintErr::IoErr(error) => todo!(),
            GlintErr::RegexErr(error) => todo!(),
            GlintErr::SourceErr(SourceErr::InvalidSyntax {
                filepath,
                sourceline,
            }) =>
            {
                let path = filepath.as_str();
                let code = sourceline.as_str();

                let mut report =
                    Report::build(ReportKind::Error, (path, 0 .. 0));

                // report = report.with_config(Config::default().with_compact(false));
                // report = report.with_code(200123);
                report =
                    report.with_message(format!("What are you {}?", "doing"));

                report = report.with_label(
                    Label::new((code, 0 .. code.len()))
                        .with_color(Color::Red)
                        .with_message("invalid syntax:"),
                );
                // report = report.with_label(
                //     Label::new((path, 8 .. 10))
                //         .with_color(Color::Green)
                //         .with_message("A"),
                // );
                // report = report.with_label(
                //     Label::new((path, 13 .. 15))
                //         .with_color(Color::Green)
                //         .with_message("B"),
                // );
                // report = report.with_label(
                //     Label::new((path, 19 .. 21))
                //         .with_color(Color::Green)
                //         .with_message("C"),
                // );
                // report = report.with_note("There is probably something");

                let err_report = report.finish();
                let source = Source::from(path);
                err_report.print((path, source)).unwrap();
            }
        }
    }
}
