use std::io::Read;
use crate::shape::{Shape, shape_to_string};
use crate::jsoninfer::FromJson;

mod jsoninfer;
mod jsoninputerr;
mod jsonlex;
mod shape;

pub fn typegen(input: impl Read) -> String {
    let shape = Shape::from_json(input).unwrap();
    shape_to_string(&shape)
}
