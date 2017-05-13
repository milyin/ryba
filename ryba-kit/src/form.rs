use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;
use rocket::request::Form;
use rocket::request::FromFormValue;

#[derive(Debug)]
pub struct Field<C,T,E> {
    control: C,
    name: &'static str,
    value: Result<T, E>,
    msg: Option<String>
}

impl<C, T, E> Serialize for Field<C, T, E> where C: Serialize, T: Serialize, E: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let n = 3 + if self.msg.is_some() { 1 } else { 0 };
        let mut s = serializer.serialize_struct("Field", n)?;
        s.serialize_field("control", &self.control)?;
        s.serialize_field("name", &self.name)?;
        match self.value {
            Result::Ok(ref v) => s.serialize_field("value", &v)?,
            Result::Err(ref e) => s.serialize_field("value", &e)?
        }
        match self.msg {
            Some(ref v) => s.serialize_field("msg", &v)?,
            None => ()
        }
        s.end()
    }
}

impl<C, T, E> Field<C, T, E> {
    pub fn ok(control: C, name: &'static str, v: &T) -> Self where T: Clone {
        Field {
            control: control,
            name: name,
            value: Result::Ok(v.clone()),
            msg: None
        }
    }
    pub fn err(control: C, name: &'static str, v: &E, msg: String) -> Self where E: Clone {
        Field {
            control: control,
            name: name,
            value: Result::Err(v.clone()),
            msg: Some(msg)
        }
    }
    pub fn is_ok(&self) -> bool { self.value.is_ok() }
    pub fn is_err(&self) -> bool { self.value.is_err() }
    pub fn has_msg(&self) -> bool { self.msg != None }
    pub fn get(&self) -> Result<&T,&E> { self.value.as_ref() }
    pub fn set_msg(&mut self, msg: String) { self.msg = Some(msg) }
}

#[macro_export]
macro_rules! ryba_form { ( 
    $nform:ident : $form:ident ($($nparam:ident : $tparam:ty),*) $nctx:ident : $ctx:ident {
        $( $tctrl:ty, $f_init_ctrl:expr, $nfield:ident : $tfield:ty , $f_form_to_ctx:expr, $tctx:ty),*
    } $f_finalize_ctx:expr ) => (
        
    #[derive(Default,FromForm)]
    struct $form {
        $($nfield : $tfield),*
    } 

    #[derive(Serialize)]
    struct $ctx {
        $($nfield : Field<$tctrl, $tctx, $tfield>),*
    } 

    impl $ctx {
        fn validate($nform: &$form, $($nparam : $tparam),*) -> $ctx {
            let mut $nctx = $ctx {
                $(
                    $nfield: {
                        let control = $f_init_ctrl();
                        match $f_form_to_ctx(&$nform.$nfield) {
                            Result::Ok(v) => Field::ok(control, stringify!($nfield), v),
                            Result::Err(msg) => Field::err(control, stringify!($nfield), &$nform.$nfield, msg)
                        }
                    } 
                ),*
            };
            $f_finalize_ctx;
            $nctx
        }
        fn is_err(&self) -> bool { return $( self.$nfield.is_err())||*; }
        fn is_ok(&self) -> bool { return $( self.$nfield.is_ok())&&*; }
        fn has_msg(&self) -> bool { return $( self.$nfield.has_msg())||*; }
    }
    
    )
}

#[derive(Serialize)]
pub struct Input {
    #[serde(rename="type")]
    input_type: &'static str
}

pub fn text() -> Input {
    Input { input_type: "text" }
}

pub fn password() -> Input {
    Input { input_type: "password" }
}

pub fn submit() -> Input {
    Input { input_type: "submit" }
}

pub fn pass<T>(x:T) -> Result<T,String> { Result::Ok(x) }
pub fn fail<T>(msg:String) -> Result<T,String> { Result::Err(msg) }
