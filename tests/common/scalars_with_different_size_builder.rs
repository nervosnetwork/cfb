pub mod example {
    use cfb::builder::{Builder, Component};
    use cfb::types::{SOffset, SIZE_OF_SOFFSET};

    #[derive(Default, Debug)]
    pub struct AccountComponent {
        pub year: u32,
        pub balance: u64,
    }

    impl AccountComponent {
        const VT_YEAR: usize = 4;
        const SIZE_YEAR: usize = 4;
        const ALIGNMENT_YEAR: usize = 4;
        const VT_BALANCE: usize = 6;
        const SIZE_BALANCE: usize = 8;
        const ALIGNMENT_BALANCE: usize = 8;
        const MAX_ALIGNMENT: usize = 8;
    }

    impl<'c> Component<'c> for AccountComponent {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.balance != 0 {
                    vtable.add_field(Self::VT_BALANCE, Self::SIZE_BALANCE, Self::ALIGNMENT_BALANCE);
                }
                if self.year != 0 {
                    vtable.add_field(Self::VT_YEAR, Self::SIZE_YEAR, Self::ALIGNMENT_YEAR);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::MAX_ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.balance != 0 {
                builder.align(Self::ALIGNMENT_BALANCE);
                builder.push_scalar(self.balance);
            }
            if self.year != 0 {
                builder.align(Self::ALIGNMENT_YEAR);
                builder.push_scalar(self.year);
            }

            table_start
        }
    }

}
