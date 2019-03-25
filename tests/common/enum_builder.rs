pub mod example {
    #![allow(unused_imports, clippy::large_enum_variant)]

    use cfb::builder::{
        Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
        ScalarVectorComponent, StringComponent,
    };
    use cfb::scalar::Scalar;
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};
    #[cfg(not(target_endian = "little"))]
    use std::mem::transmute;

    #[repr(i8)]
    #[derive(Clone, Copy, PartialEq, Debug)]
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

    impl Scalar for Color {
        fn to_le(self) -> Self {
            #[cfg(target_endian = "little")]
            {
                self
            }
            #[cfg(not(target_endian = "little"))]
            {
                unsafe { transmute((self as i8).swap_bytes()) }
            }
        }

        fn from_le(x: Self) -> Self {
            #[cfg(target_endian = "little")]
            {
                x
            }
            #[cfg(not(target_endian = "little"))]
            {
                unsafe { transmute((x as i8).swap_bytes()) }
            }
        }
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Bag {
        pub color: Color,
    }

    impl Bag {
        const VT_COLOR: usize = 4;
        const SIZE_COLOR: usize = 1;
        const ALIGNMENT_COLOR: usize = 1;
        const ALIGNMENT: usize = 1;
    }

    impl<'c> Component<'c> for Bag {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.color != Color::Red {
                    vtable.add_field(Self::VT_COLOR, Self::SIZE_COLOR, Self::ALIGNMENT_COLOR);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.color != Color::Red {
                builder.align(Self::ALIGNMENT_COLOR);
                builder.push_scalar(self.color);
            }

            table_start
        }
    }
}
