use std::{collections::btree_set::Range, process::Output};

use bevy::{
    asset::{Asset, Handle},
    reflect::TypePath,
    render::texture::Image,
    tasks::futures_lite::{pin, AsyncBufReadExt, StreamExt},
};
use thiserror::Error;

use crate::markdown_style::MarkdownStyle;

/// The level of a heading set with ammount of `#`
type HeadingLevel = u16;

#[derive(Debug, PartialEq, Eq)]
pub enum MarkdownElement {
    Text(MarkdownText),
    Heading(MarkdownText, HeadingLevel),
    HorizontalRule,
    Image {
        alt_text: String,
        image: Handle<Image>,
    },
    OrderedListItem(MarkdownText),
    UnorderedListItem(MarkdownText),
    CodeBlock(String),
    LineBreak,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MarkdownText {
    style: MarkdownTextStyle,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
enum MarkdownTextStyle {
    Standard,
    Bold,
    Italic,
    Link {
        target: String,
        title: Option<String>,
    },
    Code,
}

#[derive(Asset, TypePath)]
pub struct Markdown {
    content: Vec<MarkdownElement>,
    style: MarkdownStyle,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub(crate) enum MarkdownParseError {
    /// An [IO](std::io) Error
    #[error("Failed reading line: {0}")]
    Io(#[from] std::io::Error),
}

/// Entry point for the parsing of the markdown text
pub(crate) async fn parse_markdown<T>(buffer: T) -> Result<Vec<MarkdownElement>, MarkdownParseError>
where
    T: AsyncBufReadExt,
{
    pin!(buffer);
    let mut lines = buffer.lines();

    let mut output = Vec::new();
    while let Some(line) = lines.next().await {
        let line = line?;

        if !line.is_empty() {
            output = parse_text(&line, output)?;
        } else {
            output = parse_empty_line(&line, output)?;
        }
    }
    Ok(output)
}

/// Parses a text line
fn parse_text(
    line: &str,
    mut output: Vec<MarkdownElement>,
) -> Result<Vec<MarkdownElement>, MarkdownParseError> {
    if line != "" {
        output.push(MarkdownElement::Text(MarkdownText {
            style: MarkdownTextStyle::Standard,
            text: line.trim().to_string(),
        }))
    }

    if line.ends_with("  ") {
        output.push(MarkdownElement::LineBreak)
    }

    Ok(output)
}

/// Parse an empty line and add correct line breaks
fn parse_empty_line(
    _line: &str,
    mut output: Vec<MarkdownElement>,
) -> Result<Vec<MarkdownElement>, MarkdownParseError> {
    let line_breaks = output
        .iter()
        .rev()
        .take(2)
        .take_while(|e| e == &&MarkdownElement::LineBreak)
        .count();

    for _ in 0..(2 - line_breaks) {
        output.push(MarkdownElement::LineBreak)
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::tasks::block_on;

    /// Test if normal text works
    #[test]
    fn test_text() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"hello world";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 1, "should contain 1 element");

        let result = result.first().unwrap();

        if let MarkdownElement::Text(text) = result {
            assert_eq!(text.style, MarkdownTextStyle::Standard);
            assert_eq!(text.text, "hello world");
        } else {
            panic!("result not a regular text block")
        }

        Ok(())
    }

    /// Test italics using asterisks
    /// for example `*hello world*` shoould be *hello world*
    #[test]
    #[ignore = "not implemented"]
    fn test_asterics_italics() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"*hello world*";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();

        if let MarkdownElement::Text(text) = result {
            assert_eq!(text.style, MarkdownTextStyle::Italic);
            assert_eq!(text.text, "hello world");
        } else {
            panic!("result not a regular text block")
        }

        Ok(())
    }

    /// Test italics using underscore
    /// for example `_hello world_` shoould be _hello world_
    #[test]
    #[ignore = "not implemented"]
    fn test_underscore_italics() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"_hello world_";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();

        if let MarkdownElement::Text(text) = result {
            assert_eq!(text.style, MarkdownTextStyle::Italic);
            assert_eq!(text.text, "hello world");
        } else {
            panic!("result not a regular text block")
        }

        Ok(())
    }

    /// Test Bold using underscore
    /// for example `__hello world__` shoould be __hello world__
    #[test]
    #[ignore = "not implemented"]
    fn test_underscore_bold() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"__hello world__";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();

        if let MarkdownElement::Text(text) = result {
            assert_eq!(text.style, MarkdownTextStyle::Bold);
            assert_eq!(text.text, "hello world");
        } else {
            panic!("result not a regular text block")
        }

        Ok(())
    }

    /// Test Bold using underscore
    /// for example `**hello world**` shoould be **hello world**
    #[test]
    #[ignore = "not implemented"]
    fn test_asterisk_bold() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"__hello world__";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();

        if let MarkdownElement::Text(text) = result {
            assert_eq!(text.style, MarkdownTextStyle::Bold);
            assert_eq!(text.text, "hello world");
        } else {
            panic!("result not a regular text block")
        }

        Ok(())
    }

    /// Test headings
    /// Headings should be written starting with hashtags
    #[test]
    #[ignore = "not implemented"]
    fn test_headings() -> Result<(), MarkdownParseError> {
        let input: &[u8] =
            b"# Heading level 1 \n## heading level 2 \n### Heading Level 3 \n##### Heading level 5";
        let result = block_on(parse_markdown(input))?;

        assert_eq!(result.len(), 8);

        for (test_index, test_level) in [(0, 1), (2, 2), (4, 3), (6, 5)] {
            if let MarkdownElement::Heading(text, level) = result.get(test_index).unwrap() {
                assert_eq!(format!("Heading level {}", test_level), text.text);
                assert_eq!(test_level, *level);
                assert_eq!(text.style, MarkdownTextStyle::Standard);
            } else {
                panic!("Not a heading")
            }
        }

        for i in [1, 3, 5, 7] {
            assert_eq!(
                MarkdownElement::LineBreak,
                *result.get(i).unwrap(),
                "every line should end with a line break"
            );
        }

        Ok(())
    }

    /// Testing line breaks  
    /// A normal text should brake on two spaces before a line brake charachter
    #[test]
    fn test_line_breaks() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"First line  \nSecond line \nThird Line\nFourth Line";
        let result = block_on(parse_markdown(input))?;

        let comparison = vec![
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "First line".to_string(),
            }),
            MarkdownElement::LineBreak,
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "Second line".to_string(),
            }),
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "Third Line".to_string(),
            }),
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "Fourth Line".to_string(),
            }),
        ];

        assert_eq!(result, comparison);

        Ok(())
    }

    /// Testing that an empty line results in empty
    ///
    /// Like so
    #[test]
    fn test_empty_line() -> Result<(), MarkdownParseError> {
        let input: &[u8] = b"This should result in an \n\n empty line  \n\n Also this";
        let result = block_on(parse_markdown(input))?;

        let comparison = vec![
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "This should result in an".to_string(),
            }),
            MarkdownElement::LineBreak,
            MarkdownElement::LineBreak,
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "empty line".to_string(),
            }),
            MarkdownElement::LineBreak,
            MarkdownElement::LineBreak,
            MarkdownElement::Text(MarkdownText {
                style: MarkdownTextStyle::Standard,
                text: "Also this".to_string(),
            }),
        ];

        assert_eq!(result, comparison);

        Ok(())
    }
}
