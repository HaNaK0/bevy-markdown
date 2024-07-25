use crate::markdown_asset::{parse_markdown, Markdown};
use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    tasks::futures_lite::{io::BufReader, AsyncBufReadExt},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct MarkdownLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum MarkdownLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Deserialize, Serialize, Default)]
pub struct MarkdownLoaderSettings {
    style: String,
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
        let buf_reader = BufReader::new(reader);
        parse_markdown(buf_reader).await;

        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &[".md"]
    }
}
