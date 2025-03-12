use super::*;

pub mod in_game;
pub mod menu;

// Global systems go here

fn clean_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive()
    }
}
