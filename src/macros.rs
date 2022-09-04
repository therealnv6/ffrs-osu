pub fn read_value<T>(lines: Vec<String>, mut closure: T) -> Option<String>
where
    T: FnMut(&String) -> bool,
{
    return match lines
        .iter()
        .filter(|x| closure(x.clone()))
        .cloned()
        .collect::<Vec<String>>()
        .first()
    {
        Some(value) => Some(value.to_owned()),
        None => None,
    };
}

macro_rules! field_parser {
    ( $type:ty ) => {
        impl FieldParser<$type> for String {
            fn parse_field(&self) -> Result<$type, ParseError>
            where
                Self: Sized,
            {
                return self.parse().map_err(|_| ParseError);
            }
        }

        impl FieldParser<$type> for &str {
            fn parse_field(&self) -> Result<$type, ParseError>
            where
                Self: Sized,
            {
                return self.parse().map_err(|_| ParseError);
            }
        }
    };
}

macro_rules! parsed {
    ( $src_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    })
    => {
        #[derive(PartialEq, Debug)]
        pub struct $src_name {
            $(
                pub $attr_name : Option<$attr_type>
            ),*
        }

        impl Parsed for $src_name {
            fn parse_from(section: Vec<String>) -> Result<$src_name, ParseError> {
                Ok(
                $src_name {
                    $( $attr_name : {
                        let default = stringify!($attr_default).to_owned();
                        let name = stringify!($attr_name).to_owned();

                        let id: String = if (default == "Empty") {
                            name.to_case(Case::Pascal)
                        } else {
                            default
                        };

                        let value = read_value(section.clone(), |x| x.starts_with(&id));

                        if let Some(mut value) = value {
                            value.replace_range(0..(&id.len() + 2), "");
                            Some(value.parse().unwrap())
                        } else {
                            None
                        }
                    } ),*
                })
            }

            fn is_section_id(id: String) -> bool {
                return id == "";
            }
        }
    };
}

pub(crate) use field_parser;
pub(crate) use parsed;
