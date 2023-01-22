pub trait CRUDRepository<Entity = Self> {
    fn save(&self) -> Result<Entity, Error>;
    fn load() -> Result<Entity, Error>;
    fn edit(&self, updatedEntity: Entity) -> Result<Entity, Error>;
    fn delete(&self) -> Result<Entity, Error>;
}
