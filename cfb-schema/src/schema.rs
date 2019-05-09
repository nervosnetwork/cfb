use crate::reflection_generated::reflection;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Eq, PartialEq, Debug)]
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

#[derive(Serialize, Debug)]
pub struct Schema {
    namespaces: Vec<String>,
    objects: Vec<Object>,
    enums: Vec<Enum>,
}

#[derive(Serialize, Debug)]
pub struct Object {
    name: String,
    fields: Vec<Field>,
    is_struct: bool,
    minalign: i32,
    bytesize: i32,
    attributes: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
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
}

#[derive(Serialize, Debug)]
pub struct Enum {
    name: String,
    values: Vec<EnumVal>,
    is_union: bool,
    underlying_type: Type,
    attributes: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct EnumVal {
    name: String,
    value: i64,
    union_type: Option<usize>,
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

        let objects = vector_into_iter(schema.objects()).map(Into::into).collect();
        let enums = vector_into_iter(schema.enums()).map(Into::into).collect();

        Schema {
            namespaces,
            objects,
            enums,
        }
    }
}

impl<'a> From<reflection::Object<'a>> for Object {
    fn from(o: reflection::Object<'a>) -> Object {
        let mut fields: Vec<Field> = vector_into_iter(o.fields()).map(Into::into).collect();
        fields.sort_by_key(|f| f.offset);

        Object {
            name: base_name(o.name()).to_string(),
            is_struct: o.is_struct(),
            minalign: o.minalign(),
            bytesize: o.bytesize(),
            attributes: collect_attributes(o.attributes()),
            fields,
        }
    }
}

impl<'a> From<reflection::Enum<'a>> for Enum {
    fn from(e: reflection::Enum<'a>) -> Enum {
        let mut values: Vec<EnumVal> = vector_into_iter(e.values()).map(Into::into).collect();
        values.sort_by_key(|f| f.value);

        Enum {
            values,
            name: base_name(e.name()).to_string(),
            is_union: e.is_union(),
            underlying_type: e.underlying_type().into(),
            attributes: collect_attributes(e.attributes()),
        }
    }
}

impl<'a> From<reflection::EnumVal<'a>> for EnumVal {
    fn from(ev: reflection::EnumVal<'a>) -> EnumVal {
        EnumVal {
            name: ev.name().to_string(),
            value: ev.value(),
            union_type: ev.union_type().map(|t| t.index() as usize),
        }
    }
}

impl<'a> From<reflection::Field<'a>> for Field {
    fn from(f: reflection::Field<'a>) -> Field {
        Field {
            name: f.name().to_string(),
            r#type: f.type_().into(),
            id: f.id(),
            offset: f.offset(),
            default_integer: f.default_integer(),
            default_real: f.default_real(),
            deprecated: f.deprecated(),
            required: f.required(),
            key: f.key(),
            attributes: collect_attributes(f.attributes()),
        }
    }
}

impl<'a> From<reflection::Type<'a>> for Type {
    fn from(t: reflection::Type<'a>) -> Type {
        match t.base_type() {
            reflection::BaseType::Vector => Type::Vector(Box::new(match t.element() {
                reflection::BaseType::Obj => Type::Obj(t.index() as usize),
                element => try_from_base_type(element),
            })),
            reflection::BaseType::Obj => Type::Obj(t.index() as usize),
            reflection::BaseType::Union => Type::Union(t.index() as usize),
            base_type => try_from_base_type(base_type),
        }
    }
}

fn try_from_base_type(t: reflection::BaseType) -> Type {
    match t {
        reflection::BaseType::UType => Type::UType,
        reflection::BaseType::Bool => Type::Bool,
        reflection::BaseType::Byte => Type::Byte,
        reflection::BaseType::UByte => Type::UByte,
        reflection::BaseType::Short => Type::Short,
        reflection::BaseType::UShort => Type::UShort,
        reflection::BaseType::Int => Type::Int,
        reflection::BaseType::UInt => Type::UInt,
        reflection::BaseType::Long => Type::Long,
        reflection::BaseType::ULong => Type::ULong,
        reflection::BaseType::Float => Type::Float,
        reflection::BaseType::Double => Type::Double,
        reflection::BaseType::String => Type::String,
        _ => unreachable!(),
    }
}

