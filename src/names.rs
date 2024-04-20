use cruet::Inflector;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Names {
    pub(crate) controller_mod: String,
    pub(crate) controller_path: PathBuf,
    pub(crate) table_name: String,
    pub(crate) model_struct: String,
    pub(crate) model_mod: String,
    pub(crate) model_path: PathBuf,
    pub(crate) view_mod: String,
    pub(crate) title: String,
}

impl Names {
    pub(crate) fn new(name: &str) -> Names {
        let p = pluralizer::pluralize(name.trim(), 10, false).to_snake_case();
        let s = pluralizer::pluralize(name.trim(), 1, false).to_snake_case();
        let class = s.to_class_case();
        let model_path = PathBuf::from(format!("./src/models/{s}/mod.rs"));
        let controller_path = PathBuf::from(format!("./src/controllers/{p}_controller.rs"));

        Names {
            controller_mod: format!("{p}_controller"),
            controller_path,
            table_name: p.to_owned(),
            model_struct: class,
            model_mod: s.to_owned(),
            model_path,
            view_mod: p.to_owned(),
            title: p.to_title_case(),
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
            "./src/controllers/car_prices_controller.rs"
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
    fn view_mod() {
        let n = Names::new("carPrice");
        assert_eq!(n.view_mod, "car_prices");
    }

    #[test]
    fn table_name() {
        let n = Names::new("carPrice");
        assert_eq!(n.view_mod, "car_prices");
    }
}
