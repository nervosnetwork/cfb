from setuptools import setup

setup(name='cfbc',
      version='0.1',
      description='CFB code generator',
      url='http://github.com/nervosnetwork/cfb',
      author='Nervos Core Dev',
      author_email='dev@nervos.org',
      license='MIT',
      packages=['cfb'],
      install_requires=['docopt', 'Jinja2'],
      scripts=['bin/cfbc'],
      zip_safe=False)
