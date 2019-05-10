#![allow(unused_imports)]

use cfb_runtime::types::{SOffset, SIZE_OF_SOFFSET, SIZE_OF_UOFFSET};
use cfb_runtime::{Builder, PushReferenceInto, PushScalarInto};

#[derive(Debug)]
pub struct BagBuilder {
    pub color: Color,
}

impl BagBuilder {
    const VT_COLOR: usize = 4;
    const SIZE_COLOR: usize = 1;
    const ALIGNMENT_COLOR: usize = 1;
    const ALIGNMENT: usize = 1;
}

impl Default for BagBuilder {
    fn default() -> Self {
        BagBuilder { color: Color::Blue }
    }
}

impl PushReferenceInto for BagBuilder {
    fn push_into(self, builder: &mut Builder) -> usize {
        let vtable_start = {
            let mut vtable = builder.start_vtable();
            if self.color != Color::Blue {
                vtable.add_field(
                    BagBuilder::VT_COLOR,
                    BagBuilder::SIZE_COLOR,
                    BagBuilder::ALIGNMENT_COLOR,
                );
            }
            vtable.finish()
        };

        builder.align_after(SIZE_OF_SOFFSET, BagBuilder::ALIGNMENT);
        let table_start = builder.len();
        builder.push_scalar((table_start - vtable_start) as SOffset);

        if self.color != Color::Blue {
            builder.align(BagBuilder::ALIGNMENT_COLOR);
            builder.push_scalar(self.color);
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

impl PushScalarInto for Color {
    fn push_into(self, builder: &mut Builder) {
        (self as i8).push_into(builder)
    }
}
