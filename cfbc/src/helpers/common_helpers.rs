use handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext, RenderError,
};
use inflector::Inflector;

fn helper_screaming_snake_case(
    h: &Helper,
    _r: &Handlebars,
    _ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for screaming_snake_case"))?;

    out.write(param.value().render().to_screaming_snake_case().as_ref())
        .map_err(Into::into)
}

pub fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper(
        "screaming_snake_case",
        Box::new(helper_screaming_snake_case),
    );
}
