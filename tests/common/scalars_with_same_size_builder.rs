pub mod example {
    #![allow(unused_imports)]

    use cfb::builder::{
        Builder, Component, DesignatedComponent, ReferenceVectorComponent, ScalarVectorComponent, StringComponent,
    };
    use cfb::scalar::Scalar;
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};
    #[cfg(not(target_endian = "little"))]
    use std::mem::transmute;

    #[derive(Default, Debug)]
    pub struct Point {
        pub x: u64,
        pub y: u64,
    }

    impl Point {
        const VT_X: usize = 4;
        const SIZE_X: usize = 8;
        const ALIGNMENT_X: usize = 8;
        const VT_Y: usize = 6;
        const SIZE_Y: usize = 8;
        const ALIGNMENT_Y: usize = 8;
        const ALIGNMENT: usize = 8;
    }

    impl<'c> Component<'c> for Point {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.x != 0u64 {
                    vtable.add_field(Self::VT_X, Self::SIZE_X, Self::ALIGNMENT_X);
                }
                if self.y != 0u64 {
                    vtable.add_field(Self::VT_Y, Self::SIZE_Y, Self::ALIGNMENT_Y);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.x != 0u64 {
                builder.align(Self::ALIGNMENT_X);
                builder.push_scalar(self.x);
            }
            if self.y != 0u64 {
                builder.align(Self::ALIGNMENT_Y);
                builder.push_scalar(self.y);
            }

            table_start
        }
    }
}
