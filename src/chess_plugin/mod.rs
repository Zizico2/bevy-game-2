use bevy::prelude::*;
use num_enum::IntoPrimitive;

mod board;
pub use board::*;

pub struct ChessPlugin;

#[derive(Event, Clone, Copy)]
pub struct MoveRequest {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
}

#[derive(Event, Clone, Copy)]
struct PieceUpdateQueued;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>();

        setup_move(app);
        app.add_systems(PostStartup, |mut commands: Commands| {
            commands.trigger(PieceUpdateQueued);
        });
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl From<MoveRequest> for cozy_chess::Move {
    fn from(value: MoveRequest) -> Self {
        cozy_chess::Move {
            from: value.from.into(),
            to: value.to.into(),
            promotion: value.promotion.map(Into::into),
        }
    }
}

impl From<cozy_chess::Piece> for Piece {
    fn from(value: cozy_chess::Piece) -> Self {
        match value {
            cozy_chess::Piece::Pawn => Piece::Pawn,
            cozy_chess::Piece::Knight => Piece::Knight,
            cozy_chess::Piece::Bishop => Piece::Bishop,
            cozy_chess::Piece::Rook => Piece::Rook,
            cozy_chess::Piece::Queen => Piece::Queen,
            cozy_chess::Piece::King => Piece::King,
        }
    }
}
impl From<cozy_chess::Color> for Color {
    fn from(value: cozy_chess::Color) -> Self {
        match value {
            cozy_chess::Color::White => Color::White,
            cozy_chess::Color::Black => Color::Black,
        }
    }
}

fn setup_move(app: &mut App) {
    app.add_observer(
        |_: Trigger<PieceUpdateQueued>,
         mut commands: Commands,
         board: Res<Board>,
         squares: Query<(&Square, Entity)>| {
            for (square, entity) in squares.iter() {
                let piece_on = board.piece_on(*square);
                let color_on = board.color_on(*square);

                match (piece_on, color_on) {
                    (Some(piece), Some(color)) => {
                        commands.entity(entity).remove::<ColoredPiece>();
                        commands
                            .entity(entity)
                            .insert(ColoredPiece { piece, color });
                    }
                    (None, None) => {
                        commands.entity(entity).remove::<ColoredPiece>();
                    }
                    _ => {
                        return Err("piece and color mismatch".into());
                    }
                }
            }
            Ok(())
        },
    )
    .add_observer(
        |event: Trigger<MoveRequest>, mut board: ResMut<Board>, mut commands: Commands| {
            let mv = *event;

            if !board.is_legal(mv) {
                // ignore illegal moves
                return Ok(());
            }
            board.play_unchecked(mv);

            // TODO
            match board.status() {
                GameStatus::Ongoing => {
                    // continue the game
                }
                GameStatus::Drawn => {
                    eprintln!("Drawn!");
                }
                GameStatus::Won => {
                    eprintln!("Checkmate!");
                }
            }

            commands.trigger(PieceUpdateQueued);

            Ok(())
        },
    );
}

