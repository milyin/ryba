use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use rocket::request::FromFormValue;
use std::fmt::Display;
use rocket::http::uri::*;

#[derive(Debug)]
pub struct Field<T> {
    value: Result<T, String>,
    msg: Option<String>,
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
    pub fn set_msg(&mut self, msg: String) {
        self.msg = Some(msg)
    }
}
