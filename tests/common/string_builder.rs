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
    pub struct Author {
        pub name: String,
    }

    impl Author {
        const VT_NAME: usize = 4;
        const SIZE_NAME: usize = 4;
        const ALIGNMENT_NAME: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Author {
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
}
