



pub trait HasPrimaryKey<T> {
    fn primary_key_value(&self) -> T;
}
