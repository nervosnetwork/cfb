use crate::alignment::{align, align_after};
use crate::types::{SOffset, UOffset, VOffset, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET, SIZE_OF_VOFFSET};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

#[derive(Default, Debug)]
pub struct Builder {
    buffer: Vec<u8>,
    vtables: HashMap<u64, usize>,
}

pub trait PushScalarInto {
    fn push_into(self, builder: &mut Builder);
}

pub trait PushReferenceInto {
    fn push_into(self, builder: &mut Builder) -> usize;
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

    pub fn build<T: PushReferenceInto>(mut self, root: T) -> Vec<u8> {
        self.align(SIZE_OF_UOFFSET);
        let uoffset_pos = self.len();
        self.pad(SIZE_OF_UOFFSET);
        self.push_reference(uoffset_pos, root);
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

    pub fn push_scalar<T: PushScalarInto>(&mut self, scalar: T) {
        scalar.push_into(self);
    }

    pub fn push_reference<T: PushReferenceInto>(&mut self, uoffset_pos: usize, reference: T) {
        let offset = reference.push_into(self);
        let uoffset = (offset - uoffset_pos) as UOffset;
        self.write_uoffset(uoffset_pos, uoffset);
    }

    pub fn push_byte(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        let dest = &mut self.buffer[offset..offset + bytes.len()];
        dest.copy_from_slice(bytes);
    }

    pub fn write_uoffset(&mut self, offset: usize, uoffset: UOffset) {
        self.write_bytes(offset, &uoffset.to_le_bytes());
    }

    pub fn write_voffset(&mut self, offset: usize, voffset: VOffset) {
        self.write_bytes(offset, &voffset.to_le_bytes());
    }

    pub fn write_soffset(&mut self, offset: usize, soffset: SOffset) {
        self.write_bytes(offset, &soffset.to_le_bytes());
    }

    /// Pad n bytes.
    pub fn pad(&mut self, n: usize) {
        self.buffer.resize(self.len() + n, 0);
    }

    /// Append paddings to ensure the next appended data is aligned.
    pub fn align(&mut self, alignment: usize) {
        self.buffer.resize(align(self.len(), alignment), 0);
    }

    /// Append paddings to ensure that, after data of lenth `len` has been appended, the next appended data is aligned.
    pub fn align_after(&mut self, len: usize, alignment: usize) {
        self.buffer
            .resize(align_after(self.len(), len, alignment), 0);
    }

    pub fn start_vtable(&mut self) -> VTableBuilder {
        VTableBuilder::new(self)
    }

    /// Vtable is at the top of the buffer starting from `vtable_start`. Return the position of the vtable
    /// in the buffer after deduplication.
    fn deduplicate_vtable(&mut self, vtable_start: usize) -> usize {
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

pub struct VTableBuilder<'a> {
    builder: &'a mut Builder,
    vtable_start: usize,
    fields_nbytes: usize,
}

impl<'a> VTableBuilder<'a> {
    fn new(builder: &'a mut Builder) -> Self {
        builder.align(SIZE_OF_VOFFSET);
        let vtable_start = builder.len();
        builder.pad(2 * SIZE_OF_VOFFSET);

        VTableBuilder {
            builder,
            vtable_start,
            fields_nbytes: 0,
        }
    }

    pub fn add_field(&mut self, offset_in_vtable: usize, size: usize, alignment: usize) {
        let voffset_position = self.vtable_start + offset_in_vtable;
        let offset_in_fields = align(self.fields_nbytes, alignment);

        if voffset_position < self.builder.len() {
            self.builder.write_voffset(
                voffset_position,
                (offset_in_fields + SIZE_OF_SOFFSET) as VOffset,
            );
        } else {
            self.builder.pad(voffset_position - self.builder.len());
            self.builder
                .push_scalar((offset_in_fields + SIZE_OF_SOFFSET) as VOffset);
        }
        self.fields_nbytes = offset_in_fields + size;
    }

    pub fn finish(self) -> usize {
        self.builder.write_voffset(
            self.vtable_start,
            (self.builder.len() - self.vtable_start) as VOffset,
        );
        self.builder.write_voffset(
            self.vtable_start + SIZE_OF_VOFFSET,
            (self.fields_nbytes + SIZE_OF_SOFFSET) as VOffset,
        );

        self.builder.deduplicate_vtable(self.vtable_start)
    }
}

impl PushScalarInto for bool {
    fn push_into(self, builder: &mut Builder) {
        if self {
            builder.push_byte(1);
        } else {
            builder.push_byte(0);
        }
    }
}

impl PushScalarInto for u8 {
    fn push_into(self, builder: &mut Builder) {
        builder.push_byte(self);
    }
}

impl PushScalarInto for i8 {
    fn push_into(self, builder: &mut Builder) {
        builder.push_byte(self as u8);
    }
}

macro_rules! impl_push_into_via_to_le_bytes {
    ($ty:ident) => {
        impl PushScalarInto for $ty {
            fn push_into(self, builder: &mut Builder) {
                builder.push_bytes(&self.to_le_bytes())
            }
        }
    };
}

impl_push_into_via_to_le_bytes!(i16);
impl_push_into_via_to_le_bytes!(u16);
impl_push_into_via_to_le_bytes!(i32);
impl_push_into_via_to_le_bytes!(u32);
impl_push_into_via_to_le_bytes!(i64);
impl_push_into_via_to_le_bytes!(u64);

impl PushScalarInto for f32 {
    fn push_into(self, builder: &mut Builder) {
        builder.push_bytes(&self.to_bits().to_le_bytes())
    }
}

impl PushScalarInto for f64 {
    fn push_into(self, builder: &mut Builder) {
        builder.push_bytes(&self.to_bits().to_le_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_bool_into_buffer() {
        {
            let mut builder = Builder::new();
            builder.push_scalar(true);
            assert_eq!(&[1], builder.as_bytes());
        }

        {
            let mut builder = Builder::new();
            builder.push_scalar(false);
            assert_eq!(&[0], builder.as_bytes());
        }
    }

    #[test]
    fn test_deduplicate_vtable() {
        let mut builder = Builder::new();
        builder.push_byte(0);
        builder.push_byte(0);

        builder.push_byte(1);
        assert_eq!(2, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1], builder.as_bytes());

        builder.push_byte(2);
        assert_eq!(3, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1, 2], builder.as_bytes());

        builder.push_byte(1);
        assert_eq!(2, builder.deduplicate_vtable(builder.len() - 1));
        assert_eq!(&[0, 0, 1, 2], builder.as_bytes());
    }

    #[test]
    fn test_vtable_builder() {
        let mut builder = Builder::new();

        {
            let mut vt = builder.start_vtable();
            vt.add_field(4, 4, 1);
            vt.finish();

            let expect = [
                &6u16.to_le_bytes()[..],
                &8u16.to_le_bytes(),
                &4u16.to_le_bytes(),
            ]
            .concat();
            assert_eq!(expect, &builder.as_bytes()[..6]);
        }

        {
            let mut vt = builder.start_vtable();
            vt.add_field(8, 4, 1);
            vt.add_field(4, 2, 1);
            vt.finish();

            let expect = [
                &10u16.to_le_bytes()[..],
                &10u16.to_le_bytes(),
                &8u16.to_le_bytes(),
                &0u16.to_le_bytes(),
                &4u16.to_le_bytes(),
            ]
            .concat();
            assert_eq!(expect, &builder.as_bytes()[6..16]);
        }
    }

    #[test]
    fn test_vtable_builder_alignment() {
        let mut builder = Builder::new();

        {
            let mut vt = builder.start_vtable();
            vt.add_field(8, 6, 4);
            vt.add_field(4, 4, 4);
            vt.finish();

            let expect = [
                &10u16.to_le_bytes()[..],
                &16u16.to_le_bytes(),
                &12u16.to_le_bytes(),
                &0u16.to_le_bytes(),
                &4u16.to_le_bytes(),
            ]
            .concat();
            assert_eq!(expect, &builder.as_bytes()[..10]);
        }
    }
}
