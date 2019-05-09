# CFB Encoding

## [![Build Status](https://travis-ci.com/nervosnetwork/cfb.svg?branch=master)](https://travis-ci.com/nervosnetwork/cfb)

CFB (Canonical FlatBuffers) is a restricted variant of FlatBuffers for producing unequivocal transfer syntax.

## Usage

Install the code geneartor as commane line utility:

```
cargo install cfbc
```

First generate bfbs file from fbs:

```
flatc -b --schema example.fbs
```

Then generate rust files from bfbs:

```
cfbc example.bfbs
```
