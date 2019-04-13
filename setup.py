import os
import io
from setuptools import setup, find_packages

HERE = os.path.dirname(os.path.realpath(__file__))

README = os.path.join(HERE, 'README.md')
with io.open(README, encoding='utf-8') as f:
    long_description = f.read()

VERSION = os.path.join(HERE, 'cfb', 'version.py')
with io.open(VERSION, encoding='utf-8') as f:
    package = {}
    exec(f.read(), package)
    version = package['VERSION']

setup(name='cfbc',
      version=version,
      description='CFB code generator',
      long_description=long_description,
      long_description_content_type='text/markdown',
      url='http://github.com/nervosnetwork/cfb',
      author='Nervos Core Dev',
      author_email='dev@nervos.org',
      license='MIT',
      packages=find_packages(),
      install_requires=['docopt', 'Jinja2>=2.10.1', 'flatbuffers'],
      scripts=['bin/cfbc'],
      zip_safe=False,
      classifiers=[
          'Development Status :: 2 - Pre-Alpha',
          'Environment :: Console',
      ],
      include_package_data=True,
      )
