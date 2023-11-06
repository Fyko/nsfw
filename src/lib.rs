#![doc = include_str!("../README.md")]
use std::io::Read;

use image::{
    imageops::{self, FilterType},
    ImageBuffer, Rgba,
};
use model::Classification;
use tract_onnx::prelude::*;

#[cfg(feature = "gif")]
pub mod gif;
pub mod model;

/// A reusable Result type.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The model used for classifying images.
pub type Model = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub(crate) static SIZE: usize = 224;
pub(crate) static SIZE_U32: u32 = 224;

/// The bytes of the model used for classifying images.
#[cfg(test)]
pub(crate) static MODEL_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"));

/// A static instances of model used for classifying images.
///
/// Note: This is a lazy static, so it will only be initialized the first time it is used.
#[cfg(test)]
pub(crate) static MODEL: once_cell::sync::Lazy<Model> =
    once_cell::sync::Lazy::new(|| create_model(MODEL_BYTES).unwrap());

/// Creates the model used for classifying images.
/// ```rust,ignore
/// let model = create_model(File::open("model.onnx")?)?;
/// ```
pub fn create_model<R: Read>(mut model: R) -> Result<Model> {
    let model = tract_onnx::onnx()
        .model_for_read(&mut model)
        .map_err(|_err| "failed to load model")?
        .with_input_fact(0, f32::fact([1, SIZE, SIZE, 3]).into())
        .map_err(|_err| "failed to set input fact")?
        .into_optimized()
        .map_err(|_err| "failed to optimize model")?
        .into_runnable()
        .map_err(|_err| "failed to create runnable model")?;

    Ok(model)
}

/// Runs an image through the model, returning the predictions.
/// ```rust,ignore
/// let model = create_model(File::open("model.onnx")?)?;
/// let image = image::open("mydudes.jpg")?.to_rgba8();
/// let predictions = examine(&model, &image);
/// assert!(predictions.is_ok());
/// ```
pub fn examine(
    model: &Model,
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<Vec<Classification>> {
    let resized = imageops::resize(image, SIZE_U32, SIZE_U32, FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, SIZE, SIZE, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    let result = model.run(tvec!(image.into()))?;
    let data = result[0].to_array_view::<f32>()?;

    Ok(data
        .into_iter()
        .enumerate()
        .map(|(metric, score)| Classification {
            metric: metric
                .try_into()
                .expect("received invalid metric from model, this should never happen"),
            score: *score,
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::{examine, Result};
    use crate::MODEL;

    fn test_static(path: &'static str) -> Result<()> {
        let image = image::open(path)?.to_rgba8();
        let predictions = examine(&MODEL, &image);
        assert!(predictions.is_ok());

        let predictions = predictions.unwrap();
        assert_eq!(predictions.len(), 5);

        Ok(())
    }

    #[test]
    fn test_jpg() -> Result<()> {
        test_static(concat!(env!("CARGO_MANIFEST_DIR"), "/test/mydudes.jpg"))
    }

    #[test]
    fn test_png() -> Result<()> {
        test_static(concat!(env!("CARGO_MANIFEST_DIR"), "/test/mydudes.png"))
    }

    #[test]
    fn test_webp() -> Result<()> {
        test_static(concat!(env!("CARGO_MANIFEST_DIR"), "/test/mydudes.webp"))
    }

    #[test]
    fn test_create_model() -> Result<()> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx");
        let model = super::create_model(File::open(path)?);
        assert!(model.is_ok());

        Ok(())
    }
}
