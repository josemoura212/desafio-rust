// use std::collections::HashMap;

pub trait TEntidade {
    fn id(&self) -> i32;
    fn metadata() -> Vec<(&'static str, &'static str, &'static str)>;
    fn generate_sql_create_table() -> String;
    fn generate_sql_drop_table() -> String;
    fn generate_sql_insert() -> String;
    fn generate_sql_update() -> String;
    fn generate_sql_delete() -> String;
    fn generate_sql_select() -> String;

    // fn campos_model() -> Vec<(String, String)>;
    // fn from_data_row(data: HashMap<String, Value>) -> Result<Box<Self>, String>;
}