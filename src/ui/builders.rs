use bevy::prelude::{AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, Direction, Display, FlexDirection, FlexWrap, GlobalTransform, GridAutoFlow, GridPlacement, GridTrack, InheritedVisibility, JustifyContent, JustifyItems, JustifySelf, Node, NodeBundle, Overflow, PositionType, RepeatedGridTrack, Style, Transform, UiRect, Val, ViewVisibility, Visibility, ZIndex};
use bevy::ui::FocusPolicy;

pub struct StyleBuilder {
    pub display: Display,
    pub position_type: PositionType,
    pub overflow: Overflow,
    pub direction: Direction,
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    pub aspect_ratio: Option<f32>,
    pub align_items: AlignItems,
    pub justify_items: JustifyItems,
    pub align_self: AlignSelf,
    pub justify_self: JustifySelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub margin: UiRect,
    pub padding: UiRect,
    pub border: UiRect,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Val,
    pub row_gap: Val,
    pub column_gap: Val,
    pub grid_auto_flow: GridAutoFlow,
    pub grid_template_rows: Vec<RepeatedGridTrack>,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
    pub grid_auto_rows: Vec<GridTrack>,
    pub grid_auto_columns: Vec<GridTrack>,
    pub grid_row: GridPlacement,
    pub grid_column: GridPlacement,
}

impl StyleBuilder {
    pub fn new() -> Self {
        StyleBuilder {
            display: Default::default(),
            position_type: Default::default(),
            overflow: Default::default(),
            direction: Default::default(),
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
            bottom: Default::default(),
            width: Default::default(),
            height: Default::default(),
            min_width: Default::default(),
            min_height: Default::default(),
            max_width: Default::default(),
            max_height: Default::default(),
            aspect_ratio: None,
            align_items: Default::default(),
            justify_items: Default::default(),
            align_self: Default::default(),
            justify_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            flex_basis: Default::default(),
            row_gap: Default::default(),
            column_gap: Default::default(),
            grid_auto_flow: Default::default(),
            grid_template_rows: vec![],
            grid_template_columns: vec![],
            grid_auto_rows: vec![],
            grid_auto_columns: vec![],
            grid_row: Default::default(),
            grid_column: Default::default(),
        }
    }

    pub fn build(self) -> Style {
        Style {
            display: self.display,
            position_type: self.position_type,
            overflow: self.overflow,
            direction: self.direction,
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            aspect_ratio: self.aspect_ratio,
            align_items: self.align_items,
            justify_items: self.justify_items,
            align_self: self.align_self,
            justify_self: self.justify_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            margin: self.margin,
            padding: self.padding,
            border: self.border,
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis,
            row_gap: self.row_gap,
            column_gap: self.column_gap,
            grid_auto_flow: self.grid_auto_flow,
            grid_template_rows: self.grid_template_rows,
            grid_template_columns: self.grid_template_columns,
            grid_auto_rows: self.grid_auto_rows,
            grid_auto_columns: self.grid_auto_columns,
            grid_row: self.grid_row,
            grid_column: self.grid_column,
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