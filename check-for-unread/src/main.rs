// Downscaled CBOR encoder supporting "check_for_unread()"

use std::{cell::RefCell, rc::Rc};

trait CoreTrait {
    fn to_string(&self) -> String;

    fn is_primitive(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
struct ArrayContent {
    // Specific
    vector: Rc<RefCell<Vec<CBOR>>>,
    // Common
    read_flag: Rc<RefCell<bool>>
}

#[derive(Clone, Debug)]
struct IntContent {
    // Specific
    value: i64,
    // Common
    read_flag: Rc<RefCell<bool>>
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

    fn is_primitive(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
enum CBOR {
    Array(ArrayContent),
    Int(IntContent)
}

impl CBOR {
    fn init_read_flag() -> Rc<RefCell<bool>> {
        Rc::new(RefCell::new(false))
    }

    pub fn new_array() -> CBOR {
        CBOR::Array(ArrayContent {vector: Rc::new(RefCell::new(Vec::new())),
                                  read_flag: Self::init_read_flag()})
    }

    pub fn new_int64(value: i64) -> CBOR {
        CBOR::Int(IntContent {value: value, read_flag: Self::init_read_flag()})
    }

    pub fn get_int64(&self) -> i64 {
        self.mark_as_read(true);
        match self {
            CBOR::Int(int_content) => int_content.value,
            _ => panic!("Not an integer: CBOR::{}", self.type_name())
        }
    }

    fn mark_as_read(&self, unconditional: bool) {
        if !self.as_core_trait().is_primitive() || unconditional {
            *self.as_read_flag_ref().borrow_mut() = true;
        }
    }

    fn is_read(&self) -> bool {
        *self.as_read_flag_ref().borrow()
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
                let cbor_object = (*array_content.vector.borrow())[index].clone();
                cbor_object.mark_as_read(false);
                cbor_object
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
        self.as_core_trait().to_string()
    }

    fn as_core_trait(&self) -> &dyn CoreTrait {
        match self {
            CBOR::Array(core_trait) => core_trait,
            CBOR::Int(core_trait) => core_trait
        }
    }

    fn as_read_flag_ref(&self) -> &RefCell<bool> {
        match self {
            CBOR::Array(core_trait) => &core_trait.read_flag,
            CBOR::Int(core_trait) => &core_trait.read_flag
        }
    }

    fn length(&self) -> usize {
        match self {
            CBOR::Array(array_content) => {
                array_content.vector.borrow_mut().len()
            },
            _ => panic!("Not an array : CBOR::{}", self.type_name())
        }
    }

    fn rc_count(&self) -> usize {
        match self {
            CBOR::Array(array_content) => {
                Rc::strong_count(&array_content.vector)
            },
            _ => panic!("Not an array : CBOR::{}", self.type_name())
        }
    }

    fn traverse(&self) -> bool {
        match self {
            CBOR::Array(array) => {
                let elements = (*array).vector.borrow();
                let mut n = 0;
                while n < elements.len() {
                    if !elements[n].traverse() {
                        return false;
                    }
                    n += 1;
                }
            },
            _ => ()
        };
        self.is_read()
    }

    pub fn check_for_unread(&self) -> bool {
        // Top-level Map, Array, and Tag are considered as read.
        self.mark_as_read(false);
        self.traverse()
    }
}

fn update_array(array: &CBOR) {
    array.add(CBOR::new_int64(9))
         .add(CBOR::new_array().add(CBOR::new_int64(-177)));
}

fn main() {
    let root_array = CBOR::new_array();
    update_array(&root_array);
    let an_integer = CBOR::new_int64(6);
    let sub_array: CBOR = CBOR::new_array();
    sub_array.add(CBOR::new_int64(567));
    root_array.add_ref(&sub_array);
    sub_array.add(CBOR::new_int64(888));
    let clone_int = an_integer.clone();
    assert_eq!(an_integer.is_read(), false);
    println!("integer = {}", an_integer.get_int64());
    assert_eq!(clone_int.is_read(), true);
    root_array.add(an_integer).add(CBOR::new_int64(7));
    root_array.get(2).add(CBOR::new_int64(44));
    println!("root array: {}", root_array.to_string());
    println!("another array: {}", sub_array.to_string());
    println!("integer = {}", root_array.get(2).get(1).get_int64());
    assert_eq!(root_array.rc_count(), 1);
    assert_eq!(sub_array.rc_count(), 2);
    {
        let y = sub_array.add(CBOR::new_int64(3));
        assert_eq!(y.rc_count(), 3);
        assert_eq!(sub_array.rc_count(), 3);
    }
    assert_eq!(sub_array.rc_count(), 2);
    assert_eq!(root_array.length(), 5);
    assert_eq!(root_array.to_string(), root_array.clone().to_string());
    assert_eq!(root_array.rc_count(), 1);

    let some_int = CBOR::new_int64(67);
    // Unread Int
    assert_eq!(some_int.check_for_unread(), false);
    let _ = some_int.get_int64();
    assert_eq!(some_int.check_for_unread(), true);

    // Root [] no get*() nessessary
    let some_array = CBOR::new_array();
    assert_eq!(some_array.check_for_unread(), true);

    // Unread [99]
    let updated_array = some_array.add(CBOR::new_int64(99));
    assert_eq!(updated_array.check_for_unread(), false);
    assert_eq!(some_array.check_for_unread(), false);
    // Get only fetches
    let fetched_array_element = updated_array.get(0);
    assert_eq!(some_array.check_for_unread(), false);
    assert_eq!(fetched_array_element.check_for_unread(), false);
    // Actual read
    let _ = fetched_array_element.get_int64();
    assert_eq!(some_array.check_for_unread(), true);

    // Must access all elements in an array, including empty arrays: [99,[]]
    let _ = some_array.add(CBOR::new_array());
    assert_eq!(some_array.check_for_unread(), false);
    assert_eq!(updated_array.check_for_unread(), false);
    let _ = updated_array.get(1);
    assert_eq!(some_array.check_for_unread(), true);

    root_array.get_int64();  // Panic!
}
