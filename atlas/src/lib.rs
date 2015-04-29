extern crate rustc_serialize;
extern crate image;
extern crate fontcache;

use fontcache::RenderedFont;
use rustc_serialize::json;

pub enum DecodingError {
    ImageDecodingError(image::ImageError),
    JsonDecodingError(json::DecoderError)
}

pub type DecodingResult<T> = Result<T, DecodingError>;

impl From<image::ImageError> for DecodingError {
    fn from(img_err: image::ImageError) -> DecodingError {
        DecodingError::ImageDecodingError(img_err)
    }
}

impl From<json::DecoderError> for DecodingError {
    fn from(img_err: json::DecoderError) -> DecodingError {
        DecodingError::JsonDecodingError(img_err)
    }
}

pub fn load_atlas(image: &[u8], metadata: &str)
-> DecodingResult<RenderedFont<image::DynamicImage>> {
    let img = try!(image::load_from_memory(image));
    let meta: RenderedFont<()> = try!(json::decode(metadata));

    Ok(meta.map_img(move |_| img))
}

