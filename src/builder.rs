use crate::types::UOffset;
use crate::types::SIZE_OF_UOFFSET;

pub trait Component {
    /// Build the component and return the start position of the component in the buffer.
    fn build(&self, builder: &mut Builder) -> usize;
}

struct DesignatedComponent<'a> {
    /// Where to store the UOffset in the buffer.
    offset_position: usize,
    component: Box<dyn Component + 'a>,
}

pub struct Builder<'a> {
    buffer: Vec<u8>,
    components: Vec<DesignatedComponent<'a>>,
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

        builder.write_uoffset(self.offset_position, uoffset);
    }
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

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn write_uoffset(&mut self, position: usize, uoffset: UOffset) {
        let target = &mut self.buffer[position..position + SIZE_OF_UOFFSET];
        target.copy_from_slice(&uoffset.to_le_bytes()[..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_component() {
        let builder = Builder::new(|builder: &mut Builder| builder.len());
        let buf = builder.build();

        assert_eq!(vec![4u8, 0, 0, 0], buf);
    }
}
