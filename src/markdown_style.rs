use std::path::PathBuf;

use bevy::{
    asset::{Asset, AssetLoader, AsyncReadExt, Handle},
    color::Color,
    log::debug,
    reflect::TypePath,
    text::{Font, TextStyle},
};
use ron::de;
use serde::{Deserialize, Serialize};
use thiserror::Error;

type TextSize = f32;

/// An asset used to store the style for a markdown file
#[derive(Asset, TypePath)]
pub struct MarkdownStyle {
    pub font: Handle<Font>,
    pub body_size: TextSize,
    pub text_color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
struct StyleRon {
    font: PathBuf,
    body_size: TextSize,
    body_color: Color,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum MarkdownStyleError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [SpannedError](ron::error)
    #[error("Could deserialize ron: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

#[derive(Default)]
pub struct MarkdownStyleLoader {}

impl AssetLoader for MarkdownStyleLoader {
    type Asset = MarkdownStyle;
    type Settings = ();
    type Error = MarkdownStyleError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let ron_data = de::from_bytes::<StyleRon>(&bytes)?;

        let font = load_context.load(ron_data.font);

        debug!("Markdown style loaded");

        Ok(MarkdownStyle {
            font,
            body_size: ron_data.body_size,
            text_color: ron_data.body_color,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["*.style.ron"]
    }
}

impl From<TextStyle> for MarkdownStyle {
    fn from(value: TextStyle) -> Self {
        Self {
            font: value.font,
            body_size: value.font_size,
            text_color: value.color,
        }
    }
}

impl Into<TextStyle> for &MarkdownStyle {
    fn into(self) -> TextStyle {
        TextStyle {
            font: self.font.clone(),
            font_size: self.body_size,
            color: self.text_color,
        }
    }
}
