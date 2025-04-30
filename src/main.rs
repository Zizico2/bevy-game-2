mod chess_plugin;
use bevy::prelude::Color;
use bevy::prelude::*;
use chess_plugin::{ALL_SQUARES, ChessPlugin, ColoredPiece, Square, update_pieces};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin, ChessPlugin))
        .insert_resource(SpritePickingSettings {
            picking_mode: SpritePickingMode::BoundingBox,
            ..Default::default()
        })
        .add_systems(Startup, (load_assets, setup.after(load_assets)))
        .add_systems(
            Update,
            update_piece_transforms_and_images.after(update_pieces),
        );

    app.run();
}

const PIECE_SPRITE_SIZE: f32 = 128.0;

#[derive(Resource)]
struct PieceAssets {
    white_pawn: Handle<Image>,
    white_knight: Handle<Image>,
    white_bishop: Handle<Image>,
    white_rook: Handle<Image>,
    white_queen: Handle<Image>,
    white_king: Handle<Image>,

    black_pawn: Handle<Image>,
    black_knight: Handle<Image>,
    black_bishop: Handle<Image>,
    black_rook: Handle<Image>,
    black_queen: Handle<Image>,
    black_king: Handle<Image>,
}
impl PieceAssets {
    fn get_image(&self, piece: chess_plugin::Piece, color: chess_plugin::Color) -> Handle<Image> {
        match piece {
            chess_plugin::Piece::Pawn => match color {
                chess_plugin::Color::White => self.white_pawn.clone(),
                chess_plugin::Color::Black => self.black_pawn.clone(),
            },
            chess_plugin::Piece::Knight => match color {
                chess_plugin::Color::White => self.white_knight.clone(),
                chess_plugin::Color::Black => self.black_knight.clone(),
            },
            chess_plugin::Piece::Bishop => match color {
                chess_plugin::Color::White => self.white_bishop.clone(),
                chess_plugin::Color::Black => self.black_bishop.clone(),
            },
            chess_plugin::Piece::Rook => match color {
                chess_plugin::Color::White => self.white_rook.clone(),
                chess_plugin::Color::Black => self.black_rook.clone(),
            },
            chess_plugin::Piece::Queen => match color {
                chess_plugin::Color::White => self.white_queen.clone(),
                chess_plugin::Color::Black => self.black_queen.clone(),
            },
            chess_plugin::Piece::King => match color {
                chess_plugin::Color::White => self.white_king.clone(),
                chess_plugin::Color::Black => self.black_king.clone(),
            },
        }
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let white_pawn = asset_server.load("white_pawn.png");
    let white_knight = asset_server.load("white_knight.png");
    let white_bishop = asset_server.load("white_bishop.png");
    let white_rook = asset_server.load("white_rook.png");
    let white_queen = asset_server.load("white_queen.png");
    let white_king = asset_server.load("white_king.png");

    let black_pawn = asset_server.load("black_pawn.png");
    let black_knight = asset_server.load("black_knight.png");
    let black_bishop = asset_server.load("black_bishop.png");
    let black_rook = asset_server.load("black_rook.png");
    let black_queen = asset_server.load("black_queen.png");
    let black_king = asset_server.load("black_king.png");

    let piece_assets = PieceAssets {
        white_pawn,
        white_knight,
        white_bishop,
        white_rook,
        white_queen,
        white_king,
        black_pawn,
        black_knight,
        black_bishop,
        black_rook,
        black_queen,
        black_king,
    };
    commands.insert_resource(piece_assets);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for square in ALL_SQUARES {
        let file = square.file();
        let rank = square.rank();
        let x = (u8::from(file) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
        let y = (u8::from(rank) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;

        commands.spawn((
            Pickable {
                is_hoverable: true,
                should_block_lower: false,
            },
            square,
        ));

        // spawn the square
        commands
            .spawn((
                Pickable::default(),
                Sprite::from_color(
                    if (u8::from(rank) + u8::from(file)) % 2 == 0 {
                        Color::srgb(0.1, 0.1, 0.1) // dark color
                    } else {
                        Color::srgb(0.9, 0.9, 0.9) // light color
                    },
                    Vec2::new(PIECE_SPRITE_SIZE, PIECE_SPRITE_SIZE),
                ),
                Transform::from_xyz(x, y, 0.0),
            ))
            .observe(
                move |drop: Trigger<Pointer<DragDrop>>,
                      squares: Query<&Square, With<ColoredPiece>>,
                      mut commands: Commands|
                      -> Result {
                    let Ok(from) = squares.get(drop.dropped) else {
                        // if the dropped entity is not a piece, do nothing
                        return Ok(());
                    };
                    let to = square;

                    // TODO: needs to check if needs promotion
                    commands.trigger(chess_plugin::Move {
                        from: *from,
                        to,
                        promotion: None,
                    });

                    Ok(())
                },
            );
    }
}

fn square_to_transform(square: Square, z: f32) -> Transform {
    let file = square.file();
    let rank = square.rank();
    let x = (u8::from(file) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
    let y = (u8::from(rank) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
    Transform::from_xyz(x, y, z)
}

fn update_piece_transforms_and_images(
    mut piece_transforms: Query<(&Square, Option<&ColoredPiece>, Entity)>,
    mut commands: Commands,
    piece_assets: Res<PieceAssets>,
) -> Result {
    for (square, colored_piece, entity) in piece_transforms.iter_mut() {
        commands.entity(entity).remove::<(Transform, Sprite)>();
        let Some(colored_piece) = colored_piece else {
            continue;
        };
        commands.entity(entity).insert((
            Sprite::from_image(piece_assets.get_image(colored_piece.piece, colored_piece.color)),
            square_to_transform(*square, 1.0),
        ));
    }
    Ok(())
}
