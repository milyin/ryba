use handlebars::*;

#[derive(PartialEq)]
enum Kind { Object, Array, String, Number }

struct IsKind {
    kind: Kind
}

impl HelperDef for IsKind {
    fn call(&self, h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        let param = h.param(0).ok_or(RenderError::new("Param expected for helper"))?;
        match if 
                self.kind==Kind::Object && param.value().is_object() ||
                self.kind==Kind::Array && param.value().is_array() ||
                self.kind==Kind::String && param.value().is_string() ||
                self.kind==Kind::Number && param.value().is_number()
            { h.template() } else { h.inverse() }
        {
            Some(ref t) => t.render(r,rc),
            None => Ok(())
        }
    }
}

fn include_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("Param expected for helper"))?;
    match param.value().as_str() {
        Some(s) => {
            match r.get_template(s) {
                Some(t) => t.render(r,rc),
                None => Err(RenderError::new("Template not found"))
            }
        }
        None => Err(RenderError::new("String parameter expected"))
    }
}

pub fn add_helpers(hb: &mut Handlebars)
{
    hb.register_helper("if_object", Box::new(IsKind {kind: Kind::Object} ));
    hb.register_helper("if_array", Box::new(IsKind {kind: Kind::Array} ));
    hb.register_helper("if_string", Box::new(IsKind {kind: Kind::String} ));
    hb.register_helper("if_number", Box::new(IsKind {kind: Kind::Number} ));
    hb.register_helper("include", Box::new(include_helper));
}