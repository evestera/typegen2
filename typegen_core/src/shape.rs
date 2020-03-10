use std::fmt::{Formatter, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Bottom,
    Top, // TODO: Replace with AnyOf
    Optional(Box<Shape>),
    Bool,
    Str, // IDEA: Add pattern-based types. URL, ISO8601 types, UUID. Also observed values.
    Integer,
    Float,
    List(Box<Shape>),
    // TODO: Tuple
    // TODO: Map
    Record(Vec<(String, Shape)>),
    // TODO: Opaque
}

#[allow(dead_code)]
pub fn fold_shapes(shapes: Vec<Shape>) -> Shape {
    shapes.into_iter().fold(Shape::Bottom, common_shape)
}

pub fn common_shape(a: Shape, b: Shape) -> Shape {
    if a == b {
        return a;
    }
    use self::Shape::*;
    match (a, b) {
        (a, Bottom) | (Bottom, a) => a,
        (Integer, Float) | (Float, Integer) => Float,
        (a, Optional(b)) | (Optional(b), a) => make_optional(common_shape(a, *b)),
        (List(e1), List(e2)) => List(Box::new(common_shape(*e1, *e2))),
        (Record(f1), Record(f2)) => Record(common_field_shapes(f1, f2)),
        _ => Top,
    }
}

fn make_optional(a: Shape) -> Shape {
    use self::Shape::*;
    match a {
        Optional(_) => a,
        non_nullable => Optional(Box::new(non_nullable)),
    }
}

fn common_field_shapes(
    f1: Vec<(String, Shape)>,
    mut f2: Vec<(String, Shape)>,
) -> Vec<(String, Shape)> {
    if f1 == f2 {
        return f1;
    }
    let mut unified: Vec<(String, Shape)> = Vec::new();
    for (key, val) in f1.into_iter() {
        let mut index = None;
        for i in 0..(f2.len()) {
            if key == f2[i].0 {
                index = Some(i)
            }
        }
        match index {
            Some(i) => unified.push((key, common_shape(val, f2.remove(i).1))),
            None => unified.push((key, make_optional(val))),
        }
    }
    for (key, val) in f2.into_iter() {
        unified.push((key, make_optional(val)));
    }
    unified
}

//impl std::fmt::Debug for Shape {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        match self {
//            Shape::Bottom => f.write_str("bottom"),
//            Shape::Top => f.write_str("top"),
//            Shape::Optional(inner) => {
//                inner.fmt(f)?;
//                f.write_str("?")
//            },
//            Shape::Bool => f.write_str("bool"),
//            Shape::Str => f.write_str("str"),
//            Shape::Integer => f.write_str("int"),
//            Shape::Float => f.write_str("float"),
//            Shape::List(inner) => {
//                inner.fmt(f)?;
//                f.write_str("[]")
//            },
//            Shape::Record(fields) => {
//                f.debug_map()
//                    .entries(fields.iter().map(|(k, v)| (k, v)))
//                    .finish()
//            },
//        }
//    }
//}

const INDENT_SIZE: u8 = 4;

pub fn shape_to_string(shape: &Shape) -> String {
    let mut buffer = String::new();
    shape_to_string_inner(shape, &mut buffer, 0);
    buffer
//    format!("{:#?}", shape)
}

fn shape_to_string_inner(shape: &Shape, buffer: &mut String, indent: u8) {
    match shape {
        Shape::Bottom => buffer.push_str("bottom"),
        Shape::Top => buffer.push_str("top"),
        Shape::Optional(inner_shape) => {
            shape_to_string_inner(inner_shape, buffer, indent);
            buffer.push('?');
        },
        Shape::Bool => buffer.push_str("bool"),
        Shape::Str => buffer.push_str("str"),
        Shape::Integer => buffer.push_str("int"),
        Shape::Float => buffer.push_str("float"),
        Shape::List(inner_shape) => {
            shape_to_string_inner(inner_shape, buffer, indent);
            buffer.push_str("[]");
        },
        Shape::Record(inner_shapes) => {
            buffer.push_str("{\n");
            for (key, inner_shape) in inner_shapes {
                do_indent(buffer, indent + INDENT_SIZE);
                buffer.push_str(key);
                buffer.push_str(": ");
                shape_to_string_inner(inner_shape, buffer, indent + INDENT_SIZE);
                buffer.push_str(",\n");
            }
            do_indent(buffer, indent);
            buffer.push_str("}");
        },
    };
}

fn do_indent(buffer: &mut String, indent: u8) {
    for _i in 0..indent {
        buffer.push(' ');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unify() {
        use self::Shape::*;
        assert_eq!(common_shape(Bool, Bool), Bool);
        assert_eq!(common_shape(Bool, Integer), Top);
        assert_eq!(common_shape(Integer, Float), Float);
        assert_eq!(
            common_shape(Optional(Box::new(Bottom)), Top),
            Optional(Box::new(Top))
        );
        assert_eq!(
            common_shape(Optional(Box::new(Bottom)), Bool),
            Optional(Box::new(Bool))
        );
        assert_eq!(
            common_shape(Optional(Box::new(Bottom)), Optional(Box::new(Integer))),
            Optional(Box::new(Integer))
        );
        assert_eq!(
            common_shape(List(Box::new(Float)), List(Box::new(Integer))),
            List(Box::new(Float))
        );
        assert_eq!(
            common_shape(Top, Optional(Box::new(Integer))),
            Optional(Box::new(Top))
        );
        assert_eq!(
            common_shape(Optional(Box::new(Integer)), Optional(Box::new(Float))),
            Optional(Box::new(Float))
        );
        assert_eq!(
            common_shape(Optional(Box::new(Str)), Optional(Box::new(Integer))),
            Optional(Box::new(Top))
        );
    }

    #[test]
    fn unify_records() {
        use self::Shape::*;
        {
            let f1 = vec![
                ("a".to_string(), Integer),
                ("b".to_string(), Bool),
                ("c".to_string(), Integer),
                ("d".to_string(), Str),
            ];
            let f2 = vec![
                ("a".to_string(), Integer),
                ("c".to_string(), Float),
                ("d".to_string(), Optional(Box::new(Bottom))),
                ("e".to_string(), Top),
            ];
            assert_eq!(
                common_field_shapes(f1, f2),
                vec![
                    ("a".to_string(), Integer),
                    ("b".to_string(), Optional(Box::new(Bool))),
                    ("c".to_string(), Float),
                    ("d".to_string(), Optional(Box::new(Str))),
                    ("e".to_string(), Optional(Box::new(Top))),
                ]
            );
        }
    }
}
