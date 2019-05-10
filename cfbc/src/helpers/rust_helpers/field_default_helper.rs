use cfb_schema::{Field, Schema, Type};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use std::sync::Arc;

pub struct FieldDefaultHelper(pub Arc<Schema>);

impl HelperDef for FieldDefaultHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars,
        _ctx: &Context,
        _rc: &mut RenderContext<'reg>,
        out: &mut Output,
    ) -> HelperResult {
        let param = h
            .param(0)
            .ok_or_else(|| RenderError::new("rust_field_default: param 0 is required"))?
            .value();
        let field: Field = serde_json::from_value(param.clone())
            .map_err(|_| RenderError::new("rust_field_default: param 0 must be a field"))?;

        write_field_default(&field, &self.0, out)
    }
}

pub fn write_field_default(field: &Field, schema: &Schema, out: &mut dyn Output) -> HelperResult {
    match field.r#type {
        Type::Bool => {
            if field.default_integer == 0 {
                out.write("false")
            } else {
                out.write("true")
            }
        }
        Type::Byte
        | Type::UByte
        | Type::Short
        | Type::UShort
        | Type::Int
        | Type::UInt
        | Type::Long
        | Type::ULong => out.write(&format!("{}", field.default_integer).to_string()),
        Type::Float | Type::Double => out.write(&format!("{:?}", field.default_real).to_string()),
        Type::Enum(index) => {
            let e = &schema.enums[index];
            let def = e
                .values
                .iter()
                .find(|v| v.value == field.default_integer)
                .or_else(|| e.values.iter().next())
                .expect("enum default value");
            out.write(&e.name)?;
            out.write("::")?;
            out.write(&def.name)
        }
        Type::Obj(index) => {
            if schema.objects[index].is_struct {
                out.write("Default::default()")
            } else {
                out.write("None")
            }
        }
        _ => out.write("None"),
    }
    .map_err(Into::into)
}
