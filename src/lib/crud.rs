pub trait CRUDRepository<Entity = Self> {
    fn save(&self) -> Result<&self, Error>;
    fn load() -> Result<&self, Error>;
    fn edit(&self, updatedEntity: Entity) -> Result<&self, Error>;
    fn delete(&self) -> Result<&self, Error>;
}
