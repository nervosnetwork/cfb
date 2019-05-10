use crate::reflection_generated::reflection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SIZE_OF_UOFFSET: usize = 4;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum Type {
    UType(usize),
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
    Enum(usize),

    String,
    Vector(Box<Type>),
    Obj(usize),
    Union(usize),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schema {
    pub namespaces: Vec<String>,
    pub objects: Vec<Object>,
    pub enums: Vec<Enum>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Object {
    pub name: String,
    pub fields: Vec<Field>,
    pub fields_by_alignment: Vec<Field>,
    pub is_struct: bool,
    pub minalign: i32,
    pub bytesize: i32,
    pub attributes: HashMap<String, String>,

    pub alignment: usize,
    pub has_reference: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
    pub id: u16,
    pub offset: u16,
    pub default_integer: i64,
    pub default_real: f64,
    pub deprecated: bool,
    pub required: bool,
    pub key: bool,
    pub attributes: HashMap<String, String>,

    pub size: usize,
    pub alignment: usize,
    pub is_reference: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumVal>,
    pub is_union: bool,
    pub underlying_type: Type,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnumVal {
    pub name: String,
    pub value: i64,
    pub union_type: Option<usize>,
}

impl Type {
    pub fn is_enum(&self) -> bool {
        match self {
            Type::Enum(_) => true,
            _ => false,
        }
    }
}

impl Schema {
    pub fn from_bytes(bytes: &[u8]) -> Schema {
        reflection::get_root_as_schema(bytes).into()
    }

    pub fn type_size(&self, t: &Type) -> usize {
        match *t {
            Type::UType(_) => 1,
            Type::Bool => 1,
            Type::Byte => 1,
            Type::Short => 2,
            Type::Int => 4,
            Type::Long => 8,
            Type::UByte => 1,
            Type::UShort => 2,
            Type::UInt => 4,
            Type::ULong => 8,
            Type::Float => 4,
            Type::Double => 8,
            Type::Enum(index) => self.type_size(&self.enums[index].underlying_type),
            Type::Obj(index) => {
                let obj = &self.objects[index];
                if obj.is_struct {
                    obj.bytesize as usize
                } else {
                    SIZE_OF_UOFFSET
                }
            }
            _ => SIZE_OF_UOFFSET,
        }
    }

    pub fn type_alignment(&self, t: &Type) -> usize {
        match t {
            Type::Obj(index) => {
                let obj = &self.objects[*index];
                if obj.is_struct {
                    obj.minalign as usize
                } else {
                    SIZE_OF_UOFFSET
                }
            }
            _ => self.type_size(t),
        }
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

        let mut objects: Vec<Object> = vector_into_iter(schema.objects()).map(Into::into).collect();
        let enums = vector_into_iter(schema.enums()).map(Into::into).collect();

        let mut schema = Schema {
            objects: objects.clone(),
            namespaces,
            enums,
        };

        for o in &mut objects {
            for f in &mut o.fields {
                f.size = schema.type_size(&f.r#type);
                f.alignment = schema.type_alignment(&f.r#type);
                f.is_reference = match f.r#type {
                    Type::String | Type::Vector(_) | Type::Union(_) => true,
                    Type::Obj(index) => !schema.objects[index].is_struct,
                    _ => false,
                }
            }
            o.fields_by_alignment = o.fields.clone();
            o.fields_by_alignment
                .sort_by(|a, b| (b.alignment, b.size).cmp(&(a.alignment, a.size)));
            o.alignment = o
                .fields
                .iter()
                .map(|f| f.alignment)
                .max()
                .unwrap_or(SIZE_OF_UOFFSET);
            o.has_reference = o.fields.iter().any(|f| f.is_reference);
        }
        schema.objects = objects;

        schema
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

            // wait for Schema to fill them
            fields_by_alignment: Vec::new(),
            alignment: 0,
            has_reference: false,
        }
    }
}

impl<'a> From<reflection::Enum<'a>> for Enum {
    fn from(e: reflection::Enum<'a>) -> Enum {
        let mut values: Vec<EnumVal> = vector_into_iter(e.values()).map(Into::into).collect();
        values.sort_by_key(|f| f.value);

        Enum {
            values,
            underlying_type: e.underlying_type().into(),
            name: base_name(e.name()).to_string(),
            is_union: e.is_union(),
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
            r#type: try_into_enum(f.type_().into(), f.type_().index()),
            id: f.id(),
            offset: f.offset(),
            default_integer: f.default_integer(),
            default_real: f.default_real(),
            deprecated: f.deprecated(),
            required: f.required(),
            key: f.key(),
            attributes: collect_attributes(f.attributes()),

            // Wait for Schema to fill them
            alignment: 0,
            size: 0,
            is_reference: false,
        }
    }
}

impl<'a> From<reflection::Type<'a>> for Type {
    fn from(t: reflection::Type<'a>) -> Type {
        let index = t.index();

        match t.base_type() {
            reflection::BaseType::Vector => Type::Vector(Box::new(match t.element() {
                reflection::BaseType::Obj => Type::Obj(index as usize),
                element => from_base_type(element),
            })),
            reflection::BaseType::Obj => Type::Obj(t.index() as usize),
            reflection::BaseType::Union => Type::Union(t.index() as usize),
            reflection::BaseType::UType => Type::UType(t.index() as usize),
            base_type => from_base_type(base_type),
        }
    }
}

fn from_base_type(base_type: reflection::BaseType) -> Type {
    match base_type {
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

fn try_into_enum(ty: Type, index: i32) -> Type {
    if index == -1 {
        return ty;
    }

    match ty {
        Type::Byte
        | Type::UByte
        | Type::Short
        | Type::UShort
        | Type::Int
        | Type::UInt
        | Type::Long
        | Type::ULong => Type::Enum(index as usize),
        _ => ty,
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
