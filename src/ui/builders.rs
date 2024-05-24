use bevy::prelude::{BackgroundColor, BorderColor, BuildChildren, ButtonBundle, ChildBuilder, Color, Display, GridPlacement, Handle, JustifyContent, NodeBundle, RepeatedGridTrack, Style, UiRect, Val};
use bevy::text::Font;
use bevy::utils::default;

use crate::beats::systems::text_bundle;

pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    pub fn new() -> Self {
        StyleBuilder {
            style: Style::default(),
        }
    }

    pub fn gutter_all_px(mut self, gutter: f32) -> Self {
        self.style.row_gap = Val::Px(gutter);
        self.style.column_gap = Val::Px(gutter);
        self
    }

    pub fn centered_content(mut self) -> Self {
        self.style.justify_content =   JustifyContent::Center;
        self
    }

    pub fn fill_parent_height(mut self) -> Self {
        self.style.height = Val::Percent(100.0);
        self
    }

    pub fn flex_columns(mut self, columns: u16, size: f32) -> Self {
        self.style.grid_template_columns = RepeatedGridTrack::flex(columns, size);
        self
    }

    pub fn flex_rows(mut self, rows: u16, size: f32) -> Self {
        self.style.grid_template_rows = RepeatedGridTrack::flex(rows, size);
        self
    }

    pub fn with_grid(mut self) -> Self {
        self.style.display = Display::Grid;
        self
    }

    pub fn span_columns(mut self, cols: u16) -> Self {
        self.style.grid_column = GridPlacement::span(cols);
        self
    }

    pub fn pad_all_px(mut self, padding: f32) -> Self {
        self.style.padding = UiRect::all(Val::Px(padding));
        self
    }

    pub fn width_and_height_in_percent(mut self, width: f32, height: f32) -> Self {
        self.style.width = Val::Percent(width);
        self.style.height = Val::Percent(height);
        self
    }

    pub fn grid_template_columns(mut self, cols: Vec<RepeatedGridTrack>) -> Self {
        self.style.grid_template_columns = cols;
        self
    }
    pub fn grid_template_rows(mut self, rows: Vec<RepeatedGridTrack>) -> Self {
        self.style.grid_template_rows = rows;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.style.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn build(self) -> Style {
        self.style.clone()
    }
}

pub fn add_button<F>(mut child_builder: &mut ChildBuilder, text: &str, font: Handle<Font>, font_size: f32, text_color: Color, button_color: Color, button_border_color: Color, build_fn: F)
    where
        F: FnOnce(StyleBuilder) -> StyleBuilder, {
    child_builder
        .spawn(
            ButtonBundle {
                style: build_fn(StyleBuilder::new()).build(),
                border_color: BorderColor(button_border_color),
                background_color: BackgroundColor(button_color),
                ..default()
            })
        .with_children(|parent| {
            text_bundle(parent, font, text, font_size, text_color);
        });
}

pub struct NodeBundleBuilder {
    node_bundle: NodeBundle,
}

impl NodeBundleBuilder {
    pub fn new() -> Self {
        NodeBundleBuilder {
            node_bundle: NodeBundle {
                style: Style::default(),
                background_color: BackgroundColor::default(),
                ..default()
            }
        }
    }

    pub fn with_style<F>(mut self, build_fn: F) -> Self
        where
            F: FnOnce(StyleBuilder) -> StyleBuilder,
    {
        let builder = StyleBuilder::new();
        let style = build_fn(builder).build();
        self.node_bundle.style = style;
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.node_bundle.background_color = BackgroundColor(color);
        self
    }

    pub fn build(&self) -> NodeBundle {
        self.node_bundle.clone()
    }
}