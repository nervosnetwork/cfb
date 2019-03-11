#![allow(unused_imports)]

use cfb::builder::{
    Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
    ScalarVectorComponent, StringComponent,
};
use cfb::scalar::Scalar;
use cfb::types::{SOffset, SIZE_OF_SOFFSET};
#[cfg(not(target_endian = "little"))]
use std::mem::transmute;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
}

impl Item {
    const VT_NAME: usize = 4;
    const SIZE_NAME: usize = 4;
    const ALIGNMENT_NAME: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Item {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if !self.name.is_empty() {
                vtable.add_field(Self::VT_NAME, Self::SIZE_NAME, Self::ALIGNMENT_NAME);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if !self.name.is_empty() {
            builder.align(Self::ALIGNMENT_NAME);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_NAME);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(StringComponent::new(self.name))
            ));
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Monster {
    pub name: String,
    pub stat: Option<Stat>,
    pub loots: Vec<Item>,
}

impl Monster {
    const VT_NAME: usize = 4;
    const SIZE_NAME: usize = 4;
    const ALIGNMENT_NAME: usize = 4;
    const VT_STAT: usize = 6;
    const SIZE_STAT: usize = 4;
    const ALIGNMENT_STAT: usize = 4;
    const VT_LOOTS: usize = 8;
    const SIZE_LOOTS: usize = 4;
    const ALIGNMENT_LOOTS: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Monster {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if !self.name.is_empty() {
                vtable.add_field(Self::VT_NAME, Self::SIZE_NAME, Self::ALIGNMENT_NAME);
            }
            if self.stat.is_some() {
                vtable.add_field(Self::VT_STAT, Self::SIZE_STAT, Self::ALIGNMENT_STAT);
            }
            if !self.loots.is_empty() {
                vtable.add_field(Self::VT_LOOTS, Self::SIZE_LOOTS, Self::ALIGNMENT_LOOTS);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if !self.name.is_empty() {
            builder.align(Self::ALIGNMENT_NAME);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_NAME);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(StringComponent::new(self.name))
            ));
        }
        if let Some(f) = self.stat {
            builder.align(Self::ALIGNMENT_STAT);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_STAT);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(f),
            ));
        }
        if !self.loots.is_empty() {
            builder.align(Self::ALIGNMENT_LOOTS);
            let offset_position = builder.tell();
            builder.pad(Self::SIZE_LOOTS);
            builder.push_component(DesignatedComponent::new(
                offset_position,
                Box::new(ReferenceVectorComponent::new(self.loots)),
            ));
        }

        table_start
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Stat {
    pub hp: u32,
    pub mp: u32,
}

impl Stat {
    const VT_HP: usize = 4;
    const SIZE_HP: usize = 4;
    const ALIGNMENT_HP: usize = 4;
    const VT_MP: usize = 6;
    const SIZE_MP: usize = 4;
    const ALIGNMENT_MP: usize = 4;
    const ALIGNMENT: usize = 4;
}

impl<'c> Component<'c> for Stat {
    fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.hp != 0u32 {
                vtable.add_field(Self::VT_HP, Self::SIZE_HP, Self::ALIGNMENT_HP);
            }
            if self.mp != 0u32 {
                vtable.add_field(Self::VT_MP, Self::SIZE_MP, Self::ALIGNMENT_MP);
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

        let table_start = builder.tell();
        builder.push_scalar((table_start - vtable_start) as SOffset);
        if self.hp != 0u32 {
            builder.align(Self::ALIGNMENT_HP);
            builder.push_scalar(self.hp);
        }
        if self.mp != 0u32 {
            builder.align(Self::ALIGNMENT_MP);
            builder.push_scalar(self.mp);
        }

        table_start
    }
}