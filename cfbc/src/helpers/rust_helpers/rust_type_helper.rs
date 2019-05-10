use cfb_schema::{Schema, Type};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use std::sync::Arc;

pub struct RustTypeHelper(pub Arc<Schema>);

impl HelperDef for RustTypeHelper {
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
            .ok_or_else(|| RenderError::new("rust_type: requires param 0"))?
            .value();
        let ty: Type = serde_json::from_value(param.clone())
            .map_err(|_| RenderError::new("rust_type: requires Type as param 0"))?;

        match ty {
            Type::Bool => out.write("bool"),
            Type::Byte => out.write("i8"),
            Type::UByte => out.write("u8"),
            Type::Short => out.write("i16"),
            Type::UShort => out.write("u16"),
            Type::Int => out.write("i32"),
            Type::UInt => out.write("u32"),
            Type::Long => out.write("i64"),
            Type::ULong => out.write("u64"),
            Type::Float => out.write("f32"),
            Type::Double => out.write("f64"),
            Type::String => out.write("Option<&'a str>"),
            Type::Enum(index) => out.write(&self.0.enums[index].name),
            _ => unreachable!(),
        }
        .map_err(Into::into)
    }
}
