//=================================================================================
// Here we are defining the Aseprite Animation AssetFile, and the Aseprite 
// Animation ID.
//=================================================================================

use std::io;

use asefile::AsepriteFile;
use bevy::{asset::{AssetIndex, AssetLoader, AsyncReadExt}, ecs::query::WorldQuery, prelude::{Vec2, *}, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}, utils::HashMap};

use crate::id::Animation;

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
    anims : HashMap<String, Frame>,
    dimensions : UVec2
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Frame {
    pub duration : u32,
    pub start : u32,
    pub end : u32
}

// #[derive(Clone)]
// pub struct AnimClip {
//     frames : Vec<AnimFrame>,
//     pub duration : u32
// }

// #[derive(Clone)]
// pub struct AnimFrame {
//     frame_index : usize,
//     duration : u32
// }

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
            for frame_index in 0..aseprite.num_frames() {
                let frame = aseprite.frame(frame_index);
                let image = Image::new(
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
                frames.push(image);
            }
            for image in frames.iter() { atlas.add_texture(None, image); }
            let (layout, image) = atlas.finish().expect("Failed to build texture atlas.");
            
            let loaded_image = image_loader.finish(image, None);
            let loaded_layout = layout_loader.finish(layout, None);
            
            let image_handle = load_context.add_loaded_labeled_asset("atlas", loaded_image);
            let layout_handle = load_context.add_loaded_labeled_asset("layout", loaded_layout);
            
            Ok(Aseprite { layout: layout_handle, image: image_handle, anims: HashMap::default(), dimensions: UVec2::new(aseprite.width() as u32, aseprite.height() as u32) })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}

//=================================================================================
//    Aseprite Animation
//=================================================================================

pub trait AsepriteAnimation : Sized {    
    fn from_tag(tag_name : &str) -> Option<Self>;
    
    fn get_anchor_pixel() -> Vec2;
}

impl <A : AsepriteAnimation> Animation for A {
    type AsociatedAsset = Aseprite;

    type Query<'w, 's> = ();

    fn apply(
        &self, 
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>, 
        asset : &Self::AsociatedAsset,
        time : &Time,
    ) {
        todo!()
    }
}