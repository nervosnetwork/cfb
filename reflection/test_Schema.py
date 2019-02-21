import os
from os import path
from unittest import TestCase
from reflection.Schema import Schema


class TestSchema(TestCase):
    def testGetRootAsSchema(self):
        dir_path = path.join(path.dirname(path.dirname(os.path.realpath(__file__))), 'tests', 'common', 'example.bfbs')
        with open(dir_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            schema = Schema.GetRootAsSchema(buf, 0)

        self.assertEqual(7, schema.ObjectsLength())
        self.assertEqual(b'example.Author', schema.Objects(0).Name())
