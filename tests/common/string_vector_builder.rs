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
    pub struct Text {
        pub lines: Vec<String>,
    }

    impl Text {
        const VT_LINES: usize = 4;
        const SIZE_LINES: usize = 4;
        const ALIGNMENT_LINES: usize = 4;
        const ALIGNMENT: usize = 4;
    }

    impl<'c> Component<'c> for Text {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if !self.lines.is_empty() {
                    vtable.add_field(Self::VT_LINES, Self::SIZE_LINES, Self::ALIGNMENT_LINES);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if !self.lines.is_empty() {
                builder.align(Self::ALIGNMENT_LINES);
                let offset_position = builder.tell();
                builder.pad(Self::SIZE_LINES);
                let children = self.lines.into_iter().map(StringComponent::new);
                builder.push_component(DesignatedComponent::new(
                    offset_position,
                    Box::new(ReferenceVectorComponent::new(children)),
                ));
            }

            table_start
        }
    }
}
