use crate::reflection_generated::reflection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub enum Type {
    UType,
    Bool,
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,
    String,

    Vector(Box<Type>),
    Obj(usize),
    Union(usize),
}

#[derive(Serialize)]
pub struct Schema {
    namespaces: Vec<String>,
    objects: Vec<Object>,
    enums: Vec<Enum>,
}

#[derive(Serialize)]
pub struct Object {
    name: String,
    fields: Vec<Field>,
    is_struct: bool,
    minalign: i32,
    bytesize: i32,
    attributes: HashMap<String, String>,
    documentation: Vec<String>,
}

#[derive(Serialize)]
pub struct Field {
    name: String,
    r#type: Type,
    id: u16,
    offset: u16,
    default_integer: i64,
    default_real: f64,
    deprecated: bool,
    required: bool,
    key: bool,
    attributes: HashMap<String, String>,
    documentation: Vec<String>,
}

#[derive(Serialize)]
pub struct Enum {
    name: String,
    values: Vec<EnumVal>,
    is_union: bool,
    underlying_type: Type,
    attributes: HashMap<String, String>,
    documentation: Vec<String>,
}

#[derive(Serialize)]
pub struct EnumVal {
    name: String,
    value: i64,
    union_type: Option<Object>,
    documentation: Vec<String>,
}

impl Schema {
    pub fn from_bytes(bytes: &[u8]) -> Schema {
        reflection::get_root_as_schema(bytes).into()
    }
}

impl<'a> From<reflection::Schema<'a>> for Schema {
    fn from(schema: reflection::Schema<'a>) -> Schema {
        let name = if schema.objects().len() > 0 {
            schema.objects().get(0).name()
        } else if schema.enums().len() > 0 {
            schema.enums().get(0).name()
        } else {
            ""
        };

        let mut namespaces: Vec<_> = name.split('.').map(str::to_string).collect();
        namespaces.pop();

        Schema {
            namespaces,
            objects: vec![],
            enums: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::{FlatBufferBuilder, WIPOffset};

    #[test]
    fn test_empty_schema_namespaces() {
        let schema = {
            let mut builder = FlatBufferBuilder::new();
            let objects_offset = builder.create_vector::<WIPOffset<reflection::Object>>(&[]);
            let enums_offset = builder.create_vector::<WIPOffset<reflection::Enum>>(&[]);

            let root_offset = {
                let mut schema_builder = reflection::SchemaBuilder::new(&mut builder);
                schema_builder.add_objects(objects_offset);
                schema_builder.add_enums(enums_offset);
                schema_builder.finish()
            };
            builder.finish_minimal(root_offset);
            Schema::from_bytes(builder.finished_data())
        };
        assert!(schema.namespaces.is_empty());
    }

    #[test]
    fn test_schema_namespaces_from_object() {
        let schema = {
            let mut builder = FlatBufferBuilder::new();
            let object_offset = {
                let name = builder.create_string("foo.bar");
                let fields_offset = builder.create_vector::<WIPOffset<reflection::Field>>(&[]);

                let mut object_builder = reflection::ObjectBuilder::new(&mut builder);
                object_builder.add_name(name);
                object_builder.add_fields(fields_offset);
                object_builder.finish()
            };
            let objects_offset = builder.create_vector(&[object_offset]);
            let enums_offset = builder.create_vector::<WIPOffset<reflection::Enum>>(&[]);

            let root_offset = {
                let mut schema_builder = reflection::SchemaBuilder::new(&mut builder);
                schema_builder.add_objects(objects_offset);
                schema_builder.add_enums(enums_offset);
                schema_builder.finish()
            };
            builder.finish_minimal(root_offset);
            Schema::from_bytes(builder.finished_data())
        };
        assert_eq!(vec!["foo"], schema.namespaces);
    }

    #[test]
    fn test_schema_namespaces_from_enum() {
        let schema = {
            let mut builder = FlatBufferBuilder::new();
            let enum_offset = {
                let name = builder.create_string("foo.bar.z");
                let values_offset = builder.create_vector::<WIPOffset<reflection::EnumVal>>(&[]);
                let underlying_type_offset = reflection::TypeBuilder::new(&mut builder).finish();

                let mut enum_builder = reflection::EnumBuilder::new(&mut builder);
                enum_builder.add_name(name);
                enum_builder.add_values(values_offset);
                enum_builder.add_underlying_type(underlying_type_offset);
                enum_builder.finish()
            };
            let objects_offset = builder.create_vector::<WIPOffset<reflection::Object>>(&[]);
            let enums_offset = builder.create_vector(&[enum_offset]);

            let root_offset = {
                let mut schema_builder = reflection::SchemaBuilder::new(&mut builder);
                schema_builder.add_objects(objects_offset);
                schema_builder.add_enums(enums_offset);
                schema_builder.finish()
            };
            builder.finish_minimal(root_offset);
            Schema::from_bytes(builder.finished_data())
        };
        assert_eq!(vec!["foo", "bar"], schema.namespaces);
    }

    #[test]
    fn test_empty_schema_namespaces_from_object() {
        let schema = {
            let mut builder = FlatBufferBuilder::new();
            let object_offset = {
                let name = builder.create_string("foo");
                let fields_offset = builder.create_vector::<WIPOffset<reflection::Field>>(&[]);

                let mut object_builder = reflection::ObjectBuilder::new(&mut builder);
                object_builder.add_name(name);
                object_builder.add_fields(fields_offset);
                object_builder.finish()
            };
            let objects_offset = builder.create_vector(&[object_offset]);
            let enums_offset = builder.create_vector::<WIPOffset<reflection::Enum>>(&[]);

            let root_offset = {
                let mut schema_builder = reflection::SchemaBuilder::new(&mut builder);
                schema_builder.add_objects(objects_offset);
                schema_builder.add_enums(enums_offset);
                schema_builder.finish()
            };
            builder.finish_minimal(root_offset);
            Schema::from_bytes(builder.finished_data())
        };
        assert!(schema.namespaces.is_empty());
    }
}
