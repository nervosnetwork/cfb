"""Generate code from serialized flatbuffers schema in bfbs format.

Usage:
  cfbc [-o <dir>] <bfbs>

Options:
  -o <dir>  output directory (default: the same directory of <bfbs>)
  <bfbs>    bfbs file which is generated using `flatc -b --schema <fbs>`.
"""
from docopt import docopt


def parse_arguments(argv=None):
    return docopt(__doc__, argv)


def main():
    print(parse_arguments())
