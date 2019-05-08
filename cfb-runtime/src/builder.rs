use crate::alignment::{align, align_after};

#[derive(Default, Debug)]
pub struct Builder {
    buffer: Vec<u8>,
}

pub trait WriteInto {
    fn write_into(self, builder: &mut Builder);
}

impl Builder {
    pub fn new() -> Builder {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Builder {
        Builder {
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn build<T: WriteInto>(mut self, root: T) -> Vec<u8> {
        root.write_into(&mut self);
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn push(&mut self, value: u8) {
        self.buffer.push(value)
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes)
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        let dest = &mut self.buffer[offset..offset + bytes.len()];
        dest.copy_from_slice(bytes);
    }

    /// Pad n bytes.
    pub fn pad(&mut self, n: usize) {
        self.buffer.resize(self.len() + n, 0)
    }

    /// Append paddings to ensure the next appended data is aligned.
    pub fn align(&mut self, alignment: usize) {
        self.buffer.resize(align(self.len(), alignment), 0)
    }

    /// Append paddings to ensure that, after data of lenth `len` has been appended, the next appended data is aligned.
    pub fn align_after(&mut self, len: usize, alignment: usize) {
        self.buffer
            .resize(align_after(self.len(), len, alignment), 0)
    }
}

impl WriteInto for bool {
    fn write_into(self, builder: &mut Builder) {
        if self {
            builder.push(1);
        } else {
            builder.push(0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_bool_into_buffer() {
        {
            let builder = Builder::new();
            assert_eq!(vec![1], builder.build(true));
        }

        {
            let builder = Builder::new();
            assert_eq!(vec![0], builder.build(false));
        }
    }
}
