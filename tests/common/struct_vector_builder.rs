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

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Hero {
        pub stats: Vec<Stat>,
    }

    impl Hero {
        const VT_STATS: usize = 4;
        const SIZE_STATS: usize = 4;
        const ALIGNMENT_STATS: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Hero {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if !self.stats.is_empty() {
                    vtable.add_field(Self::VT_STATS, Self::SIZE_STATS, Self::ALIGNMENT_STATS);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if !self.stats.is_empty() {
                builder.align(Self::ALIGNMENT_STATS);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_STATS);
                builder.push_component(DesignatedComponent::new(
                    offset_position,
                    Box::new(ScalarVectorComponent::new(self.stats, 4)),
                ));
            }

            table_start
        }
    }

    #[repr(C, align(4))]
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Stat {
        pub hp: u32,
        pub mp: u8,
        pub padding0_: u8,
        pub padding1_: u16,
    }

    impl Stat {
        pub fn is_present(&self) -> bool {
            self.hp != 0u32 || self.mp != 0u8
        }
    }

    impl Scalar for Stat {
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
            self.hp = self.hp.to_le();
            self.mp = self.mp.to_le();
            self
        }

        #[cfg(not(target_endian = "little"))]
        fn from_le(mut x: Self) -> Self {
            x.hp = Scalar::from_le(x.hp);
            x.mp = Scalar::from_le(x.mp);
            x
        }
    }
}
