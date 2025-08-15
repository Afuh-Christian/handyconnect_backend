
pub trait ColumnsAndPlaceholdersTrait {
    fn column_names() -> Vec<&'static str>;
    fn placeholders() -> Vec<&'static str>;
    fn values(&self) -> Vec<String>;
}