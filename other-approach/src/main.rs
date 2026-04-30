// Downscaled CBOR encoder trying out another approach

use std::{cell::RefCell, rc::Rc};

trait CoreTrait {
    fn to_string(&self) -> String;
}

#[derive(Clone)]
#[derive(Debug)]
struct ArrayContent {
    vector: Rc<RefCell<Vec<CBOR>>>
}

impl CoreTrait for ArrayContent {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push('[');
        let array = (*self.vector.borrow()).clone();
        let mut n = 0;
        while n < array.len() {
            if n > 0 {
                string.push(',');
            }
            string.push_str(&array[n].to_string());
            n += 1;
        }
        string.push(']');
        string
    }
}

impl ArrayContent {}

#[derive(Clone)]
#[derive(Debug)]
enum CBOR {
    Array(ArrayContent),
    Int(i64),
}

impl CBOR {
    pub fn new_array() -> CBOR {
        CBOR::Array(ArrayContent {vector: Rc::new(RefCell::new(Vec::new()))})
    }

    pub fn get_i64(&self) -> i64 {
        match self {
            CBOR::Int(value) => *value,
            _ => panic!("Not an integer: CBOR::{:?}", self)
        }
    }

    pub fn get(&self, index: usize) -> CBOR {
        match self {
            CBOR::Array(map_content) => {
                (*map_content.vector.borrow())[index].clone()
            },
            _ => panic!("Not an array : CBOR::{:?}", self)
        }
    }

    pub fn add(&self, cbor_object: CBOR) -> CBOR {
        match self {
            CBOR::Array(map_content) => {
                map_content.vector.borrow_mut().push(cbor_object);
                self.clone()
            },
            _ => panic!("Not an array : CBOR::{:?}", self)
        }
    }

    pub fn add_ref(&self, cbor_object: &CBOR) -> CBOR {
        match self {
            CBOR::Array(map_content) => {
                map_content.vector.borrow_mut().push(cbor_object.clone());
                self.clone()
            },
            _ => panic!("Not an array : CBOR::{:?}", self)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CBOR::Array(_) => self.as_trait().to_string(),
            CBOR::Int(value) => format!("{:?}", *value)
        }
    }

    fn as_trait(&self) -> &dyn CoreTrait {
        match self {
            CBOR::Array(b) => b,
            _ => panic!("Not permitted for: {:?}", self)
        }
    }
}

fn update_array(array: &CBOR) {
    array.add(CBOR::Int(9))
         .add(CBOR::new_array().add(CBOR::Int(-177)));
}

fn main() {
    let root_array = CBOR::new_array();
    update_array(&root_array);
    let an_integer = CBOR::Int(6);
    let mut another_array: CBOR = CBOR::new_array();
    another_array.add(CBOR::Int(567));
    root_array.add_ref(&another_array);
    another_array.add(CBOR::Int(888));
    println!("integer = {}", an_integer.get_i64());
    root_array.add(an_integer).add(CBOR::Int(7));
    root_array.get(2).add(CBOR::Int(44));
    println!("array structure: {}", root_array.to_string());
    println!("integer = {}", root_array.get(2).get(1).get_i64());
    root_array.get_i64();  // Panic!
}