fn vector_into_iter<'a, 'v, T>(
    vec: flatbuffers::Vector<'a, T>,
) -> impl Iterator<Item = <T as flatbuffers::Follow>::Inner> + 'a
where
    T: flatbuffers::Follow<'a> + 'a,
{
    (0..vec.len()).map(move |i| vec.get(i))
}

fn collect_attributes<'a>(
    attributes: Option<
        flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<reflection::KeyValue<'a>>>,
    >,
) -> HashMap<String, String> {
    attributes
        .map(|vec| {
            vector_into_iter(vec)
                .map(|kv| {
                    (
                        kv.key().to_string(),
                        kv.value()
                            .map(str::to_string)
                            .unwrap_or_else(Default::default),
                    )
                })
                .collect()
        })
        .unwrap_or_else(Default::default)
}

fn base_name(name: &str) -> &str {
    name.rsplitn(2, '.').take(1).next().unwrap()
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
                let underlying_type_offset = reflection::Type::create(
                    &mut builder,
                    &reflection::TypeArgs {
                        base_type: reflection::BaseType::UByte,
                        ..Default::default()
                    },
                );

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

    #[test]
    fn test_convert_object() {
        let mut builder = FlatBufferBuilder::new();

        let field_name = builder.create_string("field");
        let field_type = reflection::Type::create(
            &mut builder,
            &reflection::TypeArgs {
                base_type: reflection::BaseType::UByte,
                ..Default::default()
            },
        );
        let field_offset = reflection::Field::create(
            &mut builder,
            &reflection::FieldArgs {
                name: Some(field_name),
                type_: Some(field_type),
                ..Default::default()
            },
        );

        let object_offset = {
            let name = builder.create_string("foo");
            let fields_offset = builder.create_vector(&[field_offset]);

            let mut object_builder = reflection::ObjectBuilder::new(&mut builder);
            object_builder.add_name(name);
            object_builder.add_fields(fields_offset);
            object_builder.finish()
        };
        builder.finish_minimal(object_offset);

        let o: Object = flatbuffers::get_root::<reflection::Object>(builder.finished_data()).into();
        assert_eq!("foo", o.name);
        assert_eq!(1, o.fields.len());
        assert_eq!("field", o.fields[0].name);
        assert_eq!(Type::UByte, o.fields[0].r#type);
    }

    #[test]
    fn test_convert_enum() {
        let mut builder = FlatBufferBuilder::new();

        let enum_val_name = builder.create_string("ev1");
        let enum_val_offset = reflection::EnumVal::create(
            &mut builder,
            &reflection::EnumValArgs {
                name: Some(enum_val_name),
                value: 1,
                ..Default::default()
            },
        );

        let enum_offset = {
            let name = builder.create_string("foo");
            let values_offset = builder.create_vector(&[enum_val_offset]);
            let enum_type = reflection::Type::create(
                &mut builder,
                &reflection::TypeArgs {
                    base_type: reflection::BaseType::UByte,
                    ..Default::default()
                },
            );

            let mut enum_builder = reflection::EnumBuilder::new(&mut builder);
            enum_builder.add_name(name);
            enum_builder.add_values(values_offset);
            enum_builder.add_underlying_type(enum_type);
            enum_builder.finish()
        };
        builder.finish_minimal(enum_offset);

        let e: Enum = flatbuffers::get_root::<reflection::Enum>(builder.finished_data()).into();
        assert_eq!("foo", e.name);
        assert_eq!(1, e.values.len());
        assert_eq!("ev1", e.values[0].name);
        assert_eq!(1, e.values[0].value);
    }

    #[test]
    fn test_object_name() {
        let mut builder = FlatBufferBuilder::new();
        {
            let object_args = reflection::ObjectArgs {
                name: Some(builder.create_string("foo.bar")),
                fields: Some(builder.create_vector::<WIPOffset<reflection::Field>>(&[])),
                ..Default::default()
            };
            let object = reflection::Object::create(&mut builder, &object_args);
            builder.finish_minimal(object);
        }

        let o: Object = flatbuffers::get_root::<reflection::Object>(builder.finished_data()).into();
        assert_eq!("bar", o.name);
    }

    #[test]
    fn test_enum_name() {
        let mut builder = FlatBufferBuilder::new();
        {
            let enum_args = reflection::EnumArgs {
                name: Some(builder.create_string("foo.bar")),
                values: Some(builder.create_vector::<WIPOffset<reflection::EnumVal>>(&[])),
                underlying_type: Some(reflection::Type::create(
                    &mut builder,
                    &reflection::TypeArgs {
                        base_type: reflection::BaseType::UByte,
                        ..Default::default()
                    },
                )),
                ..Default::default()
            };
            let e = reflection::Enum::create(&mut builder, &enum_args);
            builder.finish_minimal(e);
        }

        let e: Enum = flatbuffers::get_root::<reflection::Enum>(builder.finished_data()).into();
        assert_eq!("bar", e.name);
    }

    #[test]
    fn test_fields_sorted() {
        let mut builder = FlatBufferBuilder::new();
        {
            let field1_args = reflection::FieldArgs {
                name: Some(builder.create_string("field1")),
                type_: Some(reflection::Type::create(
                    &mut builder,
                    &reflection::TypeArgs {
                        base_type: reflection::BaseType::UByte,
                        ..Default::default()
                    },
                )),
                offset: 6,
                ..Default::default()
            };
            let field2_args = reflection::FieldArgs {
                name: Some(builder.create_string("field1")),
                type_: Some(reflection::Type::create(
                    &mut builder,
                    &reflection::TypeArgs {
                        base_type: reflection::BaseType::UByte,
                        ..Default::default()
                    },
                )),
                offset: 4,
                ..Default::default()
            };
            let field1 = reflection::Field::create(&mut builder, &field1_args);
            let field2 = reflection::Field::create(&mut builder, &field2_args);
            let fields = builder.create_vector(&[field1, field2]);
            let object_args = reflection::ObjectArgs {
                name: Some(builder.create_string("foo.bar")),
                fields: Some(fields),
                ..Default::default()
            };
            let object = reflection::Object::create(&mut builder, &object_args);
            builder.finish_minimal(object);
        }

        let o: Object = flatbuffers::get_root::<reflection::Object>(builder.finished_data()).into();
        assert!(o.fields[0].offset < o.fields[1].offset);
    }

    #[test]
    fn test_enum_vals_sorted() {
        let mut builder = FlatBufferBuilder::new();
        {
            let val1_args = reflection::EnumValArgs {
                name: Some(builder.create_string("field")),
                value: 6,
                ..Default::default()
            };
            let val2_args = reflection::EnumValArgs {
                name: Some(builder.create_string("field")),
                value: 4,
                ..Default::default()
            };
            let val1 = reflection::EnumVal::create(&mut builder, &val1_args);
            let val2 = reflection::EnumVal::create(&mut builder, &val2_args);
            let vals = builder.create_vector(&[val1, val2]);
            let enum_args = reflection::EnumArgs {
                name: Some(builder.create_string("foo.bar")),
                underlying_type: Some(reflection::Type::create(
                    &mut builder,
                    &reflection::TypeArgs {
                        base_type: reflection::BaseType::UByte,
                        ..Default::default()
                    },
                )),
                values: Some(vals),
                ..Default::default()
            };
            let e = reflection::Enum::create(&mut builder, &enum_args);
            builder.finish_minimal(e);
        }

        let e: Enum = flatbuffers::get_root::<reflection::Enum>(builder.finished_data()).into();
        assert!(e.values[0].value < e.values[1].value);
    }
}
