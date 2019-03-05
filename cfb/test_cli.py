from unittest import TestCase
from cfb import cli
from docopt import DocoptExit


class TestCli(TestCase):
    def test_parse_arguments(self):
        with self.assertRaises(DocoptExit):
            cli.parse_arguments([])

        args = cli.parse_arguments(['test.bfbs'])
        self.assertEqual('test.bfbs', args['<bfbs>'])
        self.assertEqual(None, args['-o'])

        args = cli.parse_arguments(['-o', 'out', 'test.bfbs'])
        self.assertEqual('out', args['-o'])
        self.assertEqual('test.bfbs', args['<bfbs>'])
