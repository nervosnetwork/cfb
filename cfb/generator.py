from os import path
from jinja2 import Environment, PackageLoader, select_autoescape
from cfb.reflection.Schema import Schema
from cfb.context import Context


class Generator(object):
    def __init__(self, bfbs_path):
        self.outdir = path.dirname(bfbs_path)
        self.basename, _ = path.splitext(path.basename(bfbs_path))

        with open(bfbs_path, 'rb') as bfbs_file:
            buf = bytearray(bfbs_file.read())
            schema = Schema.GetRootAsSchema(buf, 0)
            self.context = Context(schema)

    def generate(self, outdir=None):
        outdir = outdir or self.outdir
        env = Environment(
            loader=PackageLoader('cfb', 'templates')
        )

        builder = env.get_template('builder.rs.jinja')
        builder_content = builder.render(cfb=self.context)
        with open(path.join(outdir, self.basename + '_builder.rs'), 'w') as out_file:
            out_file.write(builder_content)
