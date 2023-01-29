pub struct Error;

pub trait CRUDRepository: Sized {
    fn save(&self) -> Result<Self, Error>;
    fn load() -> Result<Self, Error>;
    fn edit(&self, updated_entity: Self) -> Result<Self, Error>;
    fn delete(&self) -> Result<Self, Error>;
}
