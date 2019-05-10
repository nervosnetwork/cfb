#![allow(unused_imports)]

use cfb_runtime::types::{SOffset, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET};
use cfb_runtime::{Builder, PushReferenceInto, PushScalarInto};

#[repr(C, align(8))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Inner {
    pub x: u64,
}

impl Inner {
    pub fn is_present(&self) -> bool {
        self.x != 0
    }
}

impl PushScalarInto for Inner {
    #[cfg(target_endian = "little")]
    fn push_into(self, builder: &mut Builder) {
        builder.push_direct(&self)
    }

    #[cfg(not(target_endian = "little"))]
    fn push_into(self, builder: &mut Builder) {
        builder.push_scalar(self.x);
    }
}

#[repr(C, align(8))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Outter {
    pub x: Inner,
    pub y: Color,
    pub padding0_: u8,
    pub padding1_: u16,
    pub padding2_: u32,
    pub z: u64,
}

impl Outter {
    pub fn is_present(&self) -> bool {
        self.x.is_present() || self.y != Color::Red || self.z != 0
    }
}

impl PushScalarInto for Outter {
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

#[derive(Debug)]
pub struct WrapperBuilder {
    pub outter: Outter,
}

impl WrapperBuilder {
    const VT_OUTTER: usize = 4;
    const SIZE_OUTTER: usize = 24;
    const ALIGNMENT_OUTTER: usize = 8;
    const ALIGNMENT: usize = 8;
}

impl Default for WrapperBuilder {
    fn default() -> Self {
        WrapperBuilder {
            outter: Default::default(),
        }
    }
}

impl PushReferenceInto for WrapperBuilder {
    fn push_into(self, builder: &mut Builder) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.outter.is_present() {
                vtable.add_field(
                    WrapperBuilder::VT_OUTTER,
                    WrapperBuilder::SIZE_OUTTER,
                    WrapperBuilder::ALIGNMENT_OUTTER,
                );
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, WrapperBuilder::ALIGNMENT);
        let table_start = builder.len();
        builder.push_scalar((table_start - vtable_start) as SOffset);

        if self.outter.is_present() {
            builder.align(WrapperBuilder::ALIGNMENT_OUTTER);
            builder.push_scalar(self.outter);
        }

        table_start
    }
}

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}

impl PushScalarInto for Color {
    fn push_into(self, builder: &mut Builder) {
        (self as i8).push_into(builder)
    }
}
