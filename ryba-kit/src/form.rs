use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use rocket::request::FromFormValue;
use std::fmt::Display;
use rocket::http::RawStr;

pub struct Field<'a, T> {
    pub raw_value: &'a str,
    pub value: Result<T, String>,
}


// TODO: convert error values
//                       value: Err(URI::percent_decode_lossy(form_value.as_bytes()).into_owned()),

impl<'a, T: FromFormValue<'a>> FromFormValue<'a> for Field<'a, T>
where
    <T as FromFormValue<'a>>::Error: Display,
{
    type Error = !;
    fn from_form_value(form_value: &'a RawStr) -> Result<Self, Self::Error> {
        let value = T::from_form_value(form_value);
        Ok(Field {
            raw_value: form_value,
            value: value.map_err(|e| e.to_string()),
        })
    }
}

#[derive(Debug)]
pub struct ContextField<T> {
    pub value: Result<T, String>,
    pub msg: Option<String>,
}

impl<'v, T> Serialize for ContextField<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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
where
    T: Clone,
{
    pub fn new(form_field: &Field<T>) -> ContextField<T> {
        match form_field.value {
            Ok(ref v) => ContextField {
                value: Ok(v.clone()),
                msg: None,
            },
            Err(ref e) => ContextField {
                value: Err(form_field.raw_value.to_string()),
                msg: Some(e.clone()),
            },
        }
    }
}

impl<'a, T> From<&'a Field<'a, T>> for ContextField<T>
where
    T: Clone,
{
    fn from(form_field: &'a Field<'a, T>) -> ContextField<T> {
        ContextField::<T>::new(form_field)
    }
}


impl<'a, T> Default for ContextField<T>
where
    T: FromFormValue<'a>,
    T: Default,
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
