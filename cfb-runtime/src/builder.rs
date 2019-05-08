use crate::alignment::{align, align_after};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

#[derive(Default, Debug)]
pub struct Builder {
    buffer: Vec<u8>,
    vtables: HashMap<u64, usize>,
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
            vtables: HashMap::default(),
        }
    }

    pub fn build<T: WriteInto>(mut self, root: T) -> Vec<u8> {
        root.write_into(&mut self);
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_slice()
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

    /// Vtable is at the top of the buffer starting from `vtable_start`. Return the position of the vtable
    /// in the buffer after deduplication.
    pub fn deduplicate_vtable(&mut self, vtable_start: usize) -> usize {
        let vtable_end = self.len();
        let vtable_slice = &self.buffer[vtable_start..vtable_end];
        let key = {
            let mut hasher = self.vtables.hasher().build_hasher();
            hasher.write(vtable_slice);
            hasher.finish()
        };

        if let Some(&offset) = self
            .vtables
            .get(&key)
            .filter(|offset| &self.buffer[**offset..**offset + vtable_slice.len()] == vtable_slice)
        {
            // Table alignment must be larger than vtable, so it is OK to leave the paddings in the
            // buffer.
            self.buffer.truncate(vtable_start);
            offset
        } else {
            self.vtables.insert(key, vtable_start);
            vtable_start
        }
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

impl WriteInto for u8 {
    fn write_into(self, builder: &mut Builder) {
        builder.push(self);
    }
}

impl WriteInto for i8 {
    fn write_into(self, builder: &mut Builder) {
        builder.push(self as u8);
    }
}

macro_rules! impl_write_into_via_to_le_bytes {
    ($ty:ident) => {
        impl WriteInto for $ty {
            fn write_into(self, builder: &mut Builder) {
                builder.push_bytes(&self.to_le_bytes())
            }
        }
    };
}

impl_write_into_via_to_le_bytes!(i16);
impl_write_into_via_to_le_bytes!(u16);
impl_write_into_via_to_le_bytes!(i32);
impl_write_into_via_to_le_bytes!(u32);
impl_write_into_via_to_le_bytes!(i64);
impl_write_into_via_to_le_bytes!(u64);

impl WriteInto for f32 {
    fn write_into(self, builder: &mut Builder) {
        builder.push_bytes(&self.to_bits().to_le_bytes())
    }
}

impl WriteInto for f64 {
    fn write_into(self, builder: &mut Builder) {
        builder.push_bytes(&self.to_bits().to_le_bytes())
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

    #[test]
    fn test_deduplicate_vtable() {
        let mut builder = Builder::new();
        builder.push(0);
        builder.push(0);

        builder.push(1);
        assert_eq!(2, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1], builder.as_bytes());

        builder.push(2);
        assert_eq!(3, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1, 2], builder.as_bytes());

        builder.push(1);
        assert_eq!(2, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1, 2], builder.as_bytes());
    }
}
