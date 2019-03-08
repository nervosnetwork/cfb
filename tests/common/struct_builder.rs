pub mod example {
    #![allow(unused_imports)]

    use cfb::builder::{
        Builder, Component, DesignatedComponent, ReferenceVectorComponent, ScalarVectorComponent,
        StringComponent,
    };
    use cfb::scalar::Scalar;
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};
    #[cfg(not(target_endian = "little"))]
    use std::mem::transmute;

    #[derive(Default, Debug)]
    pub struct Point {
        pub position: Vec3,
    }

    impl Point {
        const VT_POSITION: usize = 4;
        const SIZE_POSITION: usize = 24;
        const ALIGNMENT_POSITION: usize = 8;
        const ALIGNMENT: usize = 8;
    }

    impl<'c> Component<'c> for Point {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.position.is_present() {
                    vtable.add_field(Self::VT_POSITION, Self::SIZE_POSITION, Self::ALIGNMENT_POSITION);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.position.is_present() {
                builder.align(Self::ALIGNMENT_POSITION);
                builder.push_scalar(self.position);
            }

            table_start
        }
    }

    #[repr(C, align(8))]
    #[derive(Default, Clone, Copy, Debug, PartialEq)]
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
            self.x != 0u64 || self.y != 0u8 || self.z != 0u64
        }
    }

    impl Scalar for Vec3 {
        #[cfg(target_endian = "little")]
        fn to_le(self) -> Self {
            self
        }

        #[cfg(target_endian = "little")]
        fn from_le(x: Self) -> Self {
            x
        }

        #[cfg(not(target_endian = "little"))]
        fn to_le(mut self) -> Self {
            self.x = self.x.to_le();
            self.y = self.y.to_le();
            self.z = self.z.to_le();
            self
        }

        #[cfg(not(target_endian = "little"))]
        fn from_le(mut x: Self) -> Self {
            x.x = Scalar::from_le(x.x);
            x.y = Scalar::from_le(x.y);
            x.z = Scalar::from_le(x.z);
            x
        }
    }
}
