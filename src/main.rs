use anyhow::Context;
use bevy::prelude::Color;
use bevy::prelude::*;
use components::{Piece, PieceIn, Square};
use resources::BoardRes;
use timecat::{ALL_SQUARES, BoardMethodOverload, Move};

mod components;
mod resources;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(SpritePickingSettings {
            picking_mode: SpritePickingMode::BoundingBox,
            ..Default::default()
        })
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    {
        let board_res = BoardRes::default();
        let board = &board_res.0;
        let move_generator = board.generate_legal_moves();
        let moves = move_generator.into_iter().collect::<Vec<_>>();
        dbg!(moves);
        commands.insert_resource(board_res);
    }
    commands.spawn(Camera2d);

    let pawn = asset_server.load("pawn.png");

    // spawn the board tile sprites
    for square in ALL_SQUARES {
        let file = square.get_file();
        let rank = square.get_rank();

        let x = (file.to_int() as f32) * 100.0 - 400.0;
        let y = (rank.to_int() as f32) * 100.0 - 400.0;
        let square = Square(square);
        let tile_entity = commands
            .spawn((
                Sprite::from_color(
                    if (rank.to_int() + file.to_int()) % 2 == 0 {
                        Color::srgb(0.1, 0.1, 0.1) // dark color
                    } else {
                        Color::srgb(0.9, 0.9, 0.9) // light color
                    },
                    Vec2::new(100.0, 100.0),
                ),
                Transform::from_xyz(x, y, 0.0),
                Pickable {
                    is_hoverable: true,
                    should_block_lower: true,
                },
                square,
            ))
            .observe(
                move |drop: Trigger<Pointer<DragDrop>>,
                      mut commands: Commands,
                      pieces: Query<&PieceIn>,
                      squares: Query<&Square>,
                      mut board: ResMut<BoardRes>|
                      -> Result {
                    let piece_in = match pieces.get(drop.dropped) {
                        Ok(piece_in) => piece_in,
                        Err(_) => return Ok(()),
                    };
                    let source = match squares.get(piece_in.0) {
                        Ok(square) => square,
                        Err(_) => return Ok(()),
                    };
                    let dest = match squares.get(drop.target) {
                        Ok(square) => square,
                        Err(_) => return Ok(()),
                    };

                    // this is fallible in the case where source == dest
                    // this should also fail if promotion is not possible
                    let new_move = Move::new(source.0, dest.0, None)?;
                    if board.0.push(new_move).is_err() {
                        return Ok(());
                    }

                    commands.entity(drop.dropped).insert(PieceIn(drop.target));

                    Ok(())
                },
            )
            .id();

        if square == Square(timecat::Square::A2) {
            // spawn a single pawn
            spawn_piece(x, y, &mut commands, tile_entity, pawn.clone());
        } else if square == Square(timecat::Square::B2) {
            // spawn a single pawn
            spawn_piece(x, y, &mut commands, tile_entity, pawn.clone());
        }
    }
}

fn spawn_piece(
    x: f32,
    y: f32,
    commands: &mut Commands,
    tile_entity: Entity,
    piece_image: Handle<Image>,
) {
    commands
        .spawn((
            // Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), Vec2::new(80.0, 80.0)),
            Sprite::from_image(piece_image),
            Transform::from_xyz(x, y, 1.0),
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
            Piece,
            PieceIn(tile_entity),
        ))
        .observe(
            |pressed: Trigger<Pointer<Pressed>>,
             mut piece: Query<(&mut Transform, &Piece)>|
             -> Result {
                let mut piece = piece.get_mut(pressed.target)?;
                let pos = pressed.hit.position.context("no position")?;
                piece.0.translation.x = pos.x;
                piece.0.translation.y = pos.y;
                piece.0.translation.z = 2.0;
                Ok(())
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>| -> Result {
                let mut transform = transforms
                    .get_mut(trigger.target)
                    .context("TODO: can this return an error?")?;
                let drag = trigger.event();

                transform.translation.x += drag.delta.x;
                transform.translation.y -= drag.delta.y;
                Ok(())
            },
        )
        .observe(
            |trigger: Trigger<Pointer<DragEnd>>,
             transforms: Query<&mut Transform>,
             piece_ins: Query<&PieceIn>|
             -> Result { snap_piece_to_tile(trigger.target, transforms, piece_ins) },
        )
        // this is a bit of a hack to make sure the piece snaps to the tile,
        // even if DragEnd doesn't actually run because the drag distance is too small or even zero
        // we should be able to make this better in the future when we're able to order observer systems (which is being worked on in bevy)
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             transforms: Query<&mut Transform>,
             piece_ins: Query<&PieceIn>|
             -> Result { snap_piece_to_tile(trigger.target, transforms, piece_ins) },
        );
}

fn snap_piece_to_tile(
    target: Entity,
    mut transforms: Query<&mut Transform>,
    piece_ins: Query<&PieceIn>,
) -> Result {
    let piece_in = match piece_ins.get(target) {
        Ok(piece_in) => piece_in,
        Err(_) => return Ok(()),
    };

    let tile_translation = transforms.get(piece_in.0)?.translation;

    let mut piece_tr = transforms.get_mut(target)?;

    piece_tr.translation.x = tile_translation.x;
    piece_tr.translation.y = tile_translation.y;
    piece_tr.translation.z = 1.0;

    Ok(())
}
