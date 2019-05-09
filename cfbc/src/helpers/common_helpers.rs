use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
use inflector::Inflector;

fn screaming_snake_case(
    h: &Helper,
    _r: &Handlebars,
    _ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    out.write(
        h.param(0)
            .unwrap()
            .value()
            .render()
            .to_screaming_snake_case()
            .as_ref(),
    )
    .map_err(Into::into)
}

pub fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper("screaming_snake_case", Box::new(screaming_snake_case));
}
