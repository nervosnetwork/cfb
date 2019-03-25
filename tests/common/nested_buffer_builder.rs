#![allow(unused_imports, clippy::large_enum_variant)]

use cfb::builder::{
    Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
    ScalarVectorComponent, StringComponent,
};
use cfb::scalar::Scalar;
use cfb::types::{SOffset, SIZE_OF_SOFFSET};
#[cfg(not(target_endian = "little"))]
use std::mem::transmute;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Block {
    pub header: Option<Header>,
}

impl Block {
    const VT_HEADER: usize = 4;
    const SIZE_HEADER: usize = 4;
    const ALIGNMENT_HEADER: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Block {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.header.is_some() {
                vtable.add_field(Self::VT_HEADER, Self::SIZE_HEADER, Self::ALIGNMENT_HEADER);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if let Some(f) = self.header {
            builder.align(Self::ALIGNMENT_HEADER);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_HEADER);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(NestedBufferComponent::new(f)),
            ));
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Header {
    pub number: u64,
}

impl Header {
    const VT_NUMBER: usize = 4;
    const SIZE_NUMBER: usize = 8;
    const ALIGNMENT_NUMBER: usize = 8;
    const ALIGNMENT: usize = 8;
}

impl<'c> Component<'c> for Header {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.number != 0u64 {
                vtable.add_field(Self::VT_NUMBER, Self::SIZE_NUMBER, Self::ALIGNMENT_NUMBER);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.number != 0u64 {
            builder.align(Self::ALIGNMENT_NUMBER);
            builder.push_scalar(self.number);
        }

        table_start
    }
}