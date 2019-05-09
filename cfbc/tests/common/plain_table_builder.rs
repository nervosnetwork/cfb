pub mod example {
    #![allow(unused_imports)]

    use cfb_runtime::types::{SOffset, SIZE_OF_SOFFSET};
    use cfb_runtime::{Builder, PushReferenceInto};

    #[derive(Default, Debug)]
    pub struct AccountBuilder {
        year: u32,
        balance: u64,
    }

    impl AccountBuilder {
        const VT_YEAR: usize = 4;
        const SIZE_YEAR: usize = 4;
        const ALIGNMENT_YEAR: usize = 4;
        const VT_BALANCE: usize = 6;
        const SIZE_BALANCE: usize = 8;
        const ALIGNMENT_BALANCE: usize = 8;
        const ALIGNMENT: usize = 8;

        pub fn new() -> Self {
            AccountBuilder::default()
        }

        pub fn year(&mut self, year: u32) -> &mut Self {
            self.year = year;
            self
        }

        pub fn balance(&mut self, balance: u64) -> &mut Self {
            self.balance = balance;
            self
        }
    }

    impl<'a> PushReferenceInto for &'a mut AccountBuilder {
        fn push_into(self, builder: &mut Builder) -> usize {
            let vtable_start = {
                let mut vtable = builder.start_vtable();
                if self.balance != 0 {
                    vtable.add_field(
                        AccountBuilder::VT_BALANCE,
                        AccountBuilder::SIZE_BALANCE,
                        AccountBuilder::ALIGNMENT_BALANCE,
                    );
                }
                if self.year != 0 {
                    vtable.add_field(
                        AccountBuilder::VT_YEAR,
                        AccountBuilder::SIZE_YEAR,
                        AccountBuilder::ALIGNMENT_YEAR,
                    );
                }
                vtable.finish()
            };

            builder.align_after(SIZE_OF_SOFFSET, AccountBuilder::ALIGNMENT);

            let table_start = builder.len();
            builder.push_scalar((table_start - vtable_start) as SOffset);
            if self.balance != 0 {
                builder.align(AccountBuilder::ALIGNMENT_BALANCE);
                builder.push_scalar(self.balance);
            }
            if self.year != 0 {
                builder.align(AccountBuilder::ALIGNMENT_YEAR);
                builder.push_scalar(self.year);
            }

            table_start
        }
    }
}
