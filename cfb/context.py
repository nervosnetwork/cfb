from cfb.namespace import Namespace
from cfb.reflection.BaseType import BaseType

SCALARS_SIZE = dict([
    (BaseType.Bool, 1),
    (BaseType.Byte, 1),
    (BaseType.Short, 2),
    (BaseType.Int, 4),
    (BaseType.Long, 8),
    (BaseType.UByte, 1),
    (BaseType.UShort, 2),
    (BaseType.UInt, 4),
    (BaseType.ULong, 8),
    (BaseType.Float, 4),
    (BaseType.Double, 8),
])

SCALARS_TYPE = dict([
    (BaseType.Bool, 'bool'),
    (BaseType.Byte, 'i8'),
    (BaseType.Short, 'i16'),
    (BaseType.Int, 'i32'),
    (BaseType.Long, 'i64'),
    (BaseType.UByte, 'u8'),
    (BaseType.UShort, 'u16'),
    (BaseType.UInt, 'u32'),
    (BaseType.ULong, 'u64'),
    (BaseType.Float, 'f32'),
    (BaseType.Double, 'f64'),
])

SCALARS_DEFAULT = dict([
    (BaseType.Bool, 'false'),
    (BaseType.Byte, '9i8'),
    (BaseType.Short, '0i16'),
    (BaseType.Int, '0i32'),
    (BaseType.Long, '0i64'),
    (BaseType.UByte, '0u8'),
    (BaseType.UShort, '0u16'),
    (BaseType.UInt, '0u32'),
    (BaseType.ULong, '0u64'),
    (BaseType.Float, '0f32'),
    (BaseType.Double, '0f64'),
])


class Context(object):
    def __init__(self, schema):
        self.schema = schema
        self.root = Namespace.from_schema(schema)

    def field_default(self, field):
        index = field.Type().Index()
        if index == -1:
            return SCALARS_DEFAULT[field.Type().BaseType()]
        else:
            return self.enum_default(self.schema.Enums(index))

    def enum_default(self, enum):
        for i in range(enum.ValuesLength()):
            val = enum.Values(i)
            if val.Value() == 0:
                return '{0}::{1}'.format(self.base_name(enum), val.Name().decode('utf-8'))

    def field_present(self, field):
        if field.Type().BaseType() == BaseType.Bool:
            return 'self.{0}'.format(self.field_name(field))
        else:
            return 'self.{0} != {1}'.format(self.field_name(field), self.field_default(field))

    def field_type(self, field):
        index = field.Type().Index()
        if index == -1:
            return SCALARS_TYPE[field.Type().BaseType()]
        else:
            return self.base_name(self.schema.Enums(index))

    def rust_type(self, cfb_type):
        return SCALARS_TYPE[cfb_type]

    def field_size(self, field):
        return SCALARS_SIZE[field.Type().BaseType()]

    def field_alignment(self, field):
        return SCALARS_SIZE[field.Type().BaseType()]

    def table_alignment(self, table):
        return max(self.field_alignment(table.Fields(i)) for i in range(table.FieldsLength()))

    def full_name(self, object):
        return object.Name().decode('utf-8').replace('.', '::')

    def base_name(self, object):
        return object.Name().decode('utf-8').split('.')[-1]

    def field_name(self, field):
        return field.Name().decode('utf-8')

    def fields_sorted_by_alignement(self, object):
        return list(sorted((object.Fields(i) for i in range(object.FieldsLength())),
                           key=lambda f: (self.field_alignment(f),
                                          self.field_size(f)),
                           reverse=True))

    def fields_sorted_by_offset(self, object):
        return list(sorted((object.Fields(i) for i in range(object.FieldsLength())),
                           key=lambda f: f.Offset(),
                           ))
