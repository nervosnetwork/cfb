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
    pub struct Account {
        pub year: u32,
        pub balance: u64,
    }

    impl Account {
        const VT_YEAR: usize = 4;
        const SIZE_YEAR: usize = 4;
        const ALIGNMENT_YEAR: usize = 4;
        const VT_BALANCE: usize = 6;
        const SIZE_BALANCE: usize = 8;
        const ALIGNMENT_BALANCE: usize = 8;
        const ALIGNMENT: usize = 8;
    }

    impl<'c> Component<'c> for Account {
        fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.balance != 0u64 {
                    vtable.add_field(Self::VT_BALANCE, Self::SIZE_BALANCE, Self::ALIGNMENT_BALANCE);
                }
                if self.year != 0u32 {
                    vtable.add_field(Self::VT_YEAR, Self::SIZE_YEAR, Self::ALIGNMENT_YEAR);
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

            let table_start = builder.tell();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.balance != 0u64 {
                builder.align(Self::ALIGNMENT_BALANCE);
                builder.push_scalar(self.balance);
            }
            if self.year != 0u32 {
                builder.align(Self::ALIGNMENT_YEAR);
                builder.push_scalar(self.year);
            }

            table_start
        }
    }
}
