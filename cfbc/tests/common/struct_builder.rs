#![allow(unused_imports)]

use cfb_runtime::types::{SOffset, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET};
use cfb_runtime::{Builder, PushReferenceInto, PushScalarInto};

#[derive(Debug)]
pub struct PointBuilder {
    pub position: Vec3,
}

impl PointBuilder {
    const VT_POSITION: usize = 4;
    const SIZE_POSITION: usize = 24;
    const ALIGNMENT_POSITION: usize = 8;
    const ALIGNMENT: usize = 8;
}

impl Default for PointBuilder {
    fn default() -> Self {
        PointBuilder {
            position: Default::default(),
        }
    }
}

impl PushReferenceInto for PointBuilder {
    fn push_into(self, builder: &mut Builder) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.position.is_present() {
                vtable.add_field(
                    PointBuilder::VT_POSITION,
                    PointBuilder::SIZE_POSITION,
                    PointBuilder::ALIGNMENT_POSITION,
                );
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, PointBuilder::ALIGNMENT);
        let table_start = builder.len();
        builder.push_scalar((table_start - vtable_start) as SOffset);

        if self.position.is_present() {
            builder.align(PointBuilder::ALIGNMENT_POSITION);
            builder.push_scalar(self.position);
        }

        table_start
    }
}

#[repr(C, align(8))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: u64,
    pub y: u8,
    pub padding0_: u8,
    pub padding1_: u16,
    pub padding2_: u32,
    pub z: u64,
}

impl Vec3 {
    pub fn is_present(&self) -> bool {
        self.x != 0 || self.y != 0 || self.z != 0
    }
}

impl PushScalarInto for Vec3 {
    #[cfg(target_endian = "little")]
    fn push_into(self, builder: &mut Builder) {
        builder.push_direct(&self)
    }

    #[cfg(not(target_endian = "little"))]
    fn push_into(self, builder: &mut Builder) {
        builder.push_scalar(self.x);
        builder.push_scalar(self.y);
        builder.push_scalar(self.padding0_);
        builder.push_scalar(self.padding1_);
        builder.push_scalar(self.padding2_);
        builder.push_scalar(self.z);
    }
}
