pub mod example {
    use cfb::builder::{Builder, Component};
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};

    #[derive(Default, Debug)]
    pub struct PointComponent {
        pub x: u64,
        pub y: u64,
    }

    impl PointComponent {
        const VT_X: usize = 4;
        const SIZE_X: usize = 8;
        const ALIGNMENT_X: usize = 8;
        const VT_Y: usize = 6;
        const SIZE_Y: usize = 8;
        const ALIGNMENT_Y: usize = 8;
        const MAX_ALIGNMENT: usize = 8;
    }

    impl<'c> Component<'c> for PointComponent {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.x != 0 {
                    vtable.add_field(Self::VT_X, Self::SIZE_X, Self::ALIGNMENT_X);
                }
                if self.y != 0 {
                    vtable.add_field(Self::VT_Y, Self::SIZE_Y, Self::ALIGNMENT_Y);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::MAX_ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.x != 0 {
                builder.push_scalar(self.x);
            }
            if self.y != 0 {
                builder.push_scalar(self.y);
            }

            table_start
        }
    }

}
