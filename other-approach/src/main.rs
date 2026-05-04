// Downscaled CBOR encoder trying out another approach

use std::{cell::RefCell, rc::Rc};

trait CoreTrait {
    fn to_string(&self) -> String;
}

#[derive(Clone, Debug)]
struct ArrayContent {
    vector: Rc<RefCell<Vec<CBOR>>>
}

#[derive(Clone, Debug)]
struct IntContent {
    value: i64
}

impl CoreTrait for IntContent {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl CoreTrait for ArrayContent {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push('[');
        let array = self.vector.borrow();
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

#[derive(Clone, Debug)]
enum CBOR {
    Array(ArrayContent),
    Int(IntContent)
}

impl CBOR {
    pub fn new_array() -> CBOR {
        CBOR::Array(ArrayContent {vector: Rc::new(RefCell::new(Vec::new()))})
    }

    pub fn new_i64(value: i64) -> CBOR {
        CBOR::Int(IntContent {value: value})
    }

    pub fn get_i64(&self) -> i64 {
        match self {
            CBOR::Int(int_content) => int_content.value,
            _ => panic!("Not an integer: CBOR::{}", self.type_name())
        }
    }

    fn type_name(&self) -> &str {
        match self {
            CBOR::Array(_) => "Array",
            CBOR::Int(_) => "Int"
        }
    }

    pub fn get(&self, index: usize) -> CBOR {
        match self {
            CBOR::Array(array_content) => {
                (*array_content.vector.borrow())[index].clone()
            },
            _ => panic!("Not an array : CBOR::{}", self.type_name())
        }
    }

    pub fn add(&self, cbor_object: CBOR) -> CBOR {
        match self {
            CBOR::Array(array_content) => {
                array_content.vector.borrow_mut().push(cbor_object);
                self.clone()
            },
            _ => panic!("Not an array : CBOR::{}", self.type_name())
        }
    }

    pub fn add_ref(&self, cbor_object: &CBOR) -> CBOR {
        match self {
            CBOR::Array(array_content) => {
                array_content.vector.borrow_mut().push(cbor_object.clone());
                self.clone()
            },
            _ => panic!("Not an array : CBOR::{}", self.type_name())
        }
    }

    pub fn to_string(&self) -> String {
        self.as_trait().to_string()
    }

    fn as_trait(&self) -> &dyn CoreTrait {
        match self {
            CBOR::Array(core_trait) => core_trait,
            CBOR::Int(core_trait) => core_trait
        }
    }

    fn rc_count(&self) -> usize {
        match self {
            CBOR::Array(array_content) => {
                Rc::strong_count(&array_content.vector)
            },
            _ => panic!("Not an array : CBOR::{:?}", self)
        }
    }
}

fn update_array(array: &CBOR) {
    array.add(CBOR::new_i64(9))
         .add(CBOR::new_array().add(CBOR::new_i64(-177)));
}

fn main() {
    let root_array = CBOR::new_array();
    update_array(&root_array);
    let an_integer = CBOR::new_i64(6);
    let sub_array: CBOR = CBOR::new_array();
    sub_array.add(CBOR::new_i64(567));
    root_array.add_ref(&sub_array);
    sub_array.add(CBOR::new_i64(888));
    println!("integer = {}", an_integer.get_i64());
    root_array.add(an_integer).add(CBOR::new_i64(7));
    root_array.get(2).add(CBOR::new_i64(44));
    println!("root array: {}", root_array.to_string());
    println!("another array: {}", sub_array.to_string());
    println!("integer = {}", root_array.get(2).get(1).get_i64());
    assert_eq!(root_array.rc_count(), 1);
    assert_eq!(sub_array.rc_count(), 2);
    {
        let y = sub_array.add(CBOR::new_i64(3));
        assert_eq!(y.rc_count(), 3);
        assert_eq!(sub_array.rc_count(), 3);
    }
    assert_eq!(sub_array.rc_count(), 2);
    root_array.get_i64();  // Panic!
}
