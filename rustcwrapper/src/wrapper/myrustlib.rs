extern crate libc;
use rustplugin::myrustlib::my_foreign_interface::MyForeignInterface;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

pub struct MyForeignInterfaceWrapper {
    pub the_wrapped_struct: Box<dyn MyForeignInterface>,
}

fn c_char_to_string(input: *const c_char) -> Option<String> {
    let input = unsafe { CStr::from_ptr(input) };
    match input.to_str() {
        Ok(output) => {
            return Some(String::from(output));
        }
        Err(_) => {
            eprintln!("c_char_to_string failed to convert C string to Rust string");
            return None;
        }
    }
}

#[no_mangle]
pub extern "C" fn deallocate_rust_string(string_to_dealloc: *mut c_char) {
    unsafe {
        if string_to_dealloc.is_null() {
            return;
        }
        CString::from_raw(string_to_dealloc);
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_empty_call(
    myruststruct: *mut MyForeignInterfaceWrapper,
) {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        myruststruct.empty_call();
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_set_int(
    myruststruct: *mut MyForeignInterfaceWrapper,
    int_to_set: i32,
) {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        myruststruct.set_int(int_to_set);
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_get_int(
    myruststruct: *mut MyForeignInterfaceWrapper,
) -> i32 {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        return myruststruct.get_int();
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_set_string(
    myruststruct: *mut MyForeignInterfaceWrapper,
    string_to_set:  *const c_char,
) {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        myruststruct.set_string(c_char_to_string(string_to_set).unwrap());
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_get_string(
    myruststruct: *mut MyForeignInterfaceWrapper,
) -> *mut c_char {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        let result = myruststruct.get_string();
        let result = CString::new(result).unwrap();
        result.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn myruststruct_set_vector(
    myruststruct: *mut MyForeignInterfaceWrapper,
    vector_to_set: *const *const c_char,
    vector_size: usize
) {
    unsafe {
        let mut rust_vector = Vec::new();
        for index in 0..vector_size {
            let my_string = c_char_to_string(*vector_to_set.offset(index as isize) as *const c_char).unwrap();
            rust_vector.push(my_string);
        }
        
        let rust_vector: Vec<&str> = rust_vector.iter().map(|x| &**x).collect();
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        myruststruct.set_vector(&rust_vector);
    }
}


#[no_mangle]
pub extern "C" fn myruststruct_get_vector(
    myruststruct: *mut MyForeignInterfaceWrapper
) -> *const *const c_char {
    unsafe {
        let myruststruct = &mut (*myruststruct).the_wrapped_struct;
        let mut rust_vector = myruststruct.get_vector();
        let rust_vector: Vec<&str> = rust_vector.to_vec();
        let rust_vector: Vec<CString> = rust_vector
            .iter()
            .map(|entry| CString::new(entry.clone()).unwrap())
            .collect();
        let mut rust_vector: Vec<*const c_char> =
            rust_vector.iter().map(|arg| arg.as_ptr()).collect();
        rust_vector.push(std::ptr::null()); // null terminate the array of pointers.
        return rust_vector.as_ptr();
    }
}
