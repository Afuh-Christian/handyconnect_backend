pub trait HasId<IdType> {
    fn id(&self) -> &IdType;
}