use crate::alignment::{align, align_end};
use crate::types::{Len, UOffset, SIZE_OF_LEN, SIZE_OF_UOFFSET};

pub trait Component {
    /// Build the component and return the start position of the component in the buffer.
    fn build(&self, builder: &mut Builder) -> usize;
}

struct DesignatedComponent<'a> {
    /// Where to store the UOffset in the buffer.
    offset_position: usize,
    component: Box<dyn Component + 'a>,
}

impl<T> Component for T
where
    T: Fn(&mut Builder) -> usize,
{
    fn build(&self, builder: &mut Builder) -> usize {
        self(builder)
    }
}

impl<'a> DesignatedComponent<'a> {
    fn new<T: Component + 'a>(offset_position: usize, component: T) -> DesignatedComponent<'a> {
        DesignatedComponent {
            offset_position,
            component: Box::new(component),
        }
    }

    fn build(self, builder: &mut Builder) {
        let position = self.component.build(builder);
        let uoffset = (position - self.offset_position) as UOffset;

        builder.put_uoffset_at(uoffset, self.offset_position);
    }
}

pub struct Builder<'a> {
    buffer: Vec<u8>,
    components: Vec<DesignatedComponent<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new<T: Component + 'a>(root: T) -> Builder<'a> {
        Builder {
            buffer: vec![0u8; SIZE_OF_UOFFSET],
            components: vec![DesignatedComponent::new(0, root)],
        }
    }

    pub fn with_capacity<T: Component + 'a>(capacity: usize, root: T) -> Builder<'a> {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.extend_from_slice(&[0u8; SIZE_OF_UOFFSET]);

        Builder {
            buffer,
            components: vec![DesignatedComponent::new(0, root)],
        }
    }

    pub fn build(mut self) -> Vec<u8> {
        while let Some(component) = self.components.pop() {
            component.build(&mut self);
        }

        self.buffer
    }

    pub fn tell(&self) -> usize {
        self.buffer.len()
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn push(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    /// Append paddings to ensure the next appended data is aligned.
    pub fn align(&mut self, alignment: usize) {
        self.buffer.resize(align(self.tell(), alignment), 0)
    }

    /// Append paddings to ensure that, after data of lenth `len` has been appended, the next appended data is aligned.
    pub fn align_end(&mut self, len: usize, alignment: usize) {
        self.buffer
            .resize(align_end(self.tell(), len, alignment), 0)
    }

    fn put_uoffset_at(&mut self, uoffset: UOffset, position: usize) {
        let target = &mut self.buffer[position..position + SIZE_OF_UOFFSET];
        target.copy_from_slice(&uoffset.to_le_bytes());
    }

    fn put_len(&mut self, n: Len) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
}

pub struct StringComponent<T>(T);

impl<T: AsRef<str>> StringComponent<T> {
    pub fn new(s: T) -> Self {
        StringComponent(s)
    }
}

impl<T: AsRef<str>> Component for StringComponent<T> {
    fn build(&self, builder: &mut Builder) -> usize {
        let s = self.0.as_ref();

        builder.align(SIZE_OF_LEN);
        let position = builder.tell();

        builder.put_len(s.len() as Len);
        builder.extend_from_slice(s.as_bytes());
        builder.push(0);

        position
    }
}

pub struct RawVectorComponent<T> {
    elements: T,
    len: usize,
}

impl<T: AsRef<[u8]>> RawVectorComponent<T> {
    pub fn new(elements: T, len: usize) -> Self {
        RawVectorComponent { elements, len }
    }
}

impl<T: AsRef<[u8]>> Component for RawVectorComponent<T> {
    fn build(&self, builder: &mut Builder) -> usize {
        let bytes = self.elements.as_ref();

        let alignment = bytes.len() / self.len;
        builder.align_end(SIZE_OF_LEN, alignment);
        let position = builder.tell();

        builder.put_len(self.len as Len);
        builder.extend_from_slice(bytes);

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
            &4u32.to_le_bytes(),
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
        builder.push(0);
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
    fn test_raw_vector_component() {
        let raw: Vec<u8> = vec![1u32, 9]
            .into_iter()
            .map(|n| n.to_le_bytes().to_vec())
            .flatten()
            .collect();
        let builder = Builder::new(RawVectorComponent::new(raw.clone(), 2));
        let buf = builder.build();

        let expect = [
            // root uoffset
            &4u32.to_le_bytes(),
            // len
            &2u32.to_le_bytes(),
            // content
            &raw[..],
        ]
        .concat();
        assert_eq!(expect, buf);
    }

}
