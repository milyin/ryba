use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use rocket::request::FromFormValue;
use std::fmt::Display;
use rocket::http::uri::*;

#[derive(Debug)]
pub struct Field<T> {
    pub value: Result<T, String>,
    pub msg: Option<String>,
}

impl<T> Field<T> {
    pub fn new(value: T) -> Field<T> {
        Field {
            value: Ok(value),
            msg: None,
        }
    }
}

impl<T> Default for Field<T>
    where T: Default
{
    fn default() -> Field<T> {
        Field {
            value: Ok(T::default()),
            msg: None,
        }
    }
}

impl<'v, T> FromFormValue<'v> for Field<T>
    where T: FromFormValue<'v>,
          <T as FromFormValue<'v>>::Error: Display
{
    type Error = ();
    fn from_form_value(form_value: &'v str) -> Result<Self, Self::Error> {
        match T::from_form_value(form_value) {
            Ok(v) => {
                Ok(Field {
                       value: Ok(v),
                       msg: None,
                   })
            }
            Err(e) => {
                Ok(Field {
                       value: Err(URI::percent_decode_lossy(form_value.as_bytes()).into_owned()),
                       msg: Some(e.to_string()),
                   })
            }
        }
    }
}

impl<'v, T> Serialize for Field<T>
    where T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let n = if self.msg.is_some() { 2 } else { 1 };
        let mut ser = serializer.serialize_struct("Field", n)?;
        match self.value {
            Result::Ok(ref v) => ser.serialize_field("value", &v)?,
            Result::Err(ref s) => ser.serialize_field("value", &s)?,
        }
        match self.msg {
            Some(ref v) => ser.serialize_field("msg", &v)?,
            None => (),
        }
        ser.end()
    }
}

impl<T> Field<T> {
    pub fn is_ok(&self) -> bool {
        self.value.is_ok()
    }
    pub fn is_err(&self) -> bool {
        self.value.is_err()
    }
    pub fn has_msg(&self) -> bool {
        self.msg != None
    }
    pub fn get(&self) -> Result<&T, &String> {
        self.value.as_ref()
    }
    pub fn get_ok(&self) -> Option<&T> {
        if let Ok(ref v) = self.value {
            Some(v)
        } else {
            None
        }
    }
    pub fn set_msg(&mut self, msg: String) {
        self.msg = Some(msg)
    }
}

pub trait FieldForm {
    fn is_ok(&self) -> bool;
    fn is_err(&self) -> bool {
        !self.is_ok()
    }
    fn has_msg(&self) -> bool;
}


pub struct FormField<'a, T> {
    pub raw_value: &'a str,
    pub value: Result<T, String>,
}

impl<'a, T: FromFormValue<'a>> FromFormValue<'a> for FormField<'a, T>
    where <T as FromFormValue<'a>>::Error: Display
{
    type Error = !;
    fn from_form_value(form_value: &'a str) -> Result<Self, Self::Error> {
        let value = T::from_form_value(form_value);
        Ok(FormField {
               raw_value: form_value,
               value: value.map_err(|e| e.to_string()),
           })
    }
}

pub struct ContextField<T> {
    pub value: Result<T, String>,
    pub msg: Option<String>,
}

impl<'v, T> Serialize for ContextField<T>
    where T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let n = if self.msg.is_some() { 2 } else { 1 };
        let mut ser = serializer.serialize_struct("ContextField", n)?;
        match self.value {
            Result::Ok(ref v) => ser.serialize_field("value", &v)?,
            Result::Err(ref s) => ser.serialize_field("value", &s)?,
        }
        match self.msg {
            Some(ref v) => ser.serialize_field("msg", &v)?,
            None => (),
        }
        ser.end()
    }
}

impl<T> ContextField<T>
    where T: Clone
{
    pub fn new(form_field: &FormField<T>) -> ContextField<T> {
        match form_field.value {
            Ok(ref v) => {
                ContextField {
                    value: Ok(v.clone()),
                    msg: None,
                }
            }
            Err(ref e) => {
                ContextField {
                    value: Err(form_field.raw_value.to_string()),
                    msg: Some(e.clone()),
                }
            }
        }
    }
}

impl<'a, T> From<&'a FormField<'a, T>> for ContextField<T>
    where T: Clone
{
    fn from(form_field: &'a FormField<'a, T>) -> ContextField<T> {
        ContextField::<T>::new(form_field)
    }
}


impl<'a, T> Default for ContextField<T>
    where T: FromFormValue<'a>,
          T: Default
{
    fn default() -> ContextField<T> {
        let v = match <T as FromFormValue<'a>>::default() {
            Some(v) => v,
            None => <T as Default>::default(),
        };
        ContextField {
            value: Ok(v),
            msg: None,
        }
    }
}
