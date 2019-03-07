use crate::alignment::{align, align_after};
use crate::scalar::Scalar;
use crate::types::{
    Len, UOffset, VOffset, SIZE_OF_LEN, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET, SIZE_OF_VOFFSET,
};
use std::collections::HashMap;

pub trait Component<'c> {
    /// Build the component and return the start position of the component in the buffer.
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize;
}

impl<'c, F> Component<'c> for F
where
    F: FnOnce(&mut Builder<'c>) -> usize,
{
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        (self)(builder)
    }
}

pub struct DesignatedComponent<'c> {
    /// Where to store the UOffset in the buffer.
    offset_position: usize,
    component: Box<dyn Component<'c> + 'c>,
}

impl<'c> DesignatedComponent<'c> {
    pub fn new(
        offset_position: usize,
        component: Box<dyn Component<'c> + 'c>,
    ) -> DesignatedComponent<'c> {
        DesignatedComponent {
            offset_position,
            component,
        }
    }

    fn build(self, builder: &mut Builder<'c>) {
        let position = self.component.build(builder);
        let uoffset = (position - self.offset_position) as UOffset;

        builder.set_scalar(self.offset_position, uoffset);
    }
}

pub struct Builder<'c> {
    buffer: Vec<u8>,
    components: Vec<DesignatedComponent<'c>>,
    vtables: HashMap<Vec<u8>, usize>,
}

impl<'c> Builder<'c> {
    pub fn new<C: Component<'c> + 'c>(root: C) -> Builder<'c> {
        Builder {
            buffer: vec![0u8; SIZE_OF_UOFFSET],
            components: vec![DesignatedComponent::new(0, Box::new(root))],
            vtables: Default::default(),
        }
    }

    pub fn with_capacity<C: Component<'c> + 'c>(capacity: usize, root: C) -> Builder<'c> {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.extend_from_slice(&[0u8; SIZE_OF_UOFFSET]);

        Builder {
            buffer,
            components: vec![DesignatedComponent::new(0, Box::new(root))],
            vtables: Default::default(),
        }
    }

    pub fn build(mut self) -> Vec<u8> {
        while let Some(component) = self.components.pop() {
            component.build(&mut self);
        }

        self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    pub fn tell(&self) -> usize {
        self.buffer.len()
    }

    pub fn push_component(&mut self, component: DesignatedComponent<'c>) {
        assert!(component.offset_position + 4 <= self.tell());
        self.components.push(component);
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(bytes);
        self
    }

    pub fn push_scalar<T: Scalar>(&mut self, mut s: T) {
        s = s.to_le();
        self.buffer.extend_from_slice(s.to_le().as_bytes());
    }

    pub fn set_scalar<T: Scalar>(&mut self, position: usize, mut s: T) {
        s = s.to_le();
        let src = s.as_bytes();
        assert!(position + src.len() <= self.buffer.len());
        let target = &mut self.buffer[position..position + src.len()];
        target.copy_from_slice(src);
    }

    /// Pad n bytes.
    pub fn pad(&mut self, n: usize) {
        self.buffer.resize(self.tell() + n, 0)
    }

    /// Append paddings to ensure the next appended data is aligned.
    pub fn align(&mut self, alignment: usize) {
        self.buffer.resize(align(self.tell(), alignment), 0)
    }

    /// Append paddings to ensure that, after data of lenth `len` has been appended, the next appended data is aligned.
    pub fn align_after(&mut self, len: usize, alignment: usize) {
        self.buffer
            .resize(align_after(self.tell(), len, alignment), 0)
    }

    pub fn start_vtable<'b>(&'b mut self) -> VTableBuilder<'b, 'c> {
        VTableBuilder::new(self)
    }

    /// Vtable is at the top of the buffer with `nbytes` bytes. Return the position of the vtable
    /// in the buffer after deduplication.
    fn deduplicate_vtable(&mut self, vtable_start: usize) -> usize {
        let vtable_end = self.tell();
        let vtable_slice = &self.buffer[vtable_start..vtable_end];

        if let Some(&offset) = self.vtables.get(vtable_slice) {
            // Table alignment must be larger than vtable, so it is OK to leave the paddings in the
            // buffer.
            self.buffer.truncate(vtable_start);
            offset
        } else {
            self.vtables.insert(vtable_slice.to_owned(), vtable_start);
            vtable_start
        }
    }
}

