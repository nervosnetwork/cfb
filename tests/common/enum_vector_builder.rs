pub mod example {
    #![allow(unused_imports)]

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
        pub colors: Vec<Color>,
    }

    impl Bag {
        const VT_COLORS: usize = 4;
        const SIZE_COLORS: usize = 4;
        const ALIGNMENT_COLORS: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Bag {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if !self.colors.is_empty() {
                    vtable.add_field(Self::VT_COLORS, Self::SIZE_COLORS, Self::ALIGNMENT_COLORS);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if !self.colors.is_empty() {
                builder.align(Self::ALIGNMENT_COLORS);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_COLORS);
                builder.push_component(DesignatedComponent::new(
                    offset_position,
                    Box::new(ScalarVectorComponent::new(self.colors, 1)),
                ));
            }

            table_start
        }
    }
}
