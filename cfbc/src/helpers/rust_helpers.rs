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
        let name = field.get("name").and_then(Value::as_str).unwrap();
        let default_integer = field
            .get("default_integer")
            .and_then(Value::as_i64)
            .unwrap();
        let default_real = field.get("default_real").and_then(Value::as_f64).unwrap();

        match field.get("type").unwrap() {
            Value::String(type_name) => match type_name.as_ref() {
                "Bool" => {
                    if default_integer != 0 {
                        out.write("!")?;
                    }
                    out.write("self.")?;
                    out.write(&name)
                }
                "Byte" | "UByte" | "Short" | "UShort" | "Int" | "UInt" | "Long" | "ULong" => {
                    out.write("self.")?;
                    out.write(&name)?;
                    out.write(" != ")?;
                    out.write(&format!("{}", default_integer).to_string())
                }
                "Float" | "Double" => {
                    out.write("self.")?;
                    out.write(&name)?;
                    out.write(" != ")?;
                    out.write(&format!("{}", default_real).to_string())
                }
                _ => {
                    out.write("self.")?;
                    out.write(&name)?;
                    out.write(".is_some()")
                }
            },
            json => out.write(json.render().as_ref()),
        }
        .map_err(Into::into)
    }
}

#[derive(Clone)]
struct FieldDefaultHelper;

impl HelperDef for FieldDefaultHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let field = h.param(0).unwrap().value();
        let default_integer = field
            .get("default_integer")
            .and_then(Value::as_i64)
            .unwrap();
        let default_real = field.get("default_real").and_then(Value::as_f64).unwrap();

        match field.get("type").unwrap() {
            Value::String(type_name) => match type_name.as_ref() {
                "Bool" => {
                    if default_integer == 0 {
                        out.write("false")
                    } else {
                        out.write("true")
                    }
                }
                "Byte" | "UByte" | "Short" | "UShort" | "Int" | "UInt" | "Long" | "ULong" => {
                    out.write(&format!("{}", default_integer).to_string())
                }
                "Float" | "Double" => out.write(&format!("{}", default_real).to_string()),
                _ => out.write("None"),
            },
            json => out.write(json.render().as_ref()),
        }
        .map_err(Into::into)
    }
}

pub fn register_helpers(reg: &mut Handlebars, schema: &Arc<Schema>) {
    reg.register_helper("rust_type", Box::new(TypeHelper(Arc::clone(&schema))));
    reg.register_helper("rust_is_field_present", Box::new(IsFieldPresentHelper));
    reg.register_helper("rust_field_default", Box::new(FieldDefaultHelper));
}