pub struct VTableBuilder<'b, 'c> {
    builder: &'b mut Builder<'c>,
    vtable_start: usize,
    fields_nbytes: usize,
}

impl<'b, 'c> VTableBuilder<'b, 'c> {
    fn new(builder: &'b mut Builder<'c>) -> Self {
        builder.align(SIZE_OF_VOFFSET);
        let vtable_start = builder.tell();
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

        if voffset_position < self.builder.tell() {
            self.builder.set_scalar(
                voffset_position,
                (offset_in_fields + SIZE_OF_SOFFSET) as VOffset,
            );
        } else {
            self.builder.pad(voffset_position - self.builder.tell());
            self.builder
                .push_scalar((offset_in_fields + SIZE_OF_SOFFSET) as VOffset);
        }
        self.fields_nbytes = offset_in_fields + size;
    }

    pub fn finish(self) -> usize {
        self.builder.set_scalar(
            self.vtable_start,
            (self.builder.tell() - self.vtable_start) as VOffset,
        );
        self.builder.set_scalar(
            self.vtable_start + SIZE_OF_VOFFSET,
            (self.fields_nbytes + SIZE_OF_SOFFSET) as VOffset,
        );

        self.builder.deduplicate_vtable(self.vtable_start)
    }
}

#[derive(Debug)]
pub struct StringComponent<T>(T);

impl<T: AsRef<str>> StringComponent<T> {
    pub fn new(s: T) -> Self {
        StringComponent(s)
    }
}

impl<'c, T: AsRef<str>> Component<'c> for StringComponent<T> {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let s = self.0.as_ref();

        builder.align(SIZE_OF_LEN);
        let position = builder.tell();

        builder.push_scalar(s.len() as Len);
        builder.extend_from_slice(s.as_bytes());
        builder.push_scalar(0u8);

        position
    }
}

#[derive(Debug)]
pub struct ScalarsVectorComponent<T> {
    scalars: T,
    len: usize,
}

impl<T> ScalarsVectorComponent<T> {
    pub fn new(scalars: T, len: usize) -> Self {
        ScalarsVectorComponent { scalars, len }
    }
}

impl<'c, T: AsRef<[u8]>> Component<'c> for ScalarsVectorComponent<T> {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let bytes = self.scalars.as_ref();

        let alignment = bytes.len() / self.len;
        builder.align_after(SIZE_OF_LEN, alignment);
        let position = builder.tell();

        builder.push_scalar(self.len as Len);
        builder.extend_from_slice(bytes);

        position
    }
}

#[derive(Debug)]
pub struct ReferencesVectorComponent<T>(T);

impl<T> ReferencesVectorComponent<T> {
    pub fn new(references: T) -> Self {
        ReferencesVectorComponent(references)
    }
}

