use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use rocket::request::FromFormValue;
use std::fmt::Display;

#[derive(Debug)]
pub struct Field<'v,T> {
    value: Result<T,&'v str>,
    msg: Option<String>
}

impl<'v, T> Default for Field<'v, T> where T: Default {
    fn default() -> Field<'v, T> {
        Field {
            value: Ok(T::default()),
            msg: None
        }
    }
}

impl<'v, T> FromFormValue<'v> for Field<'v, T> where 
    T: FromFormValue<'v>, 
    <T as FromFormValue<'v>>::Error: Display
{
    type Error = ();
    fn from_form_value(form_value: &'v str) -> Result<Self,Self::Error> {
        match T::from_form_value(form_value) {
            Ok(v) => Ok(Field{
                value: Ok(v),
                msg: None
            }),
            Err(s) => Ok(Field{
                value: Err(form_value),
                msg: Some(s.to_string())
            })
        }
    }
}

impl<'v, T> Serialize for Field<'v,T> where T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let n = if self.msg.is_some() { 2 } else { 1 };
        let mut ser = serializer.serialize_struct("Field", n)?;
        match self.value {
            Result::Ok(ref v) => ser.serialize_field("value", &v)?,
            Result::Err(ref s) => ser.serialize_field("value", &s)?
        }
        match self.msg {
            Some(ref v) => ser.serialize_field("msg", &v)?,
            None => ()
        }
        ser.end()
    }
}

impl<'v, T> Field<'v, T> {
    pub fn is_ok(&self) -> bool { self.value.is_ok() }
    pub fn is_err(&self) -> bool { self.value.is_err() }
    pub fn has_msg(&self) -> bool { self.msg != None }
    pub fn get(&self) -> Result<&T,&&'v str> { self.value.as_ref() }
    pub fn set_msg(&mut self, msg: String) { self.msg = Some(msg) }
}
