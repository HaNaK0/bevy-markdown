//! A crate that enables loading markdown for use in bevy game UIs
//!
//! This crate adds assets and components that can be used to text formatted with markdown and
//! that then is turned into bevys UI nodes which how the markdown text is displayed in game.
//!
//! # under development
//! This plugin is under development and therefore all markdown features are not implemented yet.
//! Here follows a list features and which are implemented
//! ## Basic syntax
//! - [ ] Heading
//! - [ ] Bold
//! - [ ] Italic
//! - [ ] Blockquote
//! - [ ] Ordered List
//! - [ ] Unordered List
//! - [ ] Code
//! - [ ] Horizontal Rule
//! - [ ] Link
//! - [ ] Image
//!
//! ## Advanced syntax
//! - [ ] Table
//! - [ ] Fenced Code Block
//! - [ ] Footnote
//! - [ ] Heading ID
//! - [ ] Definition List
//! - [ ] Strikethrough
//! - [ ] Task List
//! - [ ] Emoji
//! - [ ] Highlight
//! - [ ] Subscript
//! - [ ] Superscript
//!
//! # How to use
//! To use this crate you just add the [MarkdownPlugin] to your bevy app
//! and then add a [MarkdownNodeBundle] which contains a [Markdown] asset to an entity.
//! The markdown asset is loaded like any other asset in bevy except that it requires a meta file for each markdown
//! file because thas where the link to the style file for the bevy asset is linked.
//!
//! A style file is a ron file that contains information about color font an fontsize and could look like this
//! ```ron
//! (
//!     font: "fonts\\Ubuntu\\Ubuntu-Regular.ttf",
//!     body_size: 12.0,
//!     body_color: Srgba((red: 1.0,green: 1.0,blue: 1.0,alpha: 1.0))
//! )
//! ```
//! The style is then linked to a markdown document by adding it to the meta file.
//! ```ron
//! (
//! meta_format_version: "1.0",
//! asset : Load (
//!     loader : "hana_bevy_markdown::markdown_loader::MarkdownLoader",
//!     settings : (
//!         style: "Pages/Home/style.ron"
//!     ),
//! ),
//! )
//! ```
//! Above is an example of a meta file and you need to replace the style with a path to the style file you want to use
use bevy::{
    app::{Plugin, Update},
    asset::{AssetApp, AssetEvent, AssetServer, Assets, Handle},
    log::{debug, error},
    prelude::*,
    text::{BreakLineOn, Text, TextSection, TextStyle},
    ui::Node,
    utils::HashSet,
};
use markdown_asset::Markdown;
use markdown_loader::MarkdownLoader;
use markdown_style::{MarkdownStyle, MarkdownStyleLoader};

pub mod markdown_asset;
pub mod markdown_loader;
pub mod markdown_style;

/// The pugin that enables loading from markdown
pub struct MarkdownPlugin;

impl Plugin for MarkdownPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<Markdown>()
            .init_asset::<MarkdownStyle>()
            .init_asset_loader::<MarkdownLoader>()
            .init_asset_loader::<MarkdownStyleLoader>()
            .add_systems(Update, (on_add, on_asset_event));
    }
}

/// The markdown node bundle that adds markdown to a UI
///
/// Contains a [Markdown] asset, a [MarkdownComponent] and a [NodeBundle]
#[derive(Bundle, Default)]
pub struct MarkdownNodeBundle {
    pub markdown: MarkdownComponent,
    pub markdown_asset: Handle<Markdown>,
    pub node: NodeBundle,
}

/// A component that marks the root of the markdown text
#[derive(Component, Default)]
pub struct MarkdownComponent;

fn on_asset_event(
    mut commands: Commands,
    markdown_assets: Res<Assets<Markdown>>,
    markdown_styles: Res<Assets<MarkdownStyle>>,
    mut load_events: EventReader<AssetEvent<Markdown>>,
    nodes: Query<(Entity, &Handle<Markdown>, &Node)>,
) {
    let loaded_assets: HashSet<AssetId<Markdown>> = load_events
        .read()
        .filter_map(|e| {
            if let AssetEvent::LoadedWithDependencies { id } = e {
                Some(id.clone())
            } else {
                None
            }
        })
        .collect();

    for (entity, asset, _node) in nodes
        .iter()
        .filter(|(_, asset, _)| loaded_assets.contains(&asset.id()))
    {
        debug!("markdown loaded for entity {:?}", entity);
        let markdown = if let Some(markdonw) = markdown_assets.get(asset) {
            markdonw
        } else {
            error!("failed to get the queried markdown asset");
            return;
        };

        let style = if let Some(style) = markdown_styles.get(&markdown.style) {
            style
        } else {
            error!("failed to get the markdown style for the lodaed markdown");
            return;
        };

        commands
            .entity(entity)
            .with_children(|c| build_markdown(c, markdown, style));
    }
}

fn on_add(
    mut commands: Commands,
    markdown_assets: Res<Assets<Markdown>>,
    markdown_styles: Res<Assets<MarkdownStyle>>,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Handle<Markdown>), Added<MarkdownComponent>>,
) {
    for (entity, markdown) in &query {
        debug!("on add triggered");
        if asset_server.is_loaded_with_dependencies(markdown) {
            let markdown = if let Some(markdonw) = markdown_assets.get(markdown) {
                markdonw
            } else {
                error!("failed to get the queried markdown asset");
                return;
            };

            let style = if let Some(style) = markdown_styles.get(&markdown.style) {
                style
            } else {
                error!("could not get style for markdown file, might not be loaded");
                return;
            };

            debug!("markdown built when markdown was added");
            commands
                .entity(entity)
                .with_children(|c| build_markdown(c, markdown, style));
        }
    }
}

fn build_markdown(builder: &mut ChildBuilder, markdown: &Markdown, style: &MarkdownStyle) {
    let body_style: TextStyle = style.into();

    let text_sections = markdown
        .content
        .iter()
        .map(|element| match element {
            markdown_asset::MarkdownElement::Text(text) => TextSection {
                value: text.text.clone(),
                style: body_style.clone(),
            },
            markdown_asset::MarkdownElement::Heading(_, _) => todo!(),
            markdown_asset::MarkdownElement::HorizontalRule => todo!(),
            markdown_asset::MarkdownElement::Image {
                alt_text: _,
                image: _,
            } => todo!(),
            markdown_asset::MarkdownElement::OrderedListItem(_) => todo!(),
            markdown_asset::MarkdownElement::UnorderedListItem(_) => todo!(),
            markdown_asset::MarkdownElement::CodeBlock(_) => todo!(),
            markdown_asset::MarkdownElement::LineBreak => TextSection {
                value: "\n".to_string(),
                style: body_style.clone(),
            },
        })
        .collect();

    builder.spawn(TextBundle {
        text: Text {
            sections: text_sections,
            justify: bevy::text::JustifyText::Left,
            linebreak_behavior: BreakLineOn::WordBoundary,
        },
        ..Default::default()
    });
}
