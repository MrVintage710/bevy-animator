//=================================================================================
// The animation ID will tell the animator what animation to play on the component.
// This is the trait that you will define for your animation file. For now we only
// handle 2D animations for now but plan for 3D later.
//=================================================================================

use std::marker::PhantomData;

use bevy::{asset::AssetPath, ecs::{query::{QueryData, WorldQuery}, system::SystemParam}, prelude::*};

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
        let Some(asset) = assets.get(handle) else { continue };
        animator.progress += time.delta_seconds() / animator.current_state.duration(asset);
        A::apply(&mut animator, &mut query, asset);
    }
}

//=================================================================================
//    Animation
//=================================================================================

pub trait Animation : Sized {
    
    type AsociatedAsset : Asset;
    
    type Query<'w, 's> : QueryData;
    
    fn apply(
        animator : &mut Animator<Self>, 
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>,
        asset : &Self::AsociatedAsset,
    );
    
    fn spawn<'a>(commands : &mut Commands, file : Handle<Self::AsociatedAsset>);
    
    fn duration(&self, asset : &Self::AsociatedAsset) -> f32;
}

//=================================================================================
//    Animator
//=================================================================================

#[derive(Component)]
pub struct Animator<A : Animation> {
    pub current_state: A,
    pub speed : f32,
    progress : f32,
}

impl <A : Animation + Default> Default for Animator<A> {
    fn default() -> Self {
        Animator {
            current_state : A::default(),
            progress : 0.0,
            speed : 1.0,
        }
    }
}

impl <A : Animation> Animator<A> {
    pub fn new(current_state : A) -> Self {
        Animator {
            current_state,
            progress : 0.0,
            speed : 1.0,
        }
    }
    
    pub fn progress(&self) -> f32 {
        self.progress.fract()
    }
    
    pub fn repititions(&self) -> u32 {
        self.progress.floor() as u32
    }
    
    pub fn total_progress(&self) -> f32 {
        self.progress
    }
}

//=================================================================================
//    AnimationContext
//=================================================================================

#[derive(Clone, Copy)]
pub struct AnimationContext {
    duration : f32,
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

