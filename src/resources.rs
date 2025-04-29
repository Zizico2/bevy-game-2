use bevy::ecs::resource::Resource;
use timecat::Board;

#[derive(Resource, Default)]
pub struct BoardRes(pub Board);
