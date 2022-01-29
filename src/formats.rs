use std::{fmt::{Debug, Display}, any::type_name};

pub trait MaybeFormat {}

pub trait Format<T> {
    fn to_string(t: &T) -> String;
}

pub struct DebugFormat;
impl MaybeFormat for DebugFormat {}
impl <T: Debug> Format<T> for DebugFormat {
    fn to_string(t: &T) -> String {
        format!("{:?}", t)
    }
}

pub struct DisplayFormat;
impl MaybeFormat for DisplayFormat {}
impl <T: Display> Format<T> for DisplayFormat {
    fn to_string(t: &T) -> String {
        format!("{}", t)
    }
}

pub struct TypeNameFormat;
impl MaybeFormat for TypeNameFormat {}
impl <T> Format<T> for TypeNameFormat {
    fn to_string(_: &T) -> String {
        type_name::<T>().to_string()
    }
}