use crate::jsoninputerr::JsonInputErr;
use crate::jsonlex::{JsonLexer, JsonToken};
use crate::shape::{Shape, common_shape};
use std::io::Read;
use std::iter::Peekable;

pub trait FromJson {
    fn from_json(json: impl Read) -> Result<Self, JsonInputErr> where Self: Sized;
}

impl FromJson for Shape {
    fn from_json(json: impl Read) -> Result<Self, JsonInputErr> {
        Inference::new(json).infer_value()
    }
}

struct Inference<R: Read> {
    tokens: Peekable<JsonLexer<R>>,
}

impl<R: Read> Inference<R> {
    fn new(source: R) -> Self {
        Inference {
            tokens: JsonLexer::new(source).peekable(),
        }
    }

    fn infer_value(&mut self) -> Result<Shape, JsonInputErr> {
        let token = match self.tokens.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => return Err(err),
            None => return Err(JsonInputErr::UnexpectedEndOfInput),
        };

        match token {
            JsonToken::True | JsonToken::False => Ok(Shape::Bool),
            JsonToken::Null => Ok(Shape::Optional(Box::new(Shape::Bottom))),
            JsonToken::Number(s) => {
                if s.contains(".") {
                    Ok(Shape::Float)
                } else {
                    Ok(Shape::Integer)
                }
            }
            JsonToken::String(_) => Ok(Shape::Str),
            JsonToken::ObjectStart => self.infer_object(),
            JsonToken::ArrayStart => self.infer_array(),
            JsonToken::ObjectEnd | JsonToken::ArrayEnd | JsonToken::Comma | JsonToken::Colon => {
                Err(JsonInputErr::InvalidJson)
            }
        }
    }

    fn infer_object(&mut self) -> Result<Shape, JsonInputErr> {
        if let Some(&Ok(JsonToken::ObjectEnd)) = self.tokens.peek() {
            self.tokens.next();
            return Ok(Shape::Record(Vec::new()));
        }

        let mut fields = Vec::new();
        loop {
            let token = match self.tokens.next() {
                Some(Ok(token)) => token,
                Some(Err(err)) => return Err(err),
                None => return Err(JsonInputErr::UnexpectedEndOfInput),
            };

            let key = match token {
                JsonToken::String(s) => s,
                _ => return Err(JsonInputErr::InvalidJson)
            };

            self.expect_token(JsonToken::Colon)?;

            let value = self.infer_value()?;
            fields.push((key, value));

            if let Some(&Ok(JsonToken::ObjectEnd)) = self.tokens.peek() {
                self.tokens.next();
                return Ok(Shape::Record(fields));
            }

            self.expect_token(JsonToken::Comma)?;
        }
    }

    fn expect_token(&mut self, expected_token: JsonToken) -> Result<(), JsonInputErr> {
        let token = match self.tokens.next() {
            None => return Err(JsonInputErr::UnexpectedEndOfInput),
            Some(Ok(token)) => token,
            Some(Err(err)) => return Err(err),
        };

        if token == expected_token {
            Ok(())
        } else {
            Err(JsonInputErr::InvalidJson)
        }
    }

    fn infer_array(&mut self) -> Result<Shape, JsonInputErr> {
        if let Some(&Ok(JsonToken::ArrayEnd)) = self.tokens.peek() {
            self.tokens.next();
            return Ok(Shape::List(Box::new(Shape::Bottom)));
        }

        let mut inner = Shape::Bottom;

        loop {
            let value = self.infer_value()?;
            inner = common_shape(inner, value);

            if let Some(&Ok(JsonToken::ArrayEnd)) = self.tokens.peek() {
                self.tokens.next();
                return Ok(Shape::List(Box::new(inner)));
            }

            self.expect_token(JsonToken::Comma)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_object() {
        assert_eq!(
            Shape::from_json(r#"{}"#.as_bytes()),
            Ok(Shape::Record(vec![]))
        );
        assert_eq!(
            Shape::from_json(r#"{
                "foo": true
            }"#.as_bytes()),
            Ok(Shape::Record(vec![
                ("foo".to_string(), Shape::Bool)
            ]))
        );
        assert_eq!(
            Shape::from_json(r#"{
                "foo": true,
                "bar": false
            }"#.as_bytes()),
            Ok(Shape::Record(vec![
                ("foo".to_string(), Shape::Bool),
                ("bar".to_string(), Shape::Bool)
            ]))
        );

        assert_eq!(
            Shape::from_json(r#"{
                "foo": true
                "bar": false
            }"#.as_bytes()),
            Err(JsonInputErr::InvalidJson)
        );
        assert_eq!(
            Shape::from_json(r#"{
                "foo": true,
            }"#.as_bytes()),
            Err(JsonInputErr::InvalidJson)
        );
        assert_eq!(
            Shape::from_json(r#"{
                "foo": true,
            "#.as_bytes()),
            Err(JsonInputErr::UnexpectedEndOfInput)
        );
    }

    #[test]
    fn infer_array() {
        assert_eq!(
            Shape::from_json(r#"[]"#.as_bytes()),
            Ok(Shape::List(Box::new(Shape::Bottom)))
        );
        assert_eq!(
            Shape::from_json(r#"[true]"#.as_bytes()),
            Ok(Shape::List(Box::new(Shape::Bool)))
        );
        assert_eq!(
            Shape::from_json(r#"[true, false]"#.as_bytes()),
            Ok(Shape::List(Box::new(Shape::Bool)))
        );
        assert_eq!(
            Shape::from_json(r#"[true, "hello"]"#.as_bytes()),
            Ok(Shape::List(Box::new(Shape::Top)))
        );

        assert_eq!(
            Shape::from_json(r#"[true false]"#.as_bytes()),
            Err(JsonInputErr::InvalidJson)
        );
        assert_eq!(
            Shape::from_json(r#"[true,]"#.as_bytes()),
            Err(JsonInputErr::InvalidJson)
        );
        assert_eq!(
            Shape::from_json(r#"[true"#.as_bytes()),
            Err(JsonInputErr::UnexpectedEndOfInput)
        );
    }
}
