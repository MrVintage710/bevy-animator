
use bevy::prelude::*;
use bevy_animator::AnimatorPlugin;

pub fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatorPlugin)
    
        .add_systems(Startup, initialize)
    ;
}

pub fn initialize(
    mut commands : Commands
) {
    
    commands.spawn(Camera2dBundle::default());
}