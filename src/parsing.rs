#[derive(Debug)]
pub struct ParseError;

pub trait Parsed {
    fn parse_from(section: Vec<String>) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn is_section_id(id: String) -> bool;
}

pub trait FieldParser<T> {
    fn parse_field(&self) -> Result<T, ParseError>
    where
        T: Sized;
}
