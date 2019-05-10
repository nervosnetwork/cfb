use super::schema_context::SchemaContext;
use cfb_schema::{Field, Type};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

fn helper_rust_type(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let schema = SchemaContext::new(ctx);
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("rust_type: require param 0"))?
        .value();
    let ty: Type = serde_json::from_value(param.clone())
        .map_err(|_| RenderError::new("rust_type: param 0 must be a type"))?;

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
        Type::Enum(index) => out.write(schema.enum_name(index)),
        Type::Obj(index) => out.write(schema.object_name(index)),
        _ => unreachable!(),
    }
    .map_err(Into::into)
}

fn helper_rust_is_field_present(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let schema = SchemaContext::new(ctx);
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
            write_field_default(&field, &schema, out)?;
            Ok(())
        }
        _ => {
            out.write("self.")?;
            out.write(&field.name)?;
            out.write(".is_some()")
        }
    }
    .map_err(Into::into)
}

fn helper_rust_field_default(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let schema = SchemaContext::new(ctx);
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("rust_field_default: param 0 is required"))?
        .value();
    let field: Field = serde_json::from_value(param.clone())
        .map_err(|_| RenderError::new("rust_field_default: param 0 must be a field"))?;

    write_field_default(&field, &schema, out)
}

fn write_field_default<'a>(
    field: &Field,
    schema: &SchemaContext<'a>,
    out: &mut dyn Output,
) -> HelperResult {
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
            let e = schema.enum_at(index);
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
        _ => out.write("None"),
    }
    .map_err(Into::into)
}

pub fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper("rust_type", Box::new(helper_rust_type));
    reg.register_helper(
        "rust_is_field_present",
        Box::new(helper_rust_is_field_present),
    );
    reg.register_helper("rust_field_default", Box::new(helper_rust_field_default));
}
