use bevy::prelude::{AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, Direction, Display, FlexDirection, FlexWrap, GlobalTransform, GridAutoFlow, GridPlacement, GridTrack, InheritedVisibility, JustifyContent, JustifyItems, JustifySelf, Node, NodeBundle, Overflow, PositionType, RepeatedGridTrack, Style, Transform, UiRect, Val, ViewVisibility, Visibility, ZIndex};
use bevy::ui::FocusPolicy;
use bevy::utils::default;

pub struct StyleBuilder {
    pub display: Display,
    pub width: Val,
    pub height: Val,
    pub grid_template_rows: Vec<RepeatedGridTrack>,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        StyleBuilder {
            display: Default::default(),
            width: Default::default(),
            height: Default::default(),
            grid_template_rows: vec![],
            grid_template_columns: vec![],
        }
    }

    pub fn with_grid(mut self) -> Self {
        self.display = Display::Grid;
        self
    }

    pub fn width_and_height_in_percent(mut self, width: f32, height: f32) -> Self {
        self.width = Val::Percent(width);
        self.height = Val::Percent(height);
        self
    }

    pub fn grid_template_columns(mut self, cols: Vec<RepeatedGridTrack>) -> Self {
        self.grid_template_columns = cols;
        self
    }
    pub fn grid_template_rows(mut self, rows: Vec<RepeatedGridTrack>) -> Self {
        self.grid_template_rows = rows;
        self
    }

    pub fn build(self) -> Style {
        Style {
            display: self.display,
            width: self.width,
            height: self.height,
            grid_template_rows: self.grid_template_rows,
            grid_template_columns: self.grid_template_columns,
            ..default()
        }
    }
}

pub struct NodeBundleBuilder {
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl NodeBundleBuilder {
    pub fn new() -> Self {
        NodeBundleBuilder {
            node: Node::default(),
            style: Style::default(),
            background_color: BackgroundColor::default(),
            border_color: BorderColor::default(),
            focus_policy: FocusPolicy::Block,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::default(),
        }
    }

    pub fn with_style<F>(mut self, build_fn: F) -> Self
        where
            F: FnOnce(StyleBuilder) -> StyleBuilder,
    {
        let builder = StyleBuilder::new();
        let style = build_fn(builder).build();
        self.style = style;
        self
    }

    pub fn build(&self) -> NodeBundle {
        NodeBundle {
            node: self.node,
            style: self.style.clone(),
            background_color: self.background_color,
            border_color: self.border_color,
            focus_policy: self.focus_policy,
            transform: self.transform,
            global_transform: self.global_transform,
            visibility: self.visibility,
            inherited_visibility: self.inherited_visibility,
            view_visibility: self.view_visibility,
            z_index: self.z_index,
        }
    }
}