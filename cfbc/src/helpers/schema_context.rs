use cfb_schema::Enum;
use handlebars::Context;
use serde_json::Value;

pub struct SchemaContext<'a> {
    objects: &'a [Value],
    enums: &'a [Value],
}

impl<'a> SchemaContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        let schema = context.data().get("schema").expect("schema in context");
        let objects = schema
            .get("objects")
            .and_then(Value::as_array)
            .expect("objects in schema")
            .as_slice();
        let enums = schema
            .get("enums")
            .and_then(Value::as_array)
            .expect("enums in schema")
            .as_slice();

        SchemaContext { objects, enums }
    }

    pub fn object_name(&self, index: usize) -> &str {
        self.objects[index]
            .get("name")
            .and_then(Value::as_str)
            .expect("name in Object")
    }

    pub fn enum_at(&self, index: usize) -> Enum {
        serde_json::from_value(self.enums[index].clone()).expect("deserialize enum")
    }

    pub fn enum_name(&self, index: usize) -> &str {
        self.enums[index]
            .get("name")
            .and_then(Value::as_str)
            .expect("name in Enum")
    }
}