impl<'c, T, I, C> Component<'c> for ReferencesVectorComponent<T>
where
    T: IntoIterator<Item = C, IntoIter = I>,
    I: ExactSizeIterator<Item = C> + DoubleEndedIterator<Item = C>,
    C: Component<'c> + 'c,
{
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        builder.align_after(SIZE_OF_LEN, SIZE_OF_UOFFSET);
        let position = builder.tell();

        let iter = self.0.into_iter();
        let len = iter.len();

        let mut current_offset_position =
            position + SIZE_OF_LEN + len * SIZE_OF_UOFFSET - SIZE_OF_UOFFSET;
        builder.push_scalar(len as Len);
        builder.pad(len * SIZE_OF_UOFFSET);
        for c in iter.rev() {
            builder.push_component(DesignatedComponent::new(
                current_offset_position,
                Box::new(c),
            ));
            current_offset_position -= SIZE_OF_UOFFSET;
        }

        position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_component() {
        let builder = Builder::new(|builder: &mut Builder| builder.tell());
        let buf = builder.build();

        let expect = [
            // root uoffset
            4u32.to_le_bytes(),
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_owned_string_component() {
        let s = String::from("String");
        let builder = Builder::new(StringComponent::new(s.clone()));
        let buf = builder.build();

        let expect = [
            // root uoffset
            &4u32.to_le_bytes()[..],
            // len
            &((s.len() as u32).to_le_bytes()),
            // content
            s.as_bytes(),
            // null-terminated
            &[0u8],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_borrowed_string_component() {
        let s = "str".to_string();
        let builder = Builder::new(StringComponent::new(s.as_str()));
        let buf = builder.build();

        let expect = [
            // root uoffset
            &4u32.to_le_bytes(),
            // len
            &((s.len() as u32).to_le_bytes()),
            // content
            s.as_bytes(),
            // terminal null
            &[0u8],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_string_component_alignment() {
        let s = String::from("s");
        let mut builder = Builder::new(StringComponent::new(s.clone()));
        builder.push_scalar(0);
        let buf = builder.build();

        let expect = [
            &8u32.to_le_bytes(),
            &[0u8, 0, 0, 0], // padding
            &((s.len() as u32).to_le_bytes()),
            s.as_bytes(),
            &[0u8],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_scalars_vector_component() {
        let scalars: Vec<u8> = vec![1u32, 9]
            .into_iter()
            .map(|n| n.to_le_bytes().to_vec())
            .flatten()
            .collect();
        let builder = Builder::new(ScalarsVectorComponent::new(scalars.clone(), 2));
        let buf = builder.build();

        let expect = [
            // root uoffset
            &4u32.to_le_bytes(),
            // len
            &2u32.to_le_bytes(),
            // content
            &scalars[..],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_references_vector_component() {
        let builder = Builder::new(ReferencesVectorComponent::new(vec![
            StringComponent::new(String::from("s1")),
            StringComponent::new(String::from("s2")),
        ]));
        let buf = builder.build();

        let expect = [
            // root uoffset
            &4u32.to_le_bytes()[..],
            // len
            &2u32.to_le_bytes(),
            // offsets of s1
            &8u32.to_le_bytes(),
            // offsets of s2
            &12u32.to_le_bytes(),
            // s1.len
            &2u32.to_le_bytes(),
            b"s1",
            // padding
            &[0u8, 0],
            // s2.len
            &2u32.to_le_bytes(),
            b"s2",
            // padding
            &[0u8],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

    #[test]
    fn test_deduplicate_vtable() {
        let mut builder = Builder::new(|builder: &mut Builder| builder.tell());

        {
            let mut vt = builder.start_vtable();
            vt.add_field(4, 4, 1);
            assert_eq!(4, vt.finish());
            assert_eq!(10, builder.tell());
        }

        {
            let mut vt = builder.start_vtable();
            vt.add_field(4, 4, 1);
            assert_eq!(4, vt.finish());
            assert_eq!(10, builder.tell());
        }

        {
            let mut vt = builder.start_vtable();
            vt.add_field(4, 8, 1);
            assert_eq!(10, vt.finish());
            assert_eq!(16, builder.tell());
        }
    }

    #[test]
    fn test_vtable_builder() {
        let mut builder = Builder::new(|builder: &mut Builder| builder.tell());

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
            assert_eq!(expect, &builder.as_bytes()[4..10]);
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
            assert_eq!(expect, &builder.as_bytes()[10..20]);
        }
    }

    #[test]
    fn test_vtable_builder_alignment() {
        let mut builder = Builder::new(|builder: &mut Builder| builder.tell());

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
            assert_eq!(expect, &builder.as_bytes()[4..14]);
        }
    }
}
