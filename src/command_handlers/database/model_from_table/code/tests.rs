use crate::fields::Field;
use crate::fields::Type;
use crate::fields::Visibility;

use super::*;

const BASIC: &str = "
struct Test {
    #[welds(primary_key)]
    pub id: i32,
    pub other: Option<i64>,
    pub other2: Option<uuid::Uuid>,
    pub other3: String,
    pub other4: uuid::Uuid
}
";

#[test]
fn parse_basic() {
    let fields = read_fields(BASIC);
    assert_eq!(5, fields.len());
    assert_eq!(
        fields[0],
        Field {
            visibility: Visibility::Pub,
            primary_key: true,
            welds_ignored: false,
            name: "id".to_string(),
            ty: Type::Raw("".to_string(), "i32".to_string()),
            null: false
        }
    );
    assert_eq!(
        fields[1],
        Field {
            visibility: Visibility::Pub,
            primary_key: false,
            welds_ignored: false,
            name: "other".to_string(),
            ty: Type::Raw("".to_string(), "i64".to_string()),
            null: true
        }
    );
    assert_eq!(
        fields[2],
        Field {
            visibility: Visibility::Pub,
            primary_key: false,
            welds_ignored: false,
            name: "other2".to_string(),
            ty: Type::Raw("".to_string(), "uuid::Uuid".to_string()),
            null: true
        }
    );
    assert_eq!(
        fields[3],
        Field {
            visibility: Visibility::Pub,
            primary_key: false,
            welds_ignored: false,
            name: "other3".to_string(),
            ty: Type::Raw("".to_string(), "String".to_string()),
            null: false
        }
    );
    assert_eq!(
        fields[4],
        Field {
            visibility: Visibility::Pub,
            primary_key: false,
            welds_ignored: false,
            name: "other4".to_string(),
            ty: Type::Raw("".to_string(), "uuid::Uuid".to_string()),
            null: false
        }
    );
}

const KEEP: &str = "
struct Test {
    #[welds(ignore)]
    pub keep: std::collections::HashMap<String, String>,
}
";

#[test]
fn parse_keep() {
    let fields = read_fields(KEEP);
    assert_eq!(
        fields[0],
        Field {
            visibility: Visibility::Pub,
            primary_key: false,
            welds_ignored: true,
            name: "keep".to_string(),
            ty: Type::Raw(
                "".to_string(),
                "std::collections::HashMap<String, String>".to_string()
            ),
            null: false
        }
    );
}
