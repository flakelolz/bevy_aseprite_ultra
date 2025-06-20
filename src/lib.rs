#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub(crate) mod animation;
pub(crate) mod error;
pub(crate) mod loader;
#[cfg(feature = "asset_processing")]
pub(crate) mod processor;
pub(crate) mod slice;

pub mod prelude {
    pub use crate::animation::{
        render_animation, Animation, AnimationDirection, AnimationEvents, AnimationRepeat,
        AnimationState, AseAnimation, ManualTick, NextFrameEvent, PlayDirection, RenderAnimation,
    };
    pub use crate::loader::{Aseprite, AsepriteLoaderSettings, SliceMeta};
    pub use crate::slice::{render_slice, AseSlice, RenderSlice};
    pub use crate::AsepriteUltraPlugin;
}

/// # Aseprite Ultra Plugin
///
/// Quick guide:
///
/// add the plugin to your game
/// ```rust
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(ImagePlugin {
///             default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest(),
///         }))
///         .add_plugins(AsepriteUltraPlugin)
///         .add_systems(Startup, setup)
///         .run();
/// }
///
/// // spawn sprites, animations and ui
/// fn setup(mut cmd: Commands, server: Res<AssetServer>) {
///     // ui animation
///     cmd.spawn(AseUiAnimation {
///         aseprite: server.load("player.aseprite").into(),
///         animation: Animation::default().with_tag("walk-right"),
///     });
///
///     // sprite animation
///     cmd.spawn(AseSpriteAnimation {
///         aseprite: server.load("player.aseprite").into(),
///         animation: Animation::default().with_tag("walk-right"),
///     });
///
///     // static sprite
///     cmd.spawn(AseSpriteSlice {
///         name: "ghost_red".into(),
///         aseprite: server.load("ghost_slices.aseprite"),
///     });
///
///     // static ui
///     cmd.spawn(AseUiSlice {
///         name: "ghost_red".into(),
///         aseprite: server.load("ghost_slices.aseprite"),
///     });
/// }
///
/// ```
pub struct AsepriteUltraPlugin {
    pub max_atlas_size: UVec2,
}

impl Default for AsepriteUltraPlugin {
    fn default() -> Self {
        Self {
            max_atlas_size: UVec2::splat(4096),
        }
    }
}
impl Plugin for AsepriteUltraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::AsepriteLoaderPlugin {
            max_atlas_size: self.max_atlas_size,
        });
        app.add_plugins(slice::AsepriteSlicePlugin);
        app.add_plugins(animation::AsepriteAnimationPlugin);
        #[cfg(feature = "asset_processing")]
        app.add_plugins(processor::AsepriteProcessorPlugin);
    }
}
