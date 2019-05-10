use super::field_default_helper::write_field_default;
use cfb_schema::{Field, Schema, Type};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use std::sync::Arc;

pub struct IsFieldPresentHelper(pub Arc<Schema>);

impl HelperDef for IsFieldPresentHelper {
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
            .ok_or_else(|| RenderError::new("rust_is_field_present: param 0 is required"))?
            .value();
        let field: Field = serde_json::from_value(param.clone())
            .map_err(|_| RenderError::new("rust_is_field_present: param 0 must be a field"))?;

        match field.r#type {
            Type::Bool => {
                if field.default_integer != 0 {
                    out.write("!")?;
                }
                out.write("self.")?;
                out.write(&field.name)
            }
            Type::Byte
            | Type::UByte
            | Type::Short
            | Type::UShort
            | Type::Int
            | Type::UInt
            | Type::Long
            | Type::ULong
            | Type::Float
            | Type::Double
            | Type::Enum(_) => {
                out.write("self.")?;
                out.write(&field.name)?;
                out.write(" != ")?;
                write_field_default(&field, &self.0, out)?;
                Ok(())
            }
            Type::String => {
                out.write("self.")?;
                out.write(&field.name)?;
                out.write(".map(str::is_empty) == Some(false)")
            }
            _ => {
                out.write("self.")?;
                out.write(&field.name)?;
                out.write(".is_some()")
            }
        }
        .map_err(Into::into)
    }
}
