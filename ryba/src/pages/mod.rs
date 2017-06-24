pub mod index;
pub mod register;

use context::{Req, Login, Context};
use rocket::request::Form;
use rocket::response::Redirect;
use ryba_kit::template::Template;



macro login_handler($name:ident, $url:expr, $template:expr, $page:expr) 
{
#[post($url, data="<data>")]
fn $name<'a>(req: Req, data: Form<'a, Login>) -> Result<Redirect, Template> {
    let mut form = data.into_inner();

    let test_name : String = "foo".to_string();
    let test_password : String = "bar".to_string();

    if form.name.is_ok() && form.password.is_ok()
    {
        if form.name.get() != Ok(&test_name) {
            form.name.set_msg("user not found".to_string());
        } else if form.password.get() != Ok(&test_password) {
            form.password.set_msg("password not match".to_string());
        } else {
            return Ok(Redirect::to($url))
        }
    }
    
    let mut ctx = Context::new(req, $page);
    ctx.site.login = form;
    Err(Template::render($template, ctx))
}

}