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

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Hero {
        pub stat: Option<Stat>,
    }

    impl Hero {
        const VT_STAT: usize = 4;
        const SIZE_STAT: usize = 4;
        const ALIGNMENT_STAT: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Hero {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.stat.is_some() {
                    vtable.add_field(Self::VT_STAT, Self::SIZE_STAT, Self::ALIGNMENT_STAT);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if let Some(f) = self.stat {
                builder.align(Self::ALIGNMENT_STAT);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_STAT);
                builder.push_component(DesignatedComponent::new(
                    offset_position,
                    Box::new(f),
                ));
            }

            table_start
        }
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Stat {
        pub hp: u32,
    }

    impl Stat {
        const VT_HP: usize = 4;
        const SIZE_HP: usize = 4;
        const ALIGNMENT_HP: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Stat {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.hp != 0u32 {
                    vtable.add_field(Self::VT_HP, Self::SIZE_HP, Self::ALIGNMENT_HP);
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

            table_start
        }
    }
}
