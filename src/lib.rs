use bevy::{
    app::{Plugin, Update},
    asset::{AssetApp, AssetEvent, AssetServer, Assets, Handle},
    color::palettes::css::BLACK,
    log::{debug, error, warn},
    prelude::{
        Added, BuildChildren, Bundle, ChildBuilder, Children, Commands, Component, Entity,
        EventReader, NodeBundle, Query, Ref, Res, TextBundle, Trigger,
    },
    text::{BreakLineOn, Text, TextSection, TextStyle},
    ui::{Node, Style},
    utils::HashSet,
};
use markdown_asset::Markdown;
use markdown_loader::MarkdownLoader;
use markdown_style::{MarkdownStyle, MarkdownStyleLoader};

pub mod markdown_asset;
pub mod markdown_loader;
pub mod markdown_style;

pub struct MarkdownPlugin;

impl Plugin for MarkdownPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<Markdown>()
            .init_asset::<MarkdownStyle>()
            .init_asset_loader::<MarkdownLoader>()
            .init_asset_loader::<MarkdownStyleLoader>()
            .observe(on_load)
            .add_systems(Update, on_add);
    }
}

#[derive(Bundle, Default)]
pub struct MarkdownNodeBundle {
    pub markdown: MarkdownComponent,
    pub markdown_asset: Handle<Markdown>,
    pub node: NodeBundle,
}

#[derive(Component, Default)]
pub struct MarkdownComponent;

fn on_load(
    trigger: Trigger<AssetEvent<Markdown>>,
    mut commands: Commands,
    markdown_assets: Res<Assets<Markdown>>,
    markdown_styles: Res<Assets<MarkdownStyle>>,
    //mut load_events: EventReader<AssetEvent<Markdown>>,
    nodes: Query<(Entity, &Handle<Markdown>, &Node)>,
) {
    let loaded_assset = if let AssetEvent::LoadedWithDependencies { id } = trigger.event() {
        id
    } else {
        return;
    };

    debug!("Load event triggered on {:?}", trigger.entity());

    for (entity, asset, _node) in nodes
        .iter()
        .filter(|(_, asset, _)| asset.id() == *loaded_assset)
    {
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
