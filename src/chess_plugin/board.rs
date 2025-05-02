use bevy::ecs::resource::Resource;

use super::{Color, GameStatus, MoveRequest, Piece, Square};

#[derive(Resource, Default)]
pub struct Board(cozy_chess::Board);

impl Board {
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        self.0.piece_on(square.into()).map(Into::into)
    }
    pub fn color_on(&self, square: Square) -> Option<Color> {
        self.0.color_on(square.into()).map(Into::into)
    }
    pub fn is_legal(&self, mv: MoveRequest) -> bool {
        self.0.is_legal(mv.into())
    }
    pub fn play_unchecked(&mut self, mv: MoveRequest) {
        self.0.play_unchecked(mv.into());
    }
    pub fn status(&self) -> GameStatus {
        self.0.status().into()
    }
    pub fn side_to_move(&self) -> Color {
        self.0.side_to_move().into()
    }

    // TODO: should this be implemented here? or by the user of the plugin? or at most in a utils mod?
    pub fn needs_promotion(&self, mv: MoveRequest) -> bool {
        let mv_with_promotion_is_legal = self.is_legal(MoveRequest {
            from: mv.from,
            to: mv.to,
            promotion: Some(Piece::Queen),
        });
        let mv_without_promotion_is_legal = self.is_legal(MoveRequest {
            from: mv.from,
            to: mv.to,
            promotion: None,
        });

        mv_with_promotion_is_legal && !mv_without_promotion_is_legal && mv.promotion.is_none()
    }
}
