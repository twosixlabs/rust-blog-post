

pub trait MyForeignInterface {
    fn empty_call(&mut self);
    
    fn set_int(&mut self, int_to_set: i32);
    fn get_int(&mut self) -> i32;
    
    fn set_string(&mut self, string_to_set: String);
    fn get_string(&mut self) -> String;
    
    fn set_vector(&mut self, vector_to_set: &[&str]);
    fn get_vector(&mut self) -> &[&str];
}