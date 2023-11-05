use std::io::Read;

use image::{codecs::gif::GifDecoder, AnimationDecoder};

use crate::{examine, model::Classification, Model, Result};

/// An iterator that examines every frame of a GIF.
///
/// ### Example
/// ```rust,ignore
/// let gif_path = concat!(env!("CARGO_MANIFEST_DIR"), "/media/test_porn.gif");
/// let file = BufReader::new(File::open(gif_path)?);
/// let gif = GifParser::new(GifDecoder::new(file)?, &model);
///
/// for (index, frame) in gif.enumerate() {
///     println!("frame {index}: {frame:#?}");
/// }
/// ```
///
/// ### Example: Not iterating
/// ```rust, ignore
/// let gif = GifParser::new(GifDecoder::new(file)?, &model);
/// let frames = gif.collect::<Vec<_>>();
/// ```
pub struct GifParser<'a> {
    frames: image::Frames<'a>,
    model: &'a Model,
}

impl<'a> GifParser<'a> {
    pub fn new<R: Read + 'a>(gif: GifDecoder<R>, model: &'a Model) -> Self {
        let frames = gif.into_frames();

        Self { frames, model }
    }
}

impl<'a> Iterator for GifParser<'a> {
    type Item = Result<Vec<Classification>>;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.frames.next();

        match frame {
            Some(Ok(frame)) => Some(examine(self.model, &frame.into_buffer())),
            Some(Err(e)) => Some(Err(e.into())),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MODEL;
    use image::codecs::gif::GifDecoder;
    use std::{fs::File, io::BufReader};

    use super::{GifParser, Result};

    #[tokio::test]
    async fn test_gif() -> Result<()> {
        let gif_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test/puffBounce.gif");
        let file = BufReader::new(File::open(gif_path)?);
        let frames = GifParser::new(GifDecoder::new(file)?, &MODEL);

        for (_index, frame) in frames.enumerate() {
            assert!(frame.is_ok());
        }

        Ok(())
    }
}
