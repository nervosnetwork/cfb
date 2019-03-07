from unittest import TestCase
from os import path
from cfb.struct import struct_padded_fields
from cfb.context import Context
from cfb.reflection.Schema import Schema


class TestStruct(TestCase):
    def test_struct_padded_fields(self):
        dir_path = path.join(path.dirname(path.dirname(path.realpath(
            __file__))), 'tests', 'common', 'struct.bfbs')
        with open(dir_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            schema = Schema.GetRootAsSchema(buf, 0)

        fields = struct_padded_fields(Context(schema), schema.Objects(1))

        self.assertEqual(3, len(fields))
        self.assertEqual(b'x', fields[0].field.Name())
        self.assertEqual(0, len(fields[0].paddings))
        self.assertEqual(b'y', fields[1].field.Name())
        self.assertEqual(0, len(fields[0].paddings))
        self.assertEqual(b'z', fields[2].field.Name())
        self.assertEqual(3, len(fields[2].paddings))

        self.assertEqual(0, fields[2].paddings[0].index)
        self.assertEqual(1, fields[2].paddings[1].index)
        self.assertEqual(2, fields[2].paddings[2].index)
        self.assertEqual('u8', fields[2].paddings[0].ty)
        self.assertEqual('u16', fields[2].paddings[1].ty)
        self.assertEqual('u32', fields[2].paddings[2].ty)
