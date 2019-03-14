from os import path
from unittest import TestCase
from cfb.reflection.Schema import Schema


class TestSchema(TestCase):
    def testGetRootAsSchema(self):
        dir_path = path.join(path.dirname(path.dirname(path.dirname(path.realpath(
            __file__)))), 'tests', 'common', 'scalars_with_same_size.bfbs')
        with open(dir_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            schema = Schema.GetRootAsSchema(buf, 0)

        self.assertEqual(1, schema.ObjectsLength())
        self.assertEqual(b'example.Point', schema.Objects(0).Name())
