from os import path
from unittest import TestCase
from mock import MagicMock
from cfb.namespace import Namespace
from cfb.reflection.Schema import Schema


class TestNamespace(TestCase):
    def test_root_object(self):
        n = Namespace()

        o = MagicMock()
        o.Name.return_value = b'foo'
        n.append_object(o)

        self.assertEqual(1, len(n.objects))
        self.assertIn('foo', n.objects)
        self.assertIs(o, n.objects['foo'])

        self.assertEqual(0, len(n.enums))

    def test_root_enum(self):
        n = Namespace()

        e = MagicMock()
        e.Name.return_value = b'foo'
        n.append_enum(e)

        self.assertEqual(0, len(n.objects))

        self.assertEqual(1, len(n.enums))
        self.assertIn('foo', n.enums)
        self.assertIs(e, n.enums['foo'])

    def test_nested_namespaces(self):
        n = Namespace()

        o = MagicMock()
        o.Name.return_value = b'foo.bar.Unit'
        n.append_object(o)

        e = MagicMock()
        e.Name.return_value = b'foo.Weapon'
        n.append_enum(e)

        self.assertEqual(0, len(n.objects))
        self.assertEqual(0, len(n.enums))
        self.assertEqual(1, len(n.children))
        self.assertIn('foo', n.children)

        foo = n.children['foo']
        self.assertEqual(0, len(foo.objects))
        self.assertEqual(1, len(foo.enums))
        self.assertIn('Weapon', foo.enums)
        self.assertIs(e, foo.enums['Weapon'])
        self.assertEqual(1, len(foo.children))
        self.assertIn('bar', foo.children)

        bar = foo.children['bar']
        self.assertEqual(1, len(bar.objects))
        self.assertIn('Unit', bar.objects)
        self.assertIs(o, bar.objects['Unit'])
        self.assertEqual(0, len(bar.enums))
        self.assertEqual(0, len(bar.children))

    def test_namespace_from_schema(self):
        dir_path = path.join(path.dirname(path.dirname(path.realpath(
            __file__))), 'tests', 'common', 'scalars_with_same_size.bfbs')
        with open(dir_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            schema = Schema.GetRootAsSchema(buf, 0)

        self.assertEqual(1, schema.ObjectsLength())
        n = Namespace.from_schema(schema)

        self.assertEqual(0, len(n.objects))
        self.assertEqual(0, len(n.enums))
        self.assertEqual(1, len(n.children))
        self.assertIn('example', n.children)

        example = n.children['example']
        self.assertEqual(1, len(example.objects))
        self.assertIn('Point', example.objects)
        self.assertEqual(0, len(example.enums))
        self.assertEqual(0, len(example.children))

        self.assertEqual(b'example.Point', example.objects['Point'].Name())
