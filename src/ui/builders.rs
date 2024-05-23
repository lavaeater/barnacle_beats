use bevy::math::AspectRatio;
use bevy::prelude::{AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, Color, Direction, Display, FlexDirection, FlexWrap, GlobalTransform, GridAutoFlow, GridPlacement, GridTrack, InheritedVisibility, JustifyContent, JustifyItems, JustifySelf, Node, NodeBundle, Overflow, PositionType, RepeatedGridTrack, Style, Transform, UiRect, Val, ViewVisibility, Visibility, ZIndex};
use bevy::ui::FocusPolicy;
use bevy::utils::default;

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