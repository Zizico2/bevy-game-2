mod chess_plugin;
mod cursor_style;

mod index_slot_map;

use bevy::{
    dev_tools::fps_overlay::FpsOverlayPlugin,
    ecs::relationship::{RelatedSpawnerCommands, Relationship},
    prelude::*,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};

use chess_plugin::{ALL_SQUARES, Board, ChessPlugin, ColoredPiece, MoveRequest, Piece, Square};
use cursor_style::{CursorContext, OnClick, OnHover};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        FpsOverlayPlugin::default(),
        DefaultPlugins,
        MeshPickingPlugin,
        ChessPlugin,
    ))
    .insert_resource(SpritePickingSettings {
        picking_mode: SpritePickingMode::BoundingBox,
        ..Default::default()
    })
    .insert_resource(CursorContext::init(CursorIcon::System(
        SystemCursorIcon::Default,
    )))
    .add_systems(Startup, (load_assets, setup.after(load_assets)));
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

#[expect(clippy::too_many_arguments)]
fn spawn_promotion_picker_piece<R: Relationship>(
    commands: &mut RelatedSpawnerCommands<R>,
    piece_assets: &PieceAssets,
    moved_piece: Entity,
    current_visibility: Option<Visibility>,
    mv: MoveRequest,
    picker_index: usize,
    picker_piece: Piece,
    picker_color: chess_plugin::Color,
) {
    commands
        .spawn((
            Pickable::default(),
            Sprite::from_image(piece_assets.get_image(picker_piece, picker_color)),
            Transform::from_xyz(
                0.0,
                // TODO: I don't like using "as". There should be a way to convert from usize to f32 cleanly, even if it implies returing a Result
                PIECE_SPRITE_SIZE * 1.5 - PIECE_SPRITE_SIZE * picker_index as f32,
                4.0,
            ),
            OnHover(CursorIcon::System(SystemCursorIcon::Pointer), 0),
        ))
        .observe(
            move |event: Trigger<Pointer<Click>>,
                  parents: Query<&ChildOf>,
                  mut commands: Commands| {
                commands.trigger(MoveRequest {
                    promotion: Some(picker_piece),
                    ..mv
                });

                commands
                    .entity(parents.get(event.target())?.parent())
                    .despawn();

                if let Some(current_visibility) = current_visibility {
                    commands.entity(moved_piece).insert(current_visibility);
                } else {
                    commands.entity(moved_piece).remove::<Visibility>();
                }

                Ok(())
            },
        );
}

fn spawn_promotion_picker(
    piece_assets: &PieceAssets,
    board: &Board,
    moved_piece: Entity,
    visibility: Query<&Visibility>,
    commands: &mut Commands,
    mv: MoveRequest,
) {
    let color = board.side_to_move();

    let Vec2 { x, y } = square_to_xy(mv.to);

    let current_visibility = visibility.get(moved_piece).ok().copied();
    commands.entity(moved_piece).insert(Visibility::Hidden);

    commands
        .spawn((
            Pickable::default(),
            Sprite::from_color(
                Color::srgb(0.5, 0.5, 0.5),
                Vec2::new(PIECE_SPRITE_SIZE, PIECE_SPRITE_SIZE * 4.0),
            ),
            Transform::from_xyz(x, y - PIECE_SPRITE_SIZE * 1.5, 3.0),
        ))
        .with_children(|commands| {
            spawn_promotion_picker_piece(
                commands,
                piece_assets,
                moved_piece,
                current_visibility,
                mv,
                0,
                Piece::Queen,
                color,
            );
            spawn_promotion_picker_piece(
                commands,
                piece_assets,
                moved_piece,
                current_visibility,
                mv,
                1,
                Piece::Rook,
                color,
            );
            spawn_promotion_picker_piece(
                commands,
                piece_assets,
                moved_piece,
                current_visibility,
                mv,
                2,
                Piece::Knight,
                color,
            );
            spawn_promotion_picker_piece(
                commands,
                piece_assets,
                moved_piece,
                current_visibility,
                mv,
                3,
                Piece::Bishop,
                color,
            );
        });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    for square in ALL_SQUARES {
        let file = square.file();
        let rank = square.rank();
        let x = (u8::from(file) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
        let y = (u8::from(rank) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;

        commands
            .spawn((
                Pickable {
                    should_block_lower: false,
                    ..Default::default()
                },
                square,
                square_to_transform(square, 1.0),
                OnHover(CursorIcon::System(SystemCursorIcon::Grab), 0),
                OnClick(CursorIcon::System(SystemCursorIcon::Grabbing), 1),
            ))
            .observe(
                |pressed: Trigger<Pointer<Pressed>>, mut transforms: Query<&mut Transform>| {
                    let mut transform = transforms.get_mut(pressed.target())?;
                    let position = pressed.hit.position.ok_or("need hit position")?;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                    transform.translation.z = 2.0;

                    Ok(())
                },
            )
            .observe(
                |dragged: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
                    let mut transform = transforms.get_mut(dragged.target())?;
                    let delta = dragged.delta;
                    transform.translation.x += delta.x;
                    transform.translation.y -= delta.y;

                    Ok(())
                },
            )
            .observe(
                |pressed: Trigger<Pointer<Released>>,
                 mut transforms: Query<(&mut Transform, &Square)>| {
                    let (mut transform, square) = transforms.get_mut(pressed.target())?;

                    *transform = square_to_transform(*square, 1.0);

                    Ok(())
                },
            )
            .observe(
                |trigger: Trigger<OnAdd, ColoredPiece>,
                 pieces: Query<&ColoredPiece>,
                 mut commands: Commands,
                 piece_assets: Res<PieceAssets>| {
                    let piece = pieces.get(trigger.target())?;
                    commands
                        .entity(trigger.target())
                        .insert((Sprite::from_image(
                            piece_assets.get_image(piece.piece, piece.color),
                        ),));

                    Ok(())
                },
            )
            .observe(
                |trigger: Trigger<OnRemove, ColoredPiece>, mut commands: Commands| {
                    commands.entity(trigger.target()).remove::<Sprite>();
                },
            );

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
                      mut squares: Query<&Square, With<ColoredPiece>>,
                      visibility: Query<&Visibility>,
                      mut commands: Commands,
                      // this doesn't need to be here if needs_promotion is moved into a different system and triggered with an event
                      piece_assets: Res<PieceAssets>,
                      board: Res<Board>| {
                    let Ok(from) = squares.get_mut(drop.dropped) else {
                        // if the dropped entity is not a piece, do nothing
                        return Ok(());
                    };

                    let to = square;

                    let mv = MoveRequest {
                        from: *from,
                        to,
                        promotion: None,
                    };

                    if board.needs_promotion(mv) {
                        spawn_promotion_picker(
                            &piece_assets,
                            &board,
                            drop.dropped,
                            visibility,
                            &mut commands,
                            mv,
                        );

                        return Ok(());
                    }

                    commands.trigger(mv);

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

fn square_to_xy(square: Square) -> Vec2 {
    let file = square.file();
    let rank = square.rank();
    let x = (u8::from(file) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
    let y = (u8::from(rank) as f32) * PIECE_SPRITE_SIZE - PIECE_SPRITE_SIZE * 4.0;
    Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    #[test]
    #[expect(clippy::assertions_on_constants)]
    fn test() {
        assert!(true);
    }
}
