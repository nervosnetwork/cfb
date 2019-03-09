#![allow(unused_imports)]

use cfb::builder::{
    Builder, Component, DesignatedComponent, ReferenceVectorComponent, ScalarVectorComponent,
    StringComponent,
};
use cfb::scalar::Scalar;
use cfb::types::{SOffset, SIZE_OF_SOFFSET};
#[cfg(not(target_endian = "little"))]
use std::mem::transmute;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct T1 {
    pub f1: u64,
    pub s1: Vec<u8>,
    pub f2: Option<T2>,
    pub s2: Vec<u8>,
    pub f3: Vec<u64>,
    pub s3: Vec<u8>,
    pub f4: String,
}

impl T1 {
    const VT_F1: usize = 4;
    const SIZE_F1: usize = 8;
    const ALIGNMENT_F1: usize = 8;
    const VT_S1: usize = 6;
    const SIZE_S1: usize = 4;
    const ALIGNMENT_S1: usize = 4;
    const VT_F2: usize = 8;
    const SIZE_F2: usize = 4;
    const ALIGNMENT_F2: usize = 4;
    const VT_S2: usize = 10;
    const SIZE_S2: usize = 4;
    const ALIGNMENT_S2: usize = 4;
    const VT_F3: usize = 12;
    const SIZE_F3: usize = 4;
    const ALIGNMENT_F3: usize = 4;
    const VT_S3: usize = 14;
    const SIZE_S3: usize = 4;
    const ALIGNMENT_S3: usize = 4;
    const VT_F4: usize = 16;
    const SIZE_F4: usize = 4;
    const ALIGNMENT_F4: usize = 4;
    const ALIGNMENT: usize = 8;
}

impl<'c> Component<'c> for T1 {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.f1 != 0u64 {
                vtable.add_field(Self::VT_F1, Self::SIZE_F1, Self::ALIGNMENT_F1);
            }
            if !self.s1.is_empty() {
                vtable.add_field(Self::VT_S1, Self::SIZE_S1, Self::ALIGNMENT_S1);
            }
            if self.f2.is_some() {
                vtable.add_field(Self::VT_F2, Self::SIZE_F2, Self::ALIGNMENT_F2);
            }
            if !self.s2.is_empty() {
                vtable.add_field(Self::VT_S2, Self::SIZE_S2, Self::ALIGNMENT_S2);
            }
            if !self.f3.is_empty() {
                vtable.add_field(Self::VT_F3, Self::SIZE_F3, Self::ALIGNMENT_F3);
            }
            if !self.s3.is_empty() {
                vtable.add_field(Self::VT_S3, Self::SIZE_S3, Self::ALIGNMENT_S3);
            }
            if !self.f4.is_empty() {
                vtable.add_field(Self::VT_F4, Self::SIZE_F4, Self::ALIGNMENT_F4);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.f1 != 0u64 {
            builder.align(Self::ALIGNMENT_F1);
            builder.push_scalar(self.f1);
        }
        if !self.s1.is_empty() {
            builder.align(Self::ALIGNMENT_S1);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_S1);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ScalarVectorComponent::new(self.s1, 1)),
            ));
        }
        if let Some(f) = self.f2 {
            builder.align(Self::ALIGNMENT_F2);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_F2);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(f),
            ));
        }
        if !self.s2.is_empty() {
            builder.align(Self::ALIGNMENT_S2);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_S2);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ScalarVectorComponent::new(self.s2, 1)),
            ));
        }
        if !self.f3.is_empty() {
            builder.align(Self::ALIGNMENT_F3);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_F3);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ScalarVectorComponent::new(self.f3, 8)),
            ));
        }
        if !self.s3.is_empty() {
            builder.align(Self::ALIGNMENT_S3);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_S3);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ScalarVectorComponent::new(self.s3, 1)),
            ));
        }
        if !self.f4.is_empty() {
            builder.align(Self::ALIGNMENT_F4);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_F4);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(StringComponent::new(self.f4))
            ));
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct T2 {
    pub f1: u8,
}

impl T2 {
    const VT_F1: usize = 4;
    const SIZE_F1: usize = 1;
    const ALIGNMENT_F1: usize = 1;
    const ALIGNMENT: usize = 1;
}

impl<'c> Component<'c> for T2 {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.f1 != 0u8 {
                vtable.add_field(Self::VT_F1, Self::SIZE_F1, Self::ALIGNMENT_F1);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.f1 != 0u8 {
            builder.align(Self::ALIGNMENT_F1);
            builder.push_scalar(self.f1);
        }

        table_start
    }
}