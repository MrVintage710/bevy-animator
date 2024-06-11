//=================================================================================
// The animation ID will tell the animator what animation to play on the component.
// This is the trait that you will define for your animation file. For now we only
// handle 2D animations for now but plan for 3D later.
//=================================================================================

use std::marker::PhantomData;
use bevy::{ecs::query::{QueryData, WorldQuery}, prelude::*};

//=================================================================================
//    Animation Plugin
//=================================================================================

/// This plugin will register the required systems so that an animation of a type will work. Required to be implemented for each animation type.
pub struct AnimationPlugin<A : Animation>(PhantomData<A>);

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

    fn is_unique(&self) -> bool {
        false
    }
}

//=================================================================================
//    Animation Systems
//=================================================================================

/// This system will update all of the animators in the world and apply the animations to the components they are attached to.
pub(crate) fn update_animators<A : Animation + Send + Sync + 'static>(
    mut animators : Query<(&mut Animator<A>, A::Query<'_, '_>, &Handle<A::AsociatedAsset>)>,
    assets : Res<Assets<A::AsociatedAsset>>,
    time : Res<Time>,
) {
    for (mut animator, mut query, handle) in animators.iter_mut() {
        let Some(asset) = assets.get(handle) else { continue };
        animator.progress += time.delta_seconds() / animator.animation.duration(asset) * animator.speed;
        A::apply(&mut animator, &mut query, asset);
    }
}

//=================================================================================
//    Animation
//=================================================================================

/// Defines an animation and how that animation should update the world every tick. Any type that implements this trait can be given state by
/// also implementing the `AnimationState` trait. To spawn an animation, you can use the `spawn_animation` method on the `Commands` struct.
pub trait Animation : Sized {
    
    /// The asset that is associated with this animation. For example, aseprite animations are associated with the aseprite asset.
    type AsociatedAsset : Asset;
    
    /// A query that will allow the animation to effect the component it is attached to.
    type Query<'w, 's> : QueryData;
    
    /// This method defines what the animation should do to the component it is attached to every tick.
    fn apply(
        animator : &Animator<Self>, 
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>,
        asset : &Self::AsociatedAsset,
    );
    
    /// Describes how to spawn the animation in the world. This method is used by the `spawn_animation` method on the `Commands` struct.
    fn spawn(animation : Option<Self>, world: &mut World, path : String, entity : Entity);
    
    /// Given the state of the animation, should return the duration of the animation in seconds.
    fn duration(&self, asset : &Self::AsociatedAsset) -> f32;
}

//=================================================================================
//    Animator
//=================================================================================

/// This is the component that will animate the entity it is attached to based on the embeded animation. 
/// It will hold the progress of the animation.
#[derive(Component)]
pub struct Animator<A : Animation> {
    pub animation: A,
    pub speed : f32,
    progress : f32,
}

impl <A : Animation + Default> Default for Animator<A> {
    fn default() -> Self {
        Animator {
            animation : A::default(),
            progress : 0.0,
            speed : 1.0,
        }
    }
}

impl <A : Animation> Animator<A> {
    /// Creates a new animator with the given animation.
    pub fn new(current_state : A) -> Self {
        Animator {
            animation: current_state,
            progress : 0.0,
            speed : 1.0,
        }
    }
    
    /// Sets the speed of the animation. 1.0 is normal speed, 0.5 is half speed, 2.0 is double speed, etc.
    pub fn set_animation(&mut self, animation : A) {
        self.animation = animation;
    }
    
    /// Sets the animation's progress to 0.0.
    pub fn reset(&mut self) {
        self.progress = 0.0;
    }
    
    /// Gets the progress of the animation. This is a value between 0.0 and 1.0.
    pub fn progress(&self) -> f32 {
        self.progress.fract()
    }
    
    /// Gets the number of repititions the animation has gone through.
    pub fn repititions(&self) -> u32 {
        self.progress.floor() as u32
    }
    
    /// Returns a number that represents the total progress of the animation and the repititions. 
    /// For example, if the animation has gone through 2 repititions and is 50% through the 3rd repitition, this will return 2.5.
    pub fn total_progress(&self) -> f32 {
        self.progress
    }
}

