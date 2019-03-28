# CFB Encoding

## [![Build Status](https://travis-ci.com/nervosnetwork/cfb.svg?branch=master)](https://travis-ci.com/nervosnetwork/cfb)

CFB (Canonical FlatBuffers) is a restricted variant of FlatBuffers for producing unequivocal transfer syntax.

## Usage

Install the code geneartor as commane line utility:

```
pip install -U cfbc
```

First generate bfbs file from fbs:

```
flatc -b --schema example.fbs
```

Then generate rust files from bfbs:

```
cfbc example.bfbs
```

### Use Verifier with flatbuffers rust

Add the generated `<name>_generated_verifier.rs` to the project. This file
depends on the crate `flatbuffers`.

The file defines a function `get_root`, which is intended to replace the
function with the same name in `flatbuffers`. This function will verify the
buffer, and returns a `Result` to indicate whether the buffer is valid.
