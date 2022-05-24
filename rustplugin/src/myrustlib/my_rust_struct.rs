use super::my_foreign_interface::MyForeignInterface;

pub struct MyRustStruct {
    my_int: i32,
    my_string: String,
}

impl MyRustStruct {
    pub fn new() -> MyRustStruct {
        MyRustStruct{
            my_int: 0,
            my_string: String::new(),
        }
    }
}

impl MyForeignInterface for MyRustStruct {
    fn empty_call(&mut self) {
        println!("MyRustStruct::empty_call was called");
    }

    fn set_int(&mut self, int_to_set: i32) {
        println!("MyRustStruct::set_int was called with: {}", int_to_set);
        self.my_int = int_to_set;
    }

    fn get_int(&mut self) -> i32 {
        println!("MyRustStruct::get_int was called. Returning: {}", self.my_int);
        self.my_int
    }

    fn set_string(&mut self, string_to_set: String) {
        println!("MyRustStruct::set_string was called with: {}", string_to_set);
        self.my_string = string_to_set;
    }

    fn get_string<'a>(&'a mut self) -> String {
        println!("MyRustStruct::get_string was called");
        return self.my_string.clone();
    }

    fn set_vector(&mut self, vector_to_set: &[&str]) {
        println!("MyRustStruct::set_vector was called with: {:?}", vector_to_set);
    }

    fn get_vector(&mut self) -> &[&str] {
        println!("MyRustStruct::get_vector was called");
        // return vec![String::from("some"),String::from("string"),String::from("vector"),String::from("from"),String::from("rust")];
        return &["some", "string", "string", "string", "string", "string"]
    }
}
