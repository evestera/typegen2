use crate::jsoninfer::FromJson;
use crate::shape::{shape_to_string, Shape};
use std::io::Read;

mod jsoninfer;
mod jsoninputerr;
mod jsonlex;
mod shape;

pub fn typegen(input: impl Read) -> String {
    let shape = Shape::from_json(input).unwrap();
    shape_to_string(&shape)
}

//
// fn get_sample<'a>(source: &'a SampleSource) -> Result<Box<dyn Read + 'a>, JTError> {
//     let sample: Box<dyn Read> = match *source {
//         #[cfg(feature = "remote-samples")]
//         SampleSource::Url(url) => Box::new(reqwest::get(url)?),
//         #[cfg(not(feature = "remote-samples"))]
//         SampleSource::Url(_) => {
//             return Err("Remote samples disabled".into());
//         }
//
//         #[cfg(feature = "local-samples")]
//         SampleSource::File(path) => Box::new(File::open(path)?),
//         #[cfg(not(feature = "local-samples"))]
//         SampleSource::File(_) => {
//             return Err("Local samples disabled".into());
//         }
//
//         SampleSource::Text(text) => Box::new(text.as_bytes()),
//     };
//     Ok(sample)
// }
