# CFB Encoding

## [![Build Status](https://travis-ci.com/nervosnetwork/cfb.svg?branch=master)](https://travis-ci.com/nervosnetwork/cfb)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fnervosnetwork%2Fcfb.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fnervosnetwork%2Fcfb?ref=badge_shield)

CFB (Canonical FlatBuffers) is a restricted variant of FlatBuffers for producing unequivocal transfer syntax.

## Usage

Install the code geneartor as commane line utility:

```
pip install cfbc
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


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fnervosnetwork%2Fcfb.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fnervosnetwork%2Fcfb?ref=badge_large)