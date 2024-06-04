//=================================================================================
// The animation ID will tell the animator what animation to play on the component.
// This is the trait that you will define for your animation file. For now we only
// handle 2D animations for now but plan for 3D later.
//=================================================================================

use std::marker::PhantomData;

use bevy::{ecs::{query::{QueryData, WorldQuery}, system::SystemParam}, prelude::*};

//=================================================================================
//    Animation Plugin
//=================================================================================

pub struct AnimationPlugin<A : Animation + Send + Sync + 'static>(PhantomData<A>);

impl <A : Animation + Send + Sync + 'static> Default for AnimationPlugin<A> {
    fn default() -> Self {
        AnimationPlugin(PhantomData)
    }
}

impl <A : Animation + Send + Sync + 'static> Plugin for AnimationPlugin<A> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, update_animators::<A>)
        ;
    }
}

//=================================================================================
//    Animation Systems
//=================================================================================

pub fn update_animators<A : Animation + Send + Sync + 'static>(
    mut animators : Query<(&mut Animator<A>, A::Query<'_, '_>, &Handle<A::AsociatedAsset>)>,
    assets : Res<Assets<A::AsociatedAsset>>,
    time : Res<Time>,
) {
    for (mut animator, mut query, handle) in animators.iter_mut() {
        animator.animation_context.progress += time.delta_seconds() / animator.animation_context.length;
        let asset = assets.get(handle).expect("There was an error while animating. Asset doesn't exist.");
        let context = animator.animation_context;
        animator.current_state.apply(&mut query, asset, &context);
    }
}

//=================================================================================
//    Animation
//=================================================================================

pub trait Animation {
    
    type AsociatedAsset : Asset;
    
    type Query<'w, 's> : QueryData;
    
    fn apply(
        &mut self, 
        // items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>,
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>,
        asset : &Self::AsociatedAsset,
        animation_context : &AnimationContext
    );
}

//=================================================================================
//    Animator
//=================================================================================

#[derive(Component)]
pub struct Animator<A : Animation> {
    current_state: A,
    animation_context : AnimationContext,
}

//=================================================================================
//    AnimationContext
//=================================================================================

#[derive(Clone, Copy)]
pub struct AnimationContext {
    length : f32,
    progress : f32,
}

impl AnimationContext {
    pub fn get_progress(&self) -> f32 {
        self.progress.fract()
    }
    
    pub fn get_repititions(&self) -> u32 {
        self.progress.floor() as u32
    }
    
    pub fn get_total_progress(&self) -> f32 {
        self.progress
    }
}

