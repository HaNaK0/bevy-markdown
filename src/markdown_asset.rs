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

/// The function that parses the markdown data into a vector of markdown elements
pub(crate) async fn parse_markdown<T>(buffer: T) -> Result<Vec<MarkdownElement>, MarkdownParseError>
where
    T: AsyncBufReadExt,
{
    pin!(buffer);
    let mut lines = buffer.lines();
    Ok(Vec::new())
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

        assert_eq!(result.len(), 2);

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

        assert_eq!(result.len(), 3);

        if let MarkdownElement::Text(text) = result.get(0).unwrap() {
            assert_eq!("First Line", text.text);
            assert_eq!(text.style, MarkdownTextStyle::Standard);
        } else {
            panic!()
        }

        assert_eq!(MarkdownElement::LineBreak, *result.get(1).unwrap());

        if let MarkdownElement::Text(text) = result.get(2).unwrap() {
            assert_eq!("Second Line Third Line Fourth Line", text.text);
            assert_eq!(text.style, MarkdownTextStyle::Standard);
        } else {
            panic!()
        }

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
                text: "Also This".to_string(),
            }),
        ];

        assert_eq!(result, comparison);

        Ok(())
    }
}
