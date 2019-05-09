use cfb_schema::Schema;
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
struct TypeHelper(Arc<Schema>);

impl HelperDef for TypeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> HelperResult {
        match h.param(0).unwrap().value() {
            Value::String(type_name) => match type_name.as_ref() {
                "Bool" => out.write("bool"),
                "Byte" => out.write("i8"),
                "UByte" => out.write("u8"),
                "Short" => out.write("i16"),
                "UShort" => out.write("u16"),
                "Int" => out.write("i32"),
                "UInt" => out.write("u32"),
                "Long" => out.write("i64"),
                "ULong" => out.write("u64"),
                "Float" => out.write("f32"),
                "Double" => out.write("f64"),
                _ => out.write(type_name),
            },
            json => out.write(json.render().as_ref()),
        }
        .map_err(Into::into)
    }
}

#[derive(Clone)]
struct IsFieldPresentHelper;

impl HelperDef for IsFieldPresentHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let field = h.param(0).unwrap().value();
        let name = field.get("name").unwrap().render();

        out.write("self.")?;
        out.write(&name)?;

        match field.get("type").unwrap() {
            Value::String(type_name) => match type_name.as_ref() {
                "Bool" => Ok(()),
                "Byte" | "UByte" | "Short" | "UShort" | "Int" | "UInt" | "Long" | "ULong" => {
                    out.write(" != 0")
                }
                "Float" | "Double" => out.write(" != 0.0"),
                _ => out.write(".is_some()"),
            },
            json => out.write(json.render().as_ref()),
        }
        .map_err(Into::into)
    }
}

pub fn register_helpers(reg: &mut Handlebars, schema: &Arc<Schema>) {
    reg.register_helper("rust_type", Box::new(TypeHelper(Arc::clone(&schema))));
    reg.register_helper("rust_is_field_present", Box::new(IsFieldPresentHelper));
}
