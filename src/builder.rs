use crate::alignment::{align, align_after};
use crate::scalar::Scalar;
use crate::types::{Len, UOffset, SIZE_OF_LEN, SIZE_OF_UOFFSET};

pub trait Component<'a> {
    /// Build the component and return the start position of the component in the buffer.
    fn build(self: Box<Self>, builder: &mut Builder<'a>) -> usize;
}

impl<'a, F> Component<'a> for F
where
    F: FnOnce(&mut Builder<'a>) -> usize,
{
    fn build(self: Box<Self>, builder: &mut Builder<'a>) -> usize {
        (self)(builder)
    }
}

pub struct DesignatedComponent<'a> {
    /// Where to store the UOffset in the buffer.
    offset_position: usize,
    component: Box<dyn Component<'a> + 'a>,
}

impl<'a> DesignatedComponent<'a> {
    pub fn new(
        offset_position: usize,
        component: Box<dyn Component<'a> + 'a>,
    ) -> DesignatedComponent<'a> {
        DesignatedComponent {
            offset_position,
            component,
        }
    }

    fn build(self, builder: &mut Builder<'a>) {
        let position = self.component.build(builder);
        let uoffset = (position - self.offset_position) as UOffset;

        dbg!((self.offset_position, uoffset));

        builder.set_scalar(self.offset_position, uoffset);
    }
}

pub struct Builder<'a> {
    buffer: Vec<u8>,
    components: Vec<DesignatedComponent<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new<C: Component<'a> + 'a>(root: C) -> Builder<'a> {
        Builder {
            buffer: vec![0u8; SIZE_OF_UOFFSET],
            components: vec![DesignatedComponent::new(0, Box::new(root))],
        }
    }

    pub fn with_capacity<C: Component<'a> + 'a>(capacity: usize, root: C) -> Builder<'a> {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.extend_from_slice(&[0u8; SIZE_OF_UOFFSET]);

        Builder {
            buffer,
            components: vec![DesignatedComponent::new(0, Box::new(root))],
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

    pub fn push_component<C: Component<'a> + 'a>(&mut self, offset_position: usize, component: C) {
        assert!(offset_position + 4 <= self.tell());
        self.components.push(DesignatedComponent::new(
            offset_position,
            Box::new(component),
        ))
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(bytes);
        self
    }

    pub fn push_scalar<T: Scalar>(&mut self, s: T) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn set_scalar<T: Scalar>(&mut self, position: usize, s: T) {
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
}

#[derive(Debug)]
pub struct StringComponent<T>(T);

impl<T: AsRef<str>> StringComponent<T> {
    pub fn new(s: T) -> Self {
        StringComponent(s)
    }
}

impl<'a, T: AsRef<str>> Component<'a> for StringComponent<T> {
    fn build(self: Box<Self>, builder: &mut Builder<'a>) -> usize {
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

impl<'a, T: AsRef<[u8]>> Component<'a> for ScalarsVectorComponent<T> {
    fn build(self: Box<Self>, builder: &mut Builder<'a>) -> usize {
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

impl<'a, T, I, C> Component<'a> for ReferencesVectorComponent<T>
where
    T: IntoIterator<Item = C, IntoIter = I>,
    I: ExactSizeIterator<Item = C> + DoubleEndedIterator<Item = C>,
    C: Component<'a> + 'a,
{
    fn build(self: Box<Self>, builder: &mut Builder<'a>) -> usize {
        builder.align_after(SIZE_OF_LEN, SIZE_OF_UOFFSET);
        let position = builder.tell();

        let iter = self.0.into_iter();
        let len = iter.len();

        let mut current_offset_position =
            position + SIZE_OF_LEN + len * SIZE_OF_UOFFSET - SIZE_OF_UOFFSET;
        builder.push_scalar(len as Len);
        builder.pad(len * SIZE_OF_UOFFSET);
        for c in iter.rev() {
            builder.push_component(current_offset_position, c);
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
            "s1".as_bytes(),
            // padding
            &[0u8, 0],
            // s2.len
            &2u32.to_le_bytes(),
            "s2".as_bytes(),
            // padding
            &[0u8],
        ]
        .concat();
        assert_eq!(expect, buf);
    }
}
