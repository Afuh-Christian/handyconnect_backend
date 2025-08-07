use utils::op_result::OperationResult;




pub trait BaseRepoTrait<T , IdType>
where 
T : FieldNames
{

        // Define the methods that will be implemented by the repositories
    fn get_all(&self) -> Result<Vec<T>, String>;
    fn create(&self, item: T) -> Result<T, String>;
    fn get(&self, id: IdType) -> Result<String, String>;
    fn update(&self, id: IdType, item: T) -> Result<T, String>;
    fn delete(&self, id: IdType) -> Result<OperationResult, String>;
    fn save_list(&self, items : Vec<T>) -> Result<OperationResult, String>;
    fn delete_list(&self, ids: Vec<IdType> ) -> Result<OperationResult, String>;
     


}


