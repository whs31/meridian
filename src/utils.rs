use std::sync::Mutex;
use once_cell::sync::Lazy;

pub type StaticHeapObject<T> = Lazy<Mutex<Box<T>>>;