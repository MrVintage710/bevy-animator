//=================================================================================
// Here we are defining the Aseprite Animation AssetFile, and the Aseprite 
// Animation ID.
//=================================================================================

use asefile::AsepriteFile;
use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::query::WorldQuery, prelude::{Vec2, *}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::ImageSampler}, utils::HashMap};
use btree_range_map::RangeMap;

use crate::animation::{Animation, Animator};

//=================================================================================
//    AsepriteAnimationPlugin
//=================================================================================

pub(crate) struct AsepriteAnimationPlugin;

impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset_loader::<AsepriteLoader>()
            .init_asset::<Aseprite>()
        ;
    }
}

//=================================================================================
//    Aseprite Asset
//=================================================================================

#[derive(Asset, TypePath)]
pub struct Aseprite {
    layout : Handle<TextureAtlasLayout>,
    image : Handle<Image>,
    duration : Vec<u32>,
    anims : HashMap<String, Anim>,
    dimensions : UVec2
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Anim {
    pub frame_map : RangeMap<f32, usize>,
    duration : f32
}

//=================================================================================
//    Aseprite Asset Loader
//=================================================================================

#[derive(Default)]
pub struct AsepriteLoader;

impl AssetLoader for AsepriteLoader {
    type Asset = Aseprite;

    type Settings = ();

    type Error = asefile::AsepriteParseError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let aseprite = AsepriteFile::read(bytes.as_slice())?;
            
            let image_loader = load_context.begin_labeled_asset();
            let layout_loader = load_context.begin_labeled_asset();
            let mut atlas = TextureAtlasBuilder::default();
            let mut frames = Vec::new();
            let mut durations = Vec::new();
            for frame_index in 0..aseprite.num_frames() {
                let frame = aseprite.frame(frame_index);
                let mut image = Image::new(
                    Extent3d {
                       width: aseprite.width() as u32,
                       height: aseprite.height() as u32,
                       depth_or_array_layers: 1,
                   }, 
                   TextureDimension::D2, 
                   frame.image().into_vec(), 
                   TextureFormat::Rgba8UnormSrgb, 
                   RenderAssetUsages::all()
                );
                image.sampler = ImageSampler::nearest();
                durations.push(frame.duration());
                frames.push(image);
            }
            for image in frames.iter() { atlas.add_texture(None, image); }
            let (layout, image) = atlas.finish().expect("Failed to build texture atlas.");
            
            let loaded_image = image_loader.finish(image, None);
            let loaded_layout = layout_loader.finish(layout, None);
            
            let image_handle = load_context.add_loaded_labeled_asset("atlas", loaded_image);
            let layout_handle = load_context.add_loaded_labeled_asset("layout", loaded_layout);
            
            let mut anims = HashMap::default();
            for tag_index in 0..aseprite.num_tags() {
                let tag = aseprite.tag(tag_index);
                let duration = (tag.from_frame()..=tag.to_frame())
                    .map(|index| durations[index as usize])
                    .sum::<u32>();
                let mut frame_map = RangeMap::new();
                let mut last : f32 = 0.0;
                for frame_index in tag.from_frame()..=tag.to_frame() {
                    let current_duration = durations[frame_index as usize] as f32 / duration as f32;
                    frame_map.insert(last..last + current_duration, frame_index as usize);
                    last += current_duration;
                }
                let anim = Anim { frame_map, duration: duration as f32 / 1000.0 };
                anims.insert(tag.name().to_string(), anim);
            }
            
            let mut frame_map = RangeMap::new();
            frame_map.insert(0.0..1.0, "test");
            
            Ok(Aseprite { layout: layout_handle, duration: durations, image: image_handle, anims, dimensions: UVec2::new(aseprite.width() as u32, aseprite.height() as u32) })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}

//=================================================================================
//    Aseprite State Animation
//=================================================================================

pub trait AsepriteSimpleAnimation : Sized + FromWorld {
    fn get_tag_name(&self) -> &str;
    
    fn get_anchor_pixel() -> Vec2;
}

//=================================================================================
//    Aseprite State Animation
//=================================================================================

pub trait AsepriteStateAnimation : Sized + FromWorld {
    type State : Component + FromWorld;
    
    fn get_tag_name(&self) -> &str;
    
    fn get_anchor_pixel() -> Vec2;
    
    fn update_state(&mut self, item : &Self::State);
}

impl <A : AsepriteStateAnimation + Send + Sync + 'static> Animation for A {
    type AsociatedAsset = Aseprite;

    type Query<'w, 's> = (&'w mut TextureAtlas, Option<&'w A::State>);

    fn apply(
        animator : &mut Animator<Self>, 
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>, 
        asset : &Self::AsociatedAsset,
    ) {
        let (atlas, item) = items;
        atlas.layout = asset.layout.clone();
        
        if let Some(item) = item {
            animator.current_state.update_state(item);
        }
        
        let tag = animator.current_state.get_tag_name();
        if let Some(anim) = asset.anims.get(tag) {
            let frame = anim.frame_map.get(animator.progress()).unwrap_or_else(|| {println!("{}", animator.progress()); &0});
            atlas.index = *frame;
        }
    }
    
    fn duration(&self, asset : &Self::AsociatedAsset) -> f32 {
        asset.anims.get(self.get_tag_name()).unwrap().duration
    }

    fn spawn(world: &mut World, path : String, entity : Entity) {
        let animation_comp = Self::from_world(world);
        let state = A::State::from_world(world);
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let animation : Handle<Self::AsociatedAsset> = asset_server.load(&path);
        let image : Handle<Image> = asset_server.load(format!("{}#atlas", path));
        let layout : Handle<TextureAtlasLayout> = asset_server.load(format!("{}#layout", path));
        
        world.get_or_spawn(entity).unwrap()
            .insert(Animator::new(animation_comp))
            .insert(animation)
            .insert(SpriteSheetBundle {
                texture : image,
                atlas : TextureAtlas { layout, index: 0 },
                ..Default::default()
            })
            .insert(state)
        ;
    }
}