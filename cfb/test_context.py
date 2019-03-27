from os import path
from unittest import TestCase
from cfb.reflection.Schema import Schema
from cfb.context import Context


class TestContext(TestCase):
    def schema(self):
        dir_path = path.join(path.dirname(path.dirname(path.realpath(
            __file__))), 'tests', 'common', 'ckb.bfbs')
        with open(dir_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            return Schema.GetRootAsSchema(buf, 0)

    def setUp(self):
        self.context = Context('ckb', self.schema())

    def testCamelToSnake(self):
        self.assertEqual('test_camel_to_snake',
                         self.context.camel_to_snake('testCamelToSnake'))
        self.assertEqual('html', self.context.camel_to_snake('HTML'))
        self.assertEqual('html_page', self.context.camel_to_snake('HTMLPage'))
        self.assertEqual('out_html', self.context.camel_to_snake('OutHTML'))
        self.assertEqual(
            'out_html_page', self.context.camel_to_snake('OutHTMLPage'))
        self.assertEqual('p2p', self.context.camel_to_snake('P2P'))