pub enum GameStatus {
    Ongoing,
    Drawn,
    Won,
}
impl From<cozy_chess::GameStatus> for GameStatus {
    fn from(value: cozy_chess::GameStatus) -> Self {
        match value {
            cozy_chess::GameStatus::Ongoing => GameStatus::Ongoing,
            cozy_chess::GameStatus::Drawn => GameStatus::Drawn,
            cozy_chess::GameStatus::Won => GameStatus::Won,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl From<&Square> for cozy_chess::Square {
    fn from(value: &Square) -> Self {
        (*value).into()
    }
}

impl From<&Piece> for cozy_chess::Piece {
    fn from(value: &Piece) -> Self {
        (*value).into()
    }
}

impl From<Square> for cozy_chess::Square {
    fn from(value: Square) -> Self {
        match value {
            Square::A1 => cozy_chess::Square::A1,
            Square::B1 => cozy_chess::Square::B1,
            Square::C1 => cozy_chess::Square::C1,
            Square::D1 => cozy_chess::Square::D1,
            Square::E1 => cozy_chess::Square::E1,
            Square::F1 => cozy_chess::Square::F1,
            Square::G1 => cozy_chess::Square::G1,
            Square::H1 => cozy_chess::Square::H1,
            Square::A2 => cozy_chess::Square::A2,
            Square::B2 => cozy_chess::Square::B2,
            Square::C2 => cozy_chess::Square::C2,
            Square::D2 => cozy_chess::Square::D2,
            Square::E2 => cozy_chess::Square::E2,
            Square::F2 => cozy_chess::Square::F2,
            Square::G2 => cozy_chess::Square::G2,
            Square::H2 => cozy_chess::Square::H2,
            Square::A3 => cozy_chess::Square::A3,
            Square::B3 => cozy_chess::Square::B3,
            Square::C3 => cozy_chess::Square::C3,
            Square::D3 => cozy_chess::Square::D3,
            Square::E3 => cozy_chess::Square::E3,
            Square::F3 => cozy_chess::Square::F3,
            Square::G3 => cozy_chess::Square::G3,
            Square::H3 => cozy_chess::Square::H3,
            Square::A4 => cozy_chess::Square::A4,
            Square::B4 => cozy_chess::Square::B4,
            Square::C4 => cozy_chess::Square::C4,
            Square::D4 => cozy_chess::Square::D4,
            Square::E4 => cozy_chess::Square::E4,
            Square::F4 => cozy_chess::Square::F4,
            Square::G4 => cozy_chess::Square::G4,
            Square::H4 => cozy_chess::Square::H4,
            Square::A5 => cozy_chess::Square::A5,
            Square::B5 => cozy_chess::Square::B5,
            Square::C5 => cozy_chess::Square::C5,
            Square::D5 => cozy_chess::Square::D5,
            Square::E5 => cozy_chess::Square::E5,
            Square::F5 => cozy_chess::Square::F5,
            Square::G5 => cozy_chess::Square::G5,
            Square::H5 => cozy_chess::Square::H5,
            Square::A6 => cozy_chess::Square::A6,
            Square::B6 => cozy_chess::Square::B6,
            Square::C6 => cozy_chess::Square::C6,
            Square::D6 => cozy_chess::Square::D6,
            Square::E6 => cozy_chess::Square::E6,
            Square::F6 => cozy_chess::Square::F6,
            Square::G6 => cozy_chess::Square::G6,
            Square::H6 => cozy_chess::Square::H6,
            Square::A7 => cozy_chess::Square::A7,
            Square::B7 => cozy_chess::Square::B7,
            Square::C7 => cozy_chess::Square::C7,
            Square::D7 => cozy_chess::Square::D7,
            Square::E7 => cozy_chess::Square::E7,
            Square::F7 => cozy_chess::Square::F7,
            Square::G7 => cozy_chess::Square::G7,
            Square::H7 => cozy_chess::Square::H7,
            Square::A8 => cozy_chess::Square::A8,
            Square::B8 => cozy_chess::Square::B8,
            Square::C8 => cozy_chess::Square::C8,
            Square::D8 => cozy_chess::Square::D8,
            Square::E8 => cozy_chess::Square::E8,
            Square::F8 => cozy_chess::Square::F8,
            Square::G8 => cozy_chess::Square::G8,
            Square::H8 => cozy_chess::Square::H8,
        }
    }
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u8)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl From<Rank> for cozy_chess::Rank {
    fn from(value: Rank) -> Self {
        match value {
            Rank::First => cozy_chess::Rank::First,
            Rank::Second => cozy_chess::Rank::Second,
            Rank::Third => cozy_chess::Rank::Third,
            Rank::Fourth => cozy_chess::Rank::Fourth,
            Rank::Fifth => cozy_chess::Rank::Fifth,
            Rank::Sixth => cozy_chess::Rank::Sixth,
            Rank::Seventh => cozy_chess::Rank::Seventh,
            Rank::Eighth => cozy_chess::Rank::Eighth,
        }
    }
}

impl Square {
    pub fn file(self) -> File {
        match self {
            Square::A1
            | Square::A2
            | Square::A3
            | Square::A4
            | Square::A5
            | Square::A6
            | Square::A7
            | Square::A8 => File::A,
            Square::B1
            | Square::B2
            | Square::B3
            | Square::B4
            | Square::B5
            | Square::B6
            | Square::B7
            | Square::B8 => File::B,
            Square::C1
            | Square::C2
            | Square::C3
            | Square::C4
            | Square::C5
            | Square::C6
            | Square::C7
            | Square::C8 => File::C,
            Square::D1
            | Square::D2
            | Square::D3
            | Square::D4
            | Square::D5
            | Square::D6
            | Square::D7
            | Square::D8 => File::D,
            Square::E1
            | Square::E2
            | Square::E3
            | Square::E4
            | Square::E5
            | Square::E6
            | Square::E7
            | Square::E8 => File::E,
            Square::F1
            | Square::F2
            | Square::F3
            | Square::F4
            | Square::F5
            | Square::F6
            | Square::F7
            | Square::F8 => File::F,
            Square::G1
            | Square::G2
            | Square::G3
            | Square::G4
            | Square::G5
            | Square::G6
            | Square::G7
            | Square::G8 => File::G,
            Square::H1
            | Square::H2
            | Square::H3
            | Square::H4
            | Square::H5
            | Square::H6
            | Square::H7
            | Square::H8 => File::H,
        }
    }
    pub fn rank(self) -> Rank {
        match self {
            Square::A1
            | Square::B1
            | Square::C1
            | Square::D1
            | Square::E1
            | Square::F1
            | Square::G1
            | Square::H1 => Rank::First,
            Square::A2
            | Square::B2
            | Square::C2
            | Square::D2
            | Square::E2
            | Square::F2
            | Square::G2
            | Square::H2 => Rank::Second,
            Square::A3
            | Square::B3
            | Square::C3
            | Square::D3
            | Square::E3
            | Square::F3
            | Square::G3
            | Square::H3 => Rank::Third,
            Square::A4
            | Square::B4
            | Square::C4
            | Square::D4
            | Square::E4
            | Square::F4
            | Square::G4
            | Square::H4 => Rank::Fourth,
            Square::A5
            | Square::B5
            | Square::C5
            | Square::D5
            | Square::E5
            | Square::F5
            | Square::G5
            | Square::H5 => Rank::Fifth,
            Square::A6
            | Square::B6
            | Square::C6
            | Square::D6
            | Square::E6
            | Square::F6
            | Square::G6
            | Square::H6 => Rank::Sixth,
            Square::A7
            | Square::B7
            | Square::C7
            | Square::D7
            | Square::E7
            | Square::F7
            | Square::G7
            | Square::H7 => Rank::Seventh,
            Square::A8
            | Square::B8
            | Square::C8
            | Square::D8
            | Square::E8
            | Square::F8
            | Square::G8
            | Square::H8 => Rank::Eighth,
        }
    }
}

pub const ALL_SQUARES: [Square; 64] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
];

#[derive(Component, Clone, Copy, PartialEq, Eq)]
#[require(
    Square = explicit::<Piece, Square>()
)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct ColoredPiece {
    pub piece: Piece,
    pub color: Color,
}

impl From<Piece> for cozy_chess::Piece {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Pawn => cozy_chess::Piece::Pawn,
            Piece::Knight => cozy_chess::Piece::Knight,
            Piece::Bishop => cozy_chess::Piece::Bishop,
            Piece::Rook => cozy_chess::Piece::Rook,
            Piece::Queen => cozy_chess::Piece::Queen,
            Piece::King => cozy_chess::Piece::King,
        }
    }
}

fn explicit<Parent, Child>() -> Child {
    panic!(
        "{} must be explicitly set when spawning {}",
        std::any::type_name::<Child>(),
        std::any::type_name::<Parent>()
    );
}
