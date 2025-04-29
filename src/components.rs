use bevy::ecs::{component::Component, entity::Entity};

#[derive(Clone, Copy, Debug, Component, PartialEq, Eq)]
pub struct Square(pub timecat::Square);

#[derive(Clone, Copy, Debug, Component)]
pub struct Piece;

#[derive(Component)]
#[relationship(relationship_target = TileWith)]
pub struct PieceIn(pub Entity);

// this should be a one-to-one but
// https://github.com/bevyengine/bevy/pull/18087 should be added in bevy 0.17
#[derive(Component)]
#[relationship_target(relationship = PieceIn)]
pub struct TileWith(Vec<Entity>);
