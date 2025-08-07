use std::marker::PhantomData;

use sqlx::PgPool;
use utils::op_result::OperationResult;

use crate::base_repo_trait::BaseRepoTrait;

pub trait FieldNames {
    fn field_names() -> &'static [&'static str];
}


pub struct BaseRepo<T , IdType>{

//   pub db: DatabaseConnection,
     pub table_name: String,
     pub db_pool: PgPool,
     _id: PhantomData<IdType>,
     _model: PhantomData<T>,


}


impl<T, IdType>  BaseRepo<T, IdType> where 
T : FieldNames,
{
    
    pub  fn new(db_pool: PgPool, table_name: String) -> Self {
        Self {
            db_pool,
            table_name,
            _id: PhantomData,
            _model: PhantomData,
        }
    }   
}


impl <T : FieldNames, IdType>BaseRepoTrait<T, IdType> for BaseRepo<T, IdType> {






    // Implement the methods defined in the BaseRepoTrait
    fn get_all(&self) -> Result<Vec<T>, String> {
        // Implementation here
        let fields = T::field_names();
        Ok(item)
    }




    fn create(&self, item: T) -> Result<T, String> {
        // Implementation here

        Ok(item)
    }

    fn get(&self, id: IdType) -> Result<String, String> {
        // Implementation here
        Ok("Item".to_string())
    }

    fn update(&self, id: IdType, item: T) -> Result<T, String> {
        // Implementation here
        Ok(item)
    }

    fn delete(&self, id: IdType) -> Result<OperationResult, String> {
        // Implementation here
          Ok(OperationResult{
            error : None , 
            success: true,
            message: Some("Items deleted successfully".to_string()),
        })
    }

    fn save_list(&self, items: Vec<T>) -> Result<OperationResult, String> {
        // Implementation here
          Ok(OperationResult{
            error : None , 
            success: true,
            message: Some("Items saved successfully".to_string()),
        })
    }

    fn delete_list(&self, ids: Vec<IdType>) -> Result<OperationResult, String> {
        // Implementation here
        Ok(OperationResult{
            error : None , 
            success: true,
            message: Some("Item deleted successfully".to_string()),
        })
    }
    
}