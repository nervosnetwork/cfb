"""Generate code from serialized flatbuffers schema in bfbs format.

Usage:
  cfbc [-o <dir>] <bfbs>

Options:
  -o <dir>  output directory (default: the same directory of <bfbs>)
  <bfbs>    bfbs file which is generated using `flatc -b --schema <fbs>`.
"""
import os
from docopt import docopt
from cfb.generator import Generator


def parse_arguments(argv=None):
    return docopt(__doc__, argv)


def main():
    arguments = parse_arguments()
    g = Generator(arguments['<bfbs>'])
    g.generate(arguments['-o'])
