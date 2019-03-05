from os import path
from cfb.reflection.Schema import Schema
from jinja2 import Environment, PackageLoader, select_autoescape


class Generator(object):
    def __init__(self, bfbs_path):
        self.outdir = path.dirname(bfbs_path)
        self.basename, _ = path.splitext(path.basename(bfbs_path))

        with open(bfbs_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            self.schema = Schema.GetRootAsSchema(buf, 0)

    def generate(self, outdir=None):
        outdir = outdir or self.outdir
        env = Environment(
            loader=PackageLoader('cfb', 'templates')
        )

        builder = env.get_template('builder.rs.jinja')
        with open(path.join(outdir, self.basename + '_builder.rs'), 'w') as out_file:
            out_file.write(builder.render(schema=self.schema))
