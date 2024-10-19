use std::path::PathBuf;

use crate::markdown_asset::{parse_markdown, Markdown, MarkdownParseError};
use bevy::{asset::AssetLoader, log::debug, tasks::futures_lite::io::BufReader};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A loader for the the [Markdown] asset
#[derive(Default)]
pub struct MarkdownLoader;

/// Error returned from the [MarkdownLoader]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum MarkdownLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// An [MarkdownParseError]
    #[error("Could not parse markdown {0}")]
    Parse(#[from] MarkdownParseError),
}

/// Settings for [MarkdownLoader]
#[derive(Deserialize, Serialize, Default)]
pub struct MarkdownLoaderSettings {
    style: PathBuf,
}

impl AssetLoader for MarkdownLoader {
    type Asset = Markdown;
    type Settings = MarkdownLoaderSettings;
    type Error = MarkdownLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        debug!("Markdown load started");
        let style = load_context.load(settings.style.clone());

        let buf_reader = BufReader::new(reader);
        let content = parse_markdown(buf_reader).await?;

        debug!("Markdown load finnished");
        Ok(Markdown { content, style })
    }

    fn extensions(&self) -> &[&str] {
        &[".md"]
    }
}
