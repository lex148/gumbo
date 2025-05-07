use cruet::Inflector;
use std::path::PathBuf;
use welds::model_traits::TableIdent;

#[derive(Debug)]
pub(crate) struct Names {
    pub(crate) controller_mod: String,
    pub(crate) controller_path: PathBuf,
    pub(crate) table_name: String,
    pub(crate) schema_name: Option<String>,
    pub(crate) model_struct: String,
    pub(crate) model_mod: String,
    pub(crate) model_path: PathBuf,
    pub(crate) view_mod: String,
    pub(crate) title: String,
}

impl Names {
    pub(crate) fn new(name: &str) -> Names {
        let mut parts: Vec<_> = name.trim().split(".").collect();
        let tablename = parts.pop().unwrap();
        let schema_name = parts.pop().map(|x| x.to_string());

        let p = pluralizer::pluralize(tablename.trim(), 10, false).to_snake_case();
        let s = pluralizer::pluralize(tablename.trim(), 1, false).to_snake_case();
        let class = s.to_class_case();

        let model_path = PathBuf::from(format!("./src/models/{s}/mod.rs"));
        let controller_path = PathBuf::from(format!("./src/controllers/{p}_controller/mod.rs"));

        Names {
            controller_mod: format!("{p}_controller"),
            controller_path,
            table_name: p.to_owned(),
            schema_name,
            model_struct: class,
            model_mod: s.to_owned(),
            model_path,
            view_mod: p.to_owned(),
            title: p.to_title_case(),
        }
    }

    /// returns a welds attribute to use to link a model to a table
    pub(crate) fn welds_table_attribute(&self) -> String {
        let table_name = &self.table_name;
        match &self.schema_name {
            Some(schemaname) => {
                format!("#[welds(schema = \"{schemaname}\", table = \"{table_name}\")]")
            }
            None => format!("#[welds(table = \"{table_name}\")]"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn controller_name() {
        let n = Names::new("carPrice");
        assert_eq!(n.controller_mod, "car_prices_controller");
    }

    #[test]
    fn controller_path() {
        let n = Names::new("carPrice");
        assert_eq!(
            n.controller_path.to_str().unwrap(),
            "./src/controllers/car_prices_controller/mod.rs"
        );
    }

    #[test]
    fn model_name() {
        let n = Names::new("carPrice");
        assert_eq!(n.model_struct, "CarPrice");
    }

    #[test]
    fn model_mod() {
        let n = Names::new("carPrice");
        assert_eq!(n.model_mod, "car_price");
    }

    #[test]
    fn model_path() {
        let n = Names::new("carPrice");
        assert_eq!(
            n.model_path.to_str().unwrap(),
            "./src/models/car_price/mod.rs"
        );
    }

    #[test]
    fn model_path_with_schema() {
        let n = Names::new("car.Price");
        assert_eq!(n.model_path.to_str().unwrap(), "./src/models/price/mod.rs");
    }

    #[test]
    fn view_mod() {
        let n = Names::new("carPrice");
        assert_eq!(n.view_mod, "car_prices");
    }

    #[test]
    fn table_name() {
        let n = Names::new("carPrice");
        assert_eq!(n.view_mod, "car_prices");
    }

    #[test]
    fn table_name_with_schema() {
        let n = Names::new("car.Price");
        assert_eq!(n.table_name, "prices");
        assert_eq!(n.schema_name.as_deref(), Some("car"));
    }
}
