import os
import io
from setuptools import setup

HERE = os.path.dirname(os.path.realpath(__file__))
README = os.path.join(HERE, "README.md")
with io.open(README, encoding='utf-8') as f:
    long_description = f.read()

setup(name='cfbc',
      version='0.1',
      description='CFB code generator',
      long_description=long_description,
      long_description_content_type='text/markdown',
      url='http://github.com/nervosnetwork/cfb',
      author='Nervos Core Dev',
      author_email='dev@nervos.org',
      license='MIT',
      packages=['cfb'],
      install_requires=['docopt', 'Jinja2'],
      scripts=['bin/cfbc'],
      zip_safe=False)
