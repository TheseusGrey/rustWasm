use crate::lib::crud::CRUDRepository;

pub struct ExampleData {
    text: String,
    value: usize,
}

pub struct CRUDError {}

impl CRUDRepository for ExampleData {
    fn save(&self) -> Result<Self, CRUDError> {
        todo!()
    }

    fn load() -> Result<Self, CRUDError> {
        todo!()
    }

    fn edit(&self, updated_entity: Self) -> Result<Self, CRUDError> {
        todo!()
    }

    fn delete(&self) -> Result<Self, CRUDError> {
        todo!()
    }
}
