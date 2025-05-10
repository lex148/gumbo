use crate::errors::GumboError;
use cruet::Inflector;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Field {
    pub(crate) visibility: Visibility,
    pub(crate) primary_key: bool,
    pub(crate) welds_ignored: bool,
    pub(crate) name: String,
    pub(crate) col_name: String,
    pub(crate) ty: Type,
    pub(crate) null: bool,
}

impl Field {
    //pub(crate) fn rust_field_name(&self) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Visibility {
    Private,
    Pub,
    #[default]
    PubCrate,
    PubSuper,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// These are types that are defined in migrations.
/// They will get translated into DB types
pub(crate) enum Type {
    Bool,
    IntSmall,
    Int,
    IntBig,
    String,
    //StringSized(u32),
    Text,
    Float,
    FloatBig,
    Binary,
    Uuid,
    Date,
    Time,
    Datetime,
    DatetimeZone,
    Raw(String /*SQL*/, String /*Rust*/),
}

impl Type {
    pub(crate) fn rust_type(&self) -> &str {
        match self {
            Type::Bool => "bool",
            Type::IntSmall => "i16",
            Type::Int => "i32",
            Type::IntBig => "i64",
            Type::String => "String",
            Type::Text => "String",
            Type::Float => "f32",
            Type::FloatBig => "f64",
            Type::Binary => "Vec<u8>",
            Type::Uuid => "uuid::Uuid",
            Type::Time => "chrono::NaiveTime",
            Type::Date => "chrono::NaiveDate",
            Type::Datetime => "chrono::NaiveDateTime",
            Type::DatetimeZone => "chrono::DateTime<chrono::Utc>",
            Type::Raw(_sql_type, rust_type) => rust_type,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Type::Bool => "Bool",
            Type::IntSmall => "IntSmall",
            Type::Int => "Int",
            Type::IntBig => "IntBig",
            Type::String => "String",
            Type::Text => "Text",
            Type::Float => "Float",
            Type::FloatBig => "FloatBig",
            Type::Binary => "Binary",
            Type::Uuid => "Uuid",
            Type::Date => "Date",
            Type::Time => "Time",
            Type::Datetime => "Datetime",
            Type::DatetimeZone => "DatetimeZone",
            Type::Raw(sql_type, _rust) => sql_type,
        };
        f.write_str(t)?;
        Ok(())
    }
}

impl FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ty = pluralizer::pluralize(s.trim(), 1, false).to_snake_case();
        let t = match ty.as_str() {
            "bool" => Type::Bool,
            "boolean" => Type::Bool,
            "int_small" => Type::IntSmall,
            "small_int" => Type::IntSmall,
            "integer_small" => Type::IntSmall,
            "small_integer" => Type::IntSmall,
            "integer" => Type::Int,
            "int" => Type::Int,
            "intbig" => Type::IntBig,
            "bigint" => Type::IntBig,
            "int_big" => Type::IntBig,
            "big_int" => Type::IntBig,
            "integer_big" => Type::IntBig,
            "big_integer" => Type::IntBig,
            "string" => Type::String,
            "text" => Type::Text,
            "float" => Type::Float,
            "big_float" => Type::FloatBig,
            "float_big" => Type::FloatBig,
            "binary" => Type::Binary,
            "uuid" => Type::Uuid,
            "date" => Type::Date,
            "time" => Type::Time,
            "datetime" => Type::Datetime,
            "datetimezone" => Type::DatetimeZone,
            "timestamptz" => Type::DatetimeZone,
            "datetimetz" => Type::DatetimeZone,
            _ => return Err(()),
        };
        Ok(t)
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Visibility::Pub => "pub",
            Visibility::PubCrate => "pub(crate)",
            Visibility::PubSuper => "pub(super)",
            Visibility::Private => "",
        };
        f.write_str(v)?;
        Ok(())
    }
}

impl FromStr for Visibility {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vis = s.trim().to_lowercase();
        let v = match vis.as_str() {
            "pub" => Visibility::Pub,
            "pub(crate)" => Visibility::PubCrate,
            "pub(super)" => Visibility::PubSuper,
            _ => return Err(()),
        };
        Ok(v)
    }
}

impl FromStr for Field {
    type Err = GumboError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<_> = s.split(':').collect();

        // no type, default to string
        if parts.len() == 1 {
            let name = parts[0].to_snake_case();
            return Ok(Field {
                visibility: Visibility::default(),
                primary_key: parts[0] == "id",
                welds_ignored: false,
                col_name: name.clone(),
                name,
                ty: Type::String,
                null: false,
            });
        }

        // parse the type
        if parts.len() == 2 {
            let name = parts[0].to_snake_case();
            return Ok(Field {
                visibility: Visibility::default(),
                primary_key: parts[0] == "id",
                welds_ignored: false,
                col_name: name.clone(),
                name,
                ty: Type::from_str(parts[1])
                    .map_err(|_| GumboError::InvalidFieldType(s.to_string()))?,
                null: false,
            });
        }

        // parse the type, and option/null
        if parts.len() == 3 {
            let tail = parts[2].trim().to_lowercase();
            let null = tail == "null" || tail == "optional" || tail == "option";
            let name = parts[0].to_snake_case();
            return Ok(Field {
                visibility: Visibility::default(),
                primary_key: parts[0] == "id",
                welds_ignored: false,
                col_name: name.clone(),
                name,
                ty: Type::from_str(parts[1])
                    .map_err(|_| GumboError::InvalidFieldType(s.to_string()))?,
                null,
            });
        }

        Err(GumboError::InvalidFieldType(s.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {
        let f = Field::from_str("name").unwrap();
        assert_eq!(f.name, "name");
        assert_eq!(f.ty, Type::String);
        assert!(!f.primary_key);
    }

    #[test]
    fn basic_id() {
        let f = Field::from_str("id:int").unwrap();
        assert_eq!(f.name, "id");
        assert!(f.primary_key);
    }

    #[test]
    fn should_be_underscore_case() {
        let f = Field::from_str("Name").unwrap();
        assert_eq!(f.name, "name");
        assert_eq!(f.ty, Type::String);
    }

    #[test]
    fn should_read_type() {
        let f = Field::from_str("Name:int").unwrap();
        assert_eq!(f.name, "name");
        assert_eq!(f.ty, Type::Int);
    }

    #[test]
    fn already_snake() {
        let f = Field::from_str("car_price:int").unwrap();
        assert_eq!(f.name, "car_price");
        assert_eq!(f.ty, Type::Int);
    }

    #[test]
    fn class_case() {
        let f = Field::from_str("CarPrice:int").unwrap();
        assert_eq!(f.name, "car_price");
        assert_eq!(f.ty, Type::Int);
    }
}
