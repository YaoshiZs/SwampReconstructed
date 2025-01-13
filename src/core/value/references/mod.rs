use crate::core::value::value::ValueType;

pub mod map;

pub mod array;
pub mod function;
pub mod unique;
pub mod class;
pub mod object;

pub trait Param {
    fn _type(&self) -> ValueType;
    fn id(&self) -> &str;
}
