use std::io::Read;

use crate::model::Model;
pub use error::Error;
pub use lerp::*;
pub use renderer::*;
use timeline::Timeline;

mod error;
mod layer;
mod lerp;
mod model;
mod renderer;
mod timeline;

pub mod prelude {
    pub use crate::layer::frame::*;
    pub use crate::layer::hierarchy::*;
    pub use crate::layer::shape::{
        AnyFill, AnyStroke, PathFactory, StyledShape, StyledShapeIterator, TrimInfo,
    };
    pub use crate::layer::staged::{RenderableContent, StagedLayer};
    pub use crate::model::*;
    pub use crate::timeline::{Id, TimelineAction};
}

pub struct Lottie {
    pub model: Model,
    pub scale: f32,
    timeline: Timeline,
}

impl Lottie {
    /// Initiate a new `Lottie` by providing a raw `Model`, a `FontKit` for font
    /// management, and a root path.Root path will be used to resolve relative
    /// paths of media files in this lottie model
    pub fn new(model: Model, root_path: &str) -> Result<Self, Error> {
        let timeline = Timeline::new(&model, root_path)?;
        Ok(Lottie {
            model,
            timeline,
            scale: 1.0,
        })
    }

    #[cfg(not(all(target_os = "unknown", target_arch = "wasm32")))]
    pub fn from_reader<R: Read>(r: R, root_path: &str) -> Result<Self, Error> {
        let path = dirs::font_dir().unwrap();
        let model = Model::from_reader(r)?;
        Ok(Lottie::new(model, root_path)?)
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }
}
