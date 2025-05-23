use bevy::{
    ecs::component::{ComponentHook, HookContext, Mutable, StorageType},
    prelude::*,
    winit::cursor::CursorIcon,
};
use indexmap::IndexSet;
use slotmap::DefaultKey;

use crate::index_slot_map::PriorotyIndexSlotMap;

#[derive(Resource)]
pub struct CursorContext {
    // cursor_slot_map: SlotMap<DefaultKey, CursorIcon>,
    // cursor_index_set: IndexSet<(usize, DefaultKey)>,
    cursor_map: PriorotyIndexSlotMap<CursorIcon>,
}
#[derive(Component)]
struct OnHoverContext {
    // could just be a single DefaultKey
    cursor: IndexSet<DefaultKey>,
}

#[derive(Component)]
struct OnClickContext {
    // could just be a single DefaultKey
    cursor: IndexSet<DefaultKey>,
}

impl CursorContext {
    pub fn init(cursor: CursorIcon) -> Self {
        // let mut cursor_slot_map = SlotMap::with_capacity(1);
        // let key = cursor_slot_map.insert(cursor.clone());
        // let mut cursor_index_set = IndexSet::with_capacity(1);
        // cursor_index_set.insert((0, key));

        let mut cursor_map = PriorotyIndexSlotMap::with_capacity(1);
        cursor_map.insert_prioritezed(0, cursor);

        Self { cursor_map }
    }
}

// (cursor_icon, priority)
#[derive(Clone)]
pub struct OnHover(pub CursorIcon, pub usize);

impl Component for OnHover {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, HookContext { entity, .. }| {
            // is there a way to make this not panic?
            // this could emit an event instead and then the observer system would handle it,
            // where we can return a Result instead of panicking
            let component = world
                .get::<Self>(entity)
                .expect("could not get component")
                .clone();
            let cursor = component.0.clone();

            let mut commands = world.commands();

            commands
                .get_entity(entity)
                .unwrap()
                // TODO: observe Trigger<Pointer<Cancel>> ?
                .observe(
                    move |ev: Trigger<Pointer<Over>>,
                          window: Single<Entity, With<Window>>,
                          cursor_context: ResMut<CursorContext>,
                          mut entity_cursor_context: Query<&mut OnHoverContext>,
                          mut commands: Commands| {
                        let cursor_context = cursor_context.into_inner();

                        let key = cursor_context
                            .cursor_map
                            .insert_prioritezed(component.1, cursor.clone());

                        let last = cursor_context.cursor_map.last().unwrap();

                        if last == key {
                            commands.entity(*window).insert(cursor.clone());
                        }

                        if let Ok(mut entity_cursor_context) =
                            entity_cursor_context.get_mut(ev.target())
                        {
                            entity_cursor_context.cursor.insert(key);
                        } else {
                            commands.entity(ev.target()).insert(OnHoverContext {
                                cursor: IndexSet::from_iter([key]),
                            });
                        }
                    },
                )
                .observe(
                    move |ev: Trigger<OnRemove, OnHoverContext>,
                          mut cursor_context: ResMut<CursorContext>,
                          entity_cursor_context: Query<&OnHoverContext>| {
                        for entity_cursor_context in entity_cursor_context
                            .get(ev.target())
                            .unwrap()
                            .cursor
                            .iter()
                        {
                            cursor_context
                                .cursor_map
                                .shift_remove(*entity_cursor_context)
                                .unwrap();
                        }
                    },
                )
                .observe(
                    move |ev: Trigger<Pointer<Out>>,
                          window: Single<Entity, With<Window>>,
                          cursor_context: ResMut<CursorContext>,
                          mut entity_cursor_context: Query<&mut OnHoverContext>,
                          mut commands: Commands| {
                        let cursor_context = cursor_context.into_inner();
                        let mut entity_cursor_context =
                            entity_cursor_context.get_mut(ev.target()).ok().unwrap();
                        let cursor = entity_cursor_context.cursor.pop().unwrap();
                        cursor_context.cursor_map.shift_remove(cursor).unwrap();

                        let cursor = cursor_context.cursor_map.last_value().unwrap();

                        commands.entity(*window).insert(cursor.clone());

                        commands.entity(ev.target()).remove::<OnHoverContext>();

                        Ok(())
                    },
                );
        })
    }
}

// (cursor_icon, priority)
#[derive(Clone)]
pub struct OnClick(pub CursorIcon, pub usize);

impl Component for OnClick {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, HookContext { entity, .. }| {
            // is there a way to make this not panic?
            // this could emit an event instead and then the observer system would handle it,
            // where we can return a Result instead of panicking
            let component = world
                .get::<Self>(entity)
                .expect("could not get component")
                .clone();
            let cursor = component.0.clone();

            let mut commands = world.commands();

            commands
                .get_entity(entity)
                .unwrap()
                .observe(
                    move |ev: Trigger<Pointer<Pressed>>,
                          window: Single<Entity, With<Window>>,
                          cursor_context: ResMut<CursorContext>,
                          mut entity_cursor_context: Query<&mut OnClickContext>,
                          mut commands: Commands| {
                        let cursor_context = cursor_context.into_inner();
                        let key = cursor_context
                            .cursor_map
                            .insert_prioritezed(component.1, cursor.clone());

                        commands.entity(*window).insert(cursor.clone());

                        if let Ok(mut entity_cursor_context) =
                            entity_cursor_context.get_mut(ev.target())
                        {
                            entity_cursor_context.cursor.insert(key);
                        } else {
                            commands.entity(ev.target()).insert(OnClickContext {
                                cursor: IndexSet::from_iter([key]),
                            });
                        }
                    },
                )
                .observe(
                    move |ev: Trigger<Pointer<Released>>,
                          window: Single<Entity, With<Window>>,
                          cursor_context: ResMut<CursorContext>,
                          mut entity_cursor_context: Query<&mut OnClickContext>,
                          mut commands: Commands| {
                        let cursor_context = cursor_context.into_inner();
                        let Ok(mut entity_cursor_context) =
                            entity_cursor_context.get_mut(ev.target())
                        else {
                            return Ok(());
                        };
                        let cursor = entity_cursor_context.cursor.pop().unwrap();
                        cursor_context.cursor_map.shift_remove(cursor).unwrap();

                        let cursor = cursor_context.cursor_map.last_value().unwrap();

                        commands.entity(*window).insert(cursor.clone());

                        commands.entity(ev.target()).remove::<OnClickContext>();

                        Ok(())
                    },
                );
        })
    }
}
