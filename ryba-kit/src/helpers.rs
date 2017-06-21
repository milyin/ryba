use handlebars::*;
use serde_json::value::{Value, from_value};
use serde_json::Map;

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

fn get_path_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("path expected as 1st helper parameter"))?;
    let var_value = h.param(1).ok_or(RenderError::new("local lariable name expected as 2nd helper parameter"))?.value();
    let var_name = var_value.as_str().ok_or(RenderError::new("Can't get local variable name"))?;
    let path = param.path().ok_or(RenderError::new("path expected"))?;
    let mut local_rc = rc.derive();
    local_rc.set_local_var(var_name.to_string(), to_json(path));
    if let Some(t) = h.template() {
        t.render(r,&mut local_rc)
    } else {
        Ok(())
    }
}

fn get_path_leaf_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("path expected as 1st helper parameter"))?;
    let var_value = h.param(1).ok_or(RenderError::new("local lariable name expected as 2nd helper parameter"))?.value();
    let var_name = var_value.as_str().ok_or(RenderError::new("Can't get local variable name"))?;
    let path = param.path().ok_or(RenderError::new("path expected"))?;
    let path_leaf = path.split(|c| c=='.'||c=='/').last().ok_or(RenderError::new("Path is empty"))?;
    let mut local_rc = rc.derive();
    local_rc.set_local_var(var_name.to_string(), to_json(&path_leaf.to_string()));
    if let Some(t) = h.template() {
        t.render(r,&mut local_rc)
    } else {
        Ok(())
    }
}

/*fn annotate_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new("path expected as 1st helper parameter"))?;
    let data = rc.context_mut().data_mut();
    let obj = data.as_object_mut();
    if let Some(map) = rc.context_mut().data_mut().as_object_mut() {

    }


    Ok(())
}
*/

fn annotate_decorator( _: &Decorator, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    fn annotate_map(map: &mut Map<String,Value>) {
        for (k,v) in map {
            if let Some(ref mut m) = v.as_object_mut().as_mut() {
                annotate_map(**m);
                m.insert("@name".to_string(), to_json(&k));
            }
        }
    }
    if let Some(ref mut m) = rc.context_mut().data_mut().as_object_mut().as_mut() {
        annotate_map(m)
    }
    Ok(())
}

pub fn add_helpers(hb: &mut Handlebars)
{
    hb.register_helper("if_object", Box::new(IsKind {kind: Kind::Object} ));
    hb.register_helper("if_array", Box::new(IsKind {kind: Kind::Array} ));
    hb.register_helper("if_string", Box::new(IsKind {kind: Kind::String} ));
    hb.register_helper("if_number", Box::new(IsKind {kind: Kind::Number} ));
    hb.register_helper("include", Box::new(include_helper));
    hb.register_helper("get_path", Box::new(get_path_helper));
    hb.register_helper("get_path_leaf", Box::new(get_path_leaf_helper));
    hb.register_decorator("annotate", Box::new(annotate_decorator));
}