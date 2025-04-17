use crate::errors::GumboError;
use cruet::Inflector;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Action {
    pub name: String,
    pub method: String,
}

impl Action {
    pub(crate) fn fn_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl FromStr for Action {
    type Err = GumboError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(':').collect();
        if parts.len() > 2 || parts.is_empty() {
            eprintln!(
                "unknown format for action name: expected action the format of 'action:method' or 'action' "
            );
        }
        let method = parts
            .get(1)
            .map(|x| x.trim().to_lowercase())
            .unwrap_or("get".to_owned());
        validate_methods(&method).map_err(|s| GumboError::InvalidControllerAction(s.to_owned()))?;
        let name = parts.first().unwrap().to_string();

        Ok(Action { name, method })
    }
}

fn validate_methods(method: &str) -> Result<(), &str> {
    match method {
        "get" => return Ok(()),
        "post" => return Ok(()),
        "put" => return Ok(()),
        "patch" => return Ok(()),
        "delete" => return Ok(()),
        _ => {}
    }
    eprintln!("unknown method for action: expected (get post put patch delete)' ");
    Err(method)
}

#[cfg(test)]
mod test_parseing {
    use super::*;

    #[test]
    fn parse_basic_get() {
        let expected = Action {
            name: "test".to_owned(),
            method: "get".to_owned(),
        };
        let act: Action = "test".parse().unwrap();
        assert_eq!(expected, act);
    }

    #[test]
    fn parse_name_and_post() {
        let expected = Action {
            name: "bla".to_owned(),
            method: "post".to_owned(),
        };
        let act: Action = "bla:post".parse().unwrap();
        assert_eq!(expected, act);
    }
}
