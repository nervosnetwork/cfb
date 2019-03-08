from cfb.namespace import Namespace
from cfb.reflection.BaseType import BaseType
from cfb.constants import SIZE_OF_UOFFSET, BASE_TYPE_SIZE, BASE_TYPE_RUST_TYPE, BASE_TYPE_DEFAULT
from cfb.struct import struct_padded_fields


class Context(object):
    def __init__(self, schema):
        self.schema = schema
        self.root = Namespace.from_schema(schema)

    def field_default(self, field):
        index = field.Type().Index()
        if index == -1:
            return BASE_TYPE_DEFAULT[field.Type().BaseType()]
        return self.enum_default(self.schema.Enums(index))

    def enum_default(self, enum):
        for i in range(enum.ValuesLength()):
            val = enum.Values(i)
            if val.Value() == 0:
                return '{0}::{1}'.format(self.base_name(enum), val.Name().decode('utf-8'))

    def field_present(self, field, extract=None):
        base_type = field.Type().BaseType()
        if base_type == BaseType.Obj:
            obj = self.schema.Objects(field.Type().Index())
            if obj.IsStruct():
                return 'self.{0}.is_present()'.format(self.field_name(field))

            if extract is None:
                return 'self.{0}.is_some()'.format(self.field_name(field))
            return 'let Some({0}) = self.{1}'.format(extract, self.field_name(field))

        if base_type == BaseType.Bool:
            return 'self.{0}'.format(self.field_name(field))
        if base_type == BaseType.String or base_type == BaseType.Vector:
            return '!self.{0}.is_empty()'.format(self.field_name(field))

        if base_type == BaseType.Union:
            if extract is None:
                return 'self.{0}.is_some()'.format(self.field_name(field))
            return 'let Some({0}) = self.{1}'.format(extract, self.field_name(field))
        if base_type == BaseType.UType:
            if extract is None:
                return 'self.{0}.is_some()'.format(self.field_name(field)[:-5])
            return 'let Some({0}) = {1}'.format(extract, self.field_name(field))

        return 'self.{0} != {1}'.format(self.field_name(field), self.field_default(field))

    def field_type(self, field):
        index = field.Type().Index()
        base_type = field.Type().BaseType()

        if base_type == BaseType.Vector:
            if index == -1:
                return "Vec<{0}>".format(self.rust_type(field.Type().Element()))

            if field.Type().Element() == BaseType.Obj:
                obj = self.schema.Objects(index)
                return "Vec<{0}>".format(self.base_name(obj))
            enum = self.schema.Enums(index)
            return "Vec<{0}>".format(self.base_name(enum))

        if base_type == BaseType.Obj:
            obj = self.schema.Objects(index)
            if obj.IsStruct():
                return self.base_name(obj)

            return 'Option<{0}>'.format(self.base_name(obj))

        if index == -1:
            return self.rust_type(base_type)

        enum = self.schema.Enums(index)
        if base_type == BaseType.UType:
            return '{0}Type'.format(self.base_name(enum))
        if enum.IsUnion():
            return 'Option<{0}>'.format(self.base_name(enum))
        return self.base_name(enum)

    def rust_type(self, cfb_type):
        return BASE_TYPE_RUST_TYPE[cfb_type]

    def field_size(self, field):
        base_type = field.Type().BaseType()
        if base_type == BaseType.Obj:
            obj = self.schema.Objects(field.Type().Index())
            if obj.IsStruct():
                return obj.Bytesize()

            return SIZE_OF_UOFFSET

        return BASE_TYPE_SIZE[field.Type().BaseType()]

    def field_alignment(self, field):
        base_type = field.Type().BaseType()
        if base_type == BaseType.Obj:
            obj = self.schema.Objects(field.Type().Index())
            if obj.IsStruct():
                return obj.Minalign()

            return SIZE_OF_UOFFSET

        return BASE_TYPE_SIZE[base_type]

    def table_alignment(self, table):
        return max(self.field_alignment(table.Fields(i)) for i in range(table.FieldsLength()))

    def element_aligment(self, field):
        element = field.Type().Element()
        if element == BaseType.Obj:
            obj = self.schema.Objects(field.Type().Index())
            if obj.IsStruct():
                return obj.Minalign()

            return SIZE_OF_UOFFSET

        return BASE_TYPE_SIZE[element]

    def full_name(self, object):
        return object.Name().decode('utf-8').replace('.', '::')

    def base_name(self, object):
        return object.Name().decode('utf-8').split('.')[-1]

    def field_name(self, field):
        return field.Name().decode('utf-8')

    def field_union_enum(self, field):
        return self.schema.Enums(field.Type().Index())

    def is_table(self, field):
        return field.Type().BaseType() == BaseType.Obj and not self.schema.Objects(field.Type().Index()).IsStruct()

    def is_string(self, field):
        return field.Type().BaseType() == BaseType.String

    def is_union(self, field):
        return field.Type().BaseType() == BaseType.Union

    def is_union_type(self, field):
        return field.Type().BaseType() == BaseType.UType

    def is_vector(self, field):
        return field.Type().BaseType() == BaseType.Vector

    def is_element_scalar(self, field):
        element = field.Type().Element()
        if element == BaseType.Vector or element == BaseType.String or element == BaseType.Union:
            return False
        if element == BaseType.Obj:
            return self.schema.Objects(field.Type().Index()).IsStruct()

        return True

    def is_element_string(self, field):
        return field.Type().Element() == BaseType.String

    def is_element_table(self, field):
        if field.Type().Element() != BaseType.Obj:
            return False

        obj = self.schema.Objects(field.Type().Index())
        return not obj.IsStruct()

    def struct_padded_fields(self, struct):
        return struct_padded_fields(self, struct)

    def enum_values(self, enum):
        return list(sorted((enum.Values(i) for i in range(enum.ValuesLength())),
                           key=lambda v: v.Value()))

    def fields_sorted_by_alignement(self, object):
        return list(sorted((object.Fields(i) for i in range(object.FieldsLength())),
                           key=lambda f: (self.field_alignment(f),
                                          self.field_size(f)),
                           reverse=True))

    def fields_sorted_by_offset(self, object):
        return list(sorted((object.Fields(i) for i in range(object.FieldsLength())),
                           key=lambda f: f.Offset(),
                           ))
