pub mod ckb {
    pub mod protocol {
        #![allow(unused_imports, clippy::large_enum_variant)]

        use cfb::builder::{
            Builder, Component, DesignatedComponent, NestedBufferComponent, ReferenceVectorComponent,
            ScalarVectorComponent, StringComponent,
        };
        use cfb::scalar::Scalar;
        use cfb::types::{SOffset, SIZE_OF_SOFFSET};
        #[cfg(not(target_endian = "little"))]
        use std::mem::transmute;

        #[derive(Clone, PartialEq, Debug)]
        pub enum RelayPayload {
            CompactBlock(CompactBlock),
            ValidTransaction(ValidTransaction),
            GetBlockTransactions(GetBlockTransactions),
            BlockTransactions(BlockTransactions),
            GetBlockProposal(GetBlockProposal),
            BlockProposal(BlockProposal),
        }

        impl RelayPayload {
            pub fn union_type(&self) -> u8 {
                match self {
                    RelayPayload::CompactBlock(_) => 1,
                    RelayPayload::ValidTransaction(_) => 2,
                    RelayPayload::GetBlockTransactions(_) => 3,
                    RelayPayload::BlockTransactions(_) => 4,
                    RelayPayload::GetBlockProposal(_) => 5,
                    RelayPayload::BlockProposal(_) => 6,
                }
            }
        }

        #[derive(Clone, PartialEq, Debug)]
        pub enum SyncPayload {
            GetHeaders(GetHeaders),
            Headers(Headers),
            GetBlocks(GetBlocks),
            Block(Block),
            SetFilter(SetFilter),
            AddFilter(AddFilter),
            ClearFilter(ClearFilter),
            FilteredBlock(FilteredBlock),
        }

        impl SyncPayload {
            pub fn union_type(&self) -> u8 {
                match self {
                    SyncPayload::GetHeaders(_) => 1,
                    SyncPayload::Headers(_) => 2,
                    SyncPayload::GetBlocks(_) => 3,
                    SyncPayload::Block(_) => 4,
                    SyncPayload::SetFilter(_) => 5,
                    SyncPayload::AddFilter(_) => 6,
                    SyncPayload::ClearFilter(_) => 7,
                    SyncPayload::FilteredBlock(_) => 8,
                }
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct AddFilter {
            pub filter: Vec<u8>,
        }

        impl AddFilter {
            const VT_FILTER: usize = 4;
            const SIZE_FILTER: usize = 4;
            const ALIGNMENT_FILTER: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for AddFilter {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.filter.is_empty() {
                        vtable.add_field(Self::VT_FILTER, Self::SIZE_FILTER, Self::ALIGNMENT_FILTER);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.filter.is_empty() {
                    builder.align(Self::ALIGNMENT_FILTER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_FILTER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.filter, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Block {
            pub header: Option<Header>,
            pub uncles: Vec<UncleBlock>,
            pub commit_transactions: Vec<Transaction>,
            pub proposal_transactions: Vec<ProposalShortId>,
        }

        impl Block {
            const VT_HEADER: usize = 4;
            const SIZE_HEADER: usize = 4;
            const ALIGNMENT_HEADER: usize = 4;
            const VT_UNCLES: usize = 6;
            const SIZE_UNCLES: usize = 4;
            const ALIGNMENT_UNCLES: usize = 4;
            const VT_COMMIT_TRANSACTIONS: usize = 8;
            const SIZE_COMMIT_TRANSACTIONS: usize = 4;
            const ALIGNMENT_COMMIT_TRANSACTIONS: usize = 4;
            const VT_PROPOSAL_TRANSACTIONS: usize = 10;
            const SIZE_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for Block {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.header.is_some() {
                        vtable.add_field(Self::VT_HEADER, Self::SIZE_HEADER, Self::ALIGNMENT_HEADER);
                    }
                    if !self.uncles.is_empty() {
                        vtable.add_field(Self::VT_UNCLES, Self::SIZE_UNCLES, Self::ALIGNMENT_UNCLES);
                    }
                    if !self.commit_transactions.is_empty() {
                        vtable.add_field(Self::VT_COMMIT_TRANSACTIONS, Self::SIZE_COMMIT_TRANSACTIONS, Self::ALIGNMENT_COMMIT_TRANSACTIONS);
                    }
                    if !self.proposal_transactions.is_empty() {
                        vtable.add_field(Self::VT_PROPOSAL_TRANSACTIONS, Self::SIZE_PROPOSAL_TRANSACTIONS, Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if let Some(f) = self.header {
                    builder.align(Self::ALIGNMENT_HEADER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_HEADER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if !self.uncles.is_empty() {
                    builder.align(Self::ALIGNMENT_UNCLES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_UNCLES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.uncles)),
                    ));
                }
                if !self.commit_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_COMMIT_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_COMMIT_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.commit_transactions)),
                    ));
                }
                if !self.proposal_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROPOSAL_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.proposal_transactions, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct BlockProposal {
            pub transactions: Vec<Transaction>,
        }

        impl BlockProposal {
            const VT_TRANSACTIONS: usize = 4;
            const SIZE_TRANSACTIONS: usize = 4;
            const ALIGNMENT_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for BlockProposal {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.transactions.is_empty() {
                        vtable.add_field(Self::VT_TRANSACTIONS, Self::SIZE_TRANSACTIONS, Self::ALIGNMENT_TRANSACTIONS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.transactions)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct BlockTransactions {
            pub hash: H256,
            pub transactions: Vec<Transaction>,
        }

        impl BlockTransactions {
            const VT_HASH: usize = 4;
            const SIZE_HASH: usize = 32;
            const ALIGNMENT_HASH: usize = 1;
            const VT_TRANSACTIONS: usize = 6;
            const SIZE_TRANSACTIONS: usize = 4;
            const ALIGNMENT_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for BlockTransactions {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.transactions.is_empty() {
                        vtable.add_field(Self::VT_TRANSACTIONS, Self::SIZE_TRANSACTIONS, Self::ALIGNMENT_TRANSACTIONS);
                    }
                    if self.hash.is_present() {
                        vtable.add_field(Self::VT_HASH, Self::SIZE_HASH, Self::ALIGNMENT_HASH);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.transactions)),
                    ));
                }
                if self.hash.is_present() {
                    builder.align(Self::ALIGNMENT_HASH);
                    builder.push_scalar(self.hash);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Bytes {
            pub seq: Vec<u8>,
        }

        impl Bytes {
            const VT_SEQ: usize = 4;
            const SIZE_SEQ: usize = 4;
            const ALIGNMENT_SEQ: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for Bytes {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.seq.is_empty() {
                        vtable.add_field(Self::VT_SEQ, Self::SIZE_SEQ, Self::ALIGNMENT_SEQ);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.seq.is_empty() {
                    builder.align(Self::ALIGNMENT_SEQ);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_SEQ);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.seq, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct CellInput {
            pub hash: H256,
            pub index: u32,
            pub args: Vec<Bytes>,
        }

        impl CellInput {
            const VT_HASH: usize = 4;
            const SIZE_HASH: usize = 32;
            const ALIGNMENT_HASH: usize = 1;
            const VT_INDEX: usize = 6;
            const SIZE_INDEX: usize = 4;
            const ALIGNMENT_INDEX: usize = 4;
            const VT_ARGS: usize = 8;
            const SIZE_ARGS: usize = 4;
            const ALIGNMENT_ARGS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for CellInput {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.index != 0u32 {
                        vtable.add_field(Self::VT_INDEX, Self::SIZE_INDEX, Self::ALIGNMENT_INDEX);
                    }
                    if !self.args.is_empty() {
                        vtable.add_field(Self::VT_ARGS, Self::SIZE_ARGS, Self::ALIGNMENT_ARGS);
                    }
                    if self.hash.is_present() {
                        vtable.add_field(Self::VT_HASH, Self::SIZE_HASH, Self::ALIGNMENT_HASH);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.index != 0u32 {
                    builder.align(Self::ALIGNMENT_INDEX);
                    builder.push_scalar(self.index);
                }
                if !self.args.is_empty() {
                    builder.align(Self::ALIGNMENT_ARGS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_ARGS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.args)),
                    ));
                }
                if self.hash.is_present() {
                    builder.align(Self::ALIGNMENT_HASH);
                    builder.push_scalar(self.hash);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct CellOutput {
            pub capacity: u64,
            pub data: Option<Bytes>,
            pub lock: Option<Script>,
            pub type_: Option<Script>,
        }

        impl CellOutput {
            const VT_CAPACITY: usize = 4;
            const SIZE_CAPACITY: usize = 8;
            const ALIGNMENT_CAPACITY: usize = 8;
            const VT_DATA: usize = 6;
            const SIZE_DATA: usize = 4;
            const ALIGNMENT_DATA: usize = 4;
            const VT_LOCK: usize = 8;
            const SIZE_LOCK: usize = 4;
            const ALIGNMENT_LOCK: usize = 4;
            const VT_TYPE_: usize = 10;
            const SIZE_TYPE_: usize = 4;
            const ALIGNMENT_TYPE_: usize = 4;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for CellOutput {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.capacity != 0u64 {
                        vtable.add_field(Self::VT_CAPACITY, Self::SIZE_CAPACITY, Self::ALIGNMENT_CAPACITY);
                    }
                    if self.data.is_some() {
                        vtable.add_field(Self::VT_DATA, Self::SIZE_DATA, Self::ALIGNMENT_DATA);
                    }
                    if self.lock.is_some() {
                        vtable.add_field(Self::VT_LOCK, Self::SIZE_LOCK, Self::ALIGNMENT_LOCK);
                    }
                    if self.type_.is_some() {
                        vtable.add_field(Self::VT_TYPE_, Self::SIZE_TYPE_, Self::ALIGNMENT_TYPE_);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.capacity != 0u64 {
                    builder.align(Self::ALIGNMENT_CAPACITY);
                    builder.push_scalar(self.capacity);
                }
                if let Some(f) = self.data {
                    builder.align(Self::ALIGNMENT_DATA);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_DATA);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if let Some(f) = self.lock {
                    builder.align(Self::ALIGNMENT_LOCK);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_LOCK);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if let Some(f) = self.type_ {
                    builder.align(Self::ALIGNMENT_TYPE_);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TYPE_);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct ClearFilter {
        }

        impl ClearFilter {
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for ClearFilter {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let vtable = builder.start_vtable();
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct CompactBlock {
            pub header: Option<Header>,
            pub nonce: u64,
            pub short_ids: Vec<Bytes>,
            pub prefilled_transactions: Vec<IndexTransaction>,
            pub uncles: Vec<UncleBlock>,
            pub proposal_transactions: Vec<ProposalShortId>,
        }

        impl CompactBlock {
            const VT_HEADER: usize = 4;
            const SIZE_HEADER: usize = 4;
            const ALIGNMENT_HEADER: usize = 4;
            const VT_NONCE: usize = 6;
            const SIZE_NONCE: usize = 8;
            const ALIGNMENT_NONCE: usize = 8;
            const VT_SHORT_IDS: usize = 8;
            const SIZE_SHORT_IDS: usize = 4;
            const ALIGNMENT_SHORT_IDS: usize = 4;
            const VT_PREFILLED_TRANSACTIONS: usize = 10;
            const SIZE_PREFILLED_TRANSACTIONS: usize = 4;
            const ALIGNMENT_PREFILLED_TRANSACTIONS: usize = 4;
            const VT_UNCLES: usize = 12;
            const SIZE_UNCLES: usize = 4;
            const ALIGNMENT_UNCLES: usize = 4;
            const VT_PROPOSAL_TRANSACTIONS: usize = 14;
            const SIZE_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for CompactBlock {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.nonce != 0u64 {
                        vtable.add_field(Self::VT_NONCE, Self::SIZE_NONCE, Self::ALIGNMENT_NONCE);
                    }
                    if self.header.is_some() {
                        vtable.add_field(Self::VT_HEADER, Self::SIZE_HEADER, Self::ALIGNMENT_HEADER);
                    }
                    if !self.short_ids.is_empty() {
                        vtable.add_field(Self::VT_SHORT_IDS, Self::SIZE_SHORT_IDS, Self::ALIGNMENT_SHORT_IDS);
                    }
                    if !self.prefilled_transactions.is_empty() {
                        vtable.add_field(Self::VT_PREFILLED_TRANSACTIONS, Self::SIZE_PREFILLED_TRANSACTIONS, Self::ALIGNMENT_PREFILLED_TRANSACTIONS);
                    }
                    if !self.uncles.is_empty() {
                        vtable.add_field(Self::VT_UNCLES, Self::SIZE_UNCLES, Self::ALIGNMENT_UNCLES);
                    }
                    if !self.proposal_transactions.is_empty() {
                        vtable.add_field(Self::VT_PROPOSAL_TRANSACTIONS, Self::SIZE_PROPOSAL_TRANSACTIONS, Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.nonce != 0u64 {
                    builder.align(Self::ALIGNMENT_NONCE);
                    builder.push_scalar(self.nonce);
                }
                if let Some(f) = self.header {
                    builder.align(Self::ALIGNMENT_HEADER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_HEADER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if !self.short_ids.is_empty() {
                    builder.align(Self::ALIGNMENT_SHORT_IDS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_SHORT_IDS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.short_ids)),
                    ));
                }
                if !self.prefilled_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_PREFILLED_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PREFILLED_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.prefilled_transactions)),
                    ));
                }
                if !self.uncles.is_empty() {
                    builder.align(Self::ALIGNMENT_UNCLES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_UNCLES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.uncles)),
                    ));
                }
                if !self.proposal_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROPOSAL_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.proposal_transactions, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct FilteredBlock {
            pub header: Option<Header>,
            pub transactions: Vec<Transaction>,
            pub proof: Option<MerkleProof>,
        }

        impl FilteredBlock {
            const VT_HEADER: usize = 4;
            const SIZE_HEADER: usize = 4;
            const ALIGNMENT_HEADER: usize = 4;
            const VT_TRANSACTIONS: usize = 6;
            const SIZE_TRANSACTIONS: usize = 4;
            const ALIGNMENT_TRANSACTIONS: usize = 4;
            const VT_PROOF: usize = 8;
            const SIZE_PROOF: usize = 4;
            const ALIGNMENT_PROOF: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for FilteredBlock {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.header.is_some() {
                        vtable.add_field(Self::VT_HEADER, Self::SIZE_HEADER, Self::ALIGNMENT_HEADER);
                    }
                    if !self.transactions.is_empty() {
                        vtable.add_field(Self::VT_TRANSACTIONS, Self::SIZE_TRANSACTIONS, Self::ALIGNMENT_TRANSACTIONS);
                    }
                    if self.proof.is_some() {
                        vtable.add_field(Self::VT_PROOF, Self::SIZE_PROOF, Self::ALIGNMENT_PROOF);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if let Some(f) = self.header {
                    builder.align(Self::ALIGNMENT_HEADER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_HEADER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if !self.transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.transactions)),
                    ));
                }
                if let Some(f) = self.proof {
                    builder.align(Self::ALIGNMENT_PROOF);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROOF);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct GetBlockProposal {
            pub block_number: u64,
            pub proposal_transactions: Vec<ProposalShortId>,
        }

        impl GetBlockProposal {
            const VT_BLOCK_NUMBER: usize = 4;
            const SIZE_BLOCK_NUMBER: usize = 8;
            const ALIGNMENT_BLOCK_NUMBER: usize = 8;
            const VT_PROPOSAL_TRANSACTIONS: usize = 6;
            const SIZE_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for GetBlockProposal {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.block_number != 0u64 {
                        vtable.add_field(Self::VT_BLOCK_NUMBER, Self::SIZE_BLOCK_NUMBER, Self::ALIGNMENT_BLOCK_NUMBER);
                    }
                    if !self.proposal_transactions.is_empty() {
                        vtable.add_field(Self::VT_PROPOSAL_TRANSACTIONS, Self::SIZE_PROPOSAL_TRANSACTIONS, Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.block_number != 0u64 {
                    builder.align(Self::ALIGNMENT_BLOCK_NUMBER);
                    builder.push_scalar(self.block_number);
                }
                if !self.proposal_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROPOSAL_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.proposal_transactions, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct GetBlockTransactions {
            pub hash: H256,
            pub indexes: Vec<u32>,
        }

        impl GetBlockTransactions {
            const VT_HASH: usize = 4;
            const SIZE_HASH: usize = 32;
            const ALIGNMENT_HASH: usize = 1;
            const VT_INDEXES: usize = 6;
            const SIZE_INDEXES: usize = 4;
            const ALIGNMENT_INDEXES: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for GetBlockTransactions {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.indexes.is_empty() {
                        vtable.add_field(Self::VT_INDEXES, Self::SIZE_INDEXES, Self::ALIGNMENT_INDEXES);
                    }
                    if self.hash.is_present() {
                        vtable.add_field(Self::VT_HASH, Self::SIZE_HASH, Self::ALIGNMENT_HASH);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.indexes.is_empty() {
                    builder.align(Self::ALIGNMENT_INDEXES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_INDEXES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.indexes, 4)),
                    ));
                }
                if self.hash.is_present() {
                    builder.align(Self::ALIGNMENT_HASH);
                    builder.push_scalar(self.hash);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct GetBlocks {
            pub block_hashes: Vec<H256>,
        }

        impl GetBlocks {
            const VT_BLOCK_HASHES: usize = 4;
            const SIZE_BLOCK_HASHES: usize = 4;
            const ALIGNMENT_BLOCK_HASHES: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for GetBlocks {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.block_hashes.is_empty() {
                        vtable.add_field(Self::VT_BLOCK_HASHES, Self::SIZE_BLOCK_HASHES, Self::ALIGNMENT_BLOCK_HASHES);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.block_hashes.is_empty() {
                    builder.align(Self::ALIGNMENT_BLOCK_HASHES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_BLOCK_HASHES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.block_hashes, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct GetHeaders {
            pub version: u32,
            pub block_locator_hashes: Vec<H256>,
            pub hash_stop: H256,
        }

        impl GetHeaders {
            const VT_VERSION: usize = 4;
            const SIZE_VERSION: usize = 4;
            const ALIGNMENT_VERSION: usize = 4;
            const VT_BLOCK_LOCATOR_HASHES: usize = 6;
            const SIZE_BLOCK_LOCATOR_HASHES: usize = 4;
            const ALIGNMENT_BLOCK_LOCATOR_HASHES: usize = 4;
            const VT_HASH_STOP: usize = 8;
            const SIZE_HASH_STOP: usize = 32;
            const ALIGNMENT_HASH_STOP: usize = 1;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for GetHeaders {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.version != 0u32 {
                        vtable.add_field(Self::VT_VERSION, Self::SIZE_VERSION, Self::ALIGNMENT_VERSION);
                    }
                    if !self.block_locator_hashes.is_empty() {
                        vtable.add_field(Self::VT_BLOCK_LOCATOR_HASHES, Self::SIZE_BLOCK_LOCATOR_HASHES, Self::ALIGNMENT_BLOCK_LOCATOR_HASHES);
                    }
                    if self.hash_stop.is_present() {
                        vtable.add_field(Self::VT_HASH_STOP, Self::SIZE_HASH_STOP, Self::ALIGNMENT_HASH_STOP);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.version != 0u32 {
                    builder.align(Self::ALIGNMENT_VERSION);
                    builder.push_scalar(self.version);
                }
                if !self.block_locator_hashes.is_empty() {
                    builder.align(Self::ALIGNMENT_BLOCK_LOCATOR_HASHES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_BLOCK_LOCATOR_HASHES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.block_locator_hashes, 1)),
                    ));
                }
                if self.hash_stop.is_present() {
                    builder.align(Self::ALIGNMENT_HASH_STOP);
                    builder.push_scalar(self.hash_stop);
                }

                table_start
            }
        }

        #[repr(C, align(1))]
        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct H256 {
            pub u0: u8,
            pub u1: u8,
            pub u2: u8,
            pub u3: u8,
            pub u4: u8,
            pub u5: u8,
            pub u6: u8,
            pub u7: u8,
            pub u8_: u8,
            pub u9: u8,
            pub u10: u8,
            pub u11: u8,
            pub u12: u8,
            pub u13: u8,
            pub u14: u8,
            pub u15: u8,
            pub u16_: u8,
            pub u17: u8,
            pub u18: u8,
            pub u19: u8,
            pub u20: u8,
            pub u21: u8,
            pub u22: u8,
            pub u23: u8,
            pub u24: u8,
            pub u25: u8,
            pub u26: u8,
            pub u27: u8,
            pub u28: u8,
            pub u29: u8,
            pub u30: u8,
            pub u31: u8,
        }

        impl H256 {
            pub fn is_present(&self) -> bool {
                self.u0 != 0u8 || self.u1 != 0u8 || self.u2 != 0u8 || self.u3 != 0u8 || self.u4 != 0u8 || self.u5 != 0u8 || self.u6 != 0u8 || self.u7 != 0u8 || self.u8_ != 0u8 || self.u9 != 0u8 || self.u10 != 0u8 || self.u11 != 0u8 || self.u12 != 0u8 || self.u13 != 0u8 || self.u14 != 0u8 || self.u15 != 0u8 || self.u16_ != 0u8 || self.u17 != 0u8 || self.u18 != 0u8 || self.u19 != 0u8 || self.u20 != 0u8 || self.u21 != 0u8 || self.u22 != 0u8 || self.u23 != 0u8 || self.u24 != 0u8 || self.u25 != 0u8 || self.u26 != 0u8 || self.u27 != 0u8 || self.u28 != 0u8 || self.u29 != 0u8 || self.u30 != 0u8 || self.u31 != 0u8
            }
        }

        impl Scalar for H256 {
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
                self.u0 = self.u0.to_le();
                self.u1 = self.u1.to_le();
                self.u2 = self.u2.to_le();
                self.u3 = self.u3.to_le();
                self.u4 = self.u4.to_le();
                self.u5 = self.u5.to_le();
                self.u6 = self.u6.to_le();
                self.u7 = self.u7.to_le();
                self.u8_ = self.u8_.to_le();
                self.u9 = self.u9.to_le();
                self.u10 = self.u10.to_le();
                self.u11 = self.u11.to_le();
                self.u12 = self.u12.to_le();
                self.u13 = self.u13.to_le();
                self.u14 = self.u14.to_le();
                self.u15 = self.u15.to_le();
                self.u16_ = self.u16_.to_le();
                self.u17 = self.u17.to_le();
                self.u18 = self.u18.to_le();
                self.u19 = self.u19.to_le();
                self.u20 = self.u20.to_le();
                self.u21 = self.u21.to_le();
                self.u22 = self.u22.to_le();
                self.u23 = self.u23.to_le();
                self.u24 = self.u24.to_le();
                self.u25 = self.u25.to_le();
                self.u26 = self.u26.to_le();
                self.u27 = self.u27.to_le();
                self.u28 = self.u28.to_le();
                self.u29 = self.u29.to_le();
                self.u30 = self.u30.to_le();
                self.u31 = self.u31.to_le();
                self
            }

            #[cfg(not(target_endian = "little"))]
            fn from_le(mut x: Self) -> Self {
                x.u0 = Scalar::from_le(x.u0);
                x.u1 = Scalar::from_le(x.u1);
                x.u2 = Scalar::from_le(x.u2);
                x.u3 = Scalar::from_le(x.u3);
                x.u4 = Scalar::from_le(x.u4);
                x.u5 = Scalar::from_le(x.u5);
                x.u6 = Scalar::from_le(x.u6);
                x.u7 = Scalar::from_le(x.u7);
                x.u8_ = Scalar::from_le(x.u8_);
                x.u9 = Scalar::from_le(x.u9);
                x.u10 = Scalar::from_le(x.u10);
                x.u11 = Scalar::from_le(x.u11);
                x.u12 = Scalar::from_le(x.u12);
                x.u13 = Scalar::from_le(x.u13);
                x.u14 = Scalar::from_le(x.u14);
                x.u15 = Scalar::from_le(x.u15);
                x.u16_ = Scalar::from_le(x.u16_);
                x.u17 = Scalar::from_le(x.u17);
                x.u18 = Scalar::from_le(x.u18);
                x.u19 = Scalar::from_le(x.u19);
                x.u20 = Scalar::from_le(x.u20);
                x.u21 = Scalar::from_le(x.u21);
                x.u22 = Scalar::from_le(x.u22);
                x.u23 = Scalar::from_le(x.u23);
                x.u24 = Scalar::from_le(x.u24);
                x.u25 = Scalar::from_le(x.u25);
                x.u26 = Scalar::from_le(x.u26);
                x.u27 = Scalar::from_le(x.u27);
                x.u28 = Scalar::from_le(x.u28);
                x.u29 = Scalar::from_le(x.u29);
                x.u30 = Scalar::from_le(x.u30);
                x.u31 = Scalar::from_le(x.u31);
                x
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Header {
            pub version: u32,
            pub parent_hash: H256,
            pub timestamp: u64,
            pub number: u64,
            pub txs_commit: H256,
            pub txs_proposal: H256,
            pub difficulty: Option<Bytes>,
            pub nonce: u64,
            pub proof: Option<Bytes>,
            pub cellbase_id: H256,
            pub uncles_hash: H256,
            pub uncles_count: u32,
        }

        impl Header {
            const VT_VERSION: usize = 4;
            const SIZE_VERSION: usize = 4;
            const ALIGNMENT_VERSION: usize = 4;
            const VT_PARENT_HASH: usize = 6;
            const SIZE_PARENT_HASH: usize = 32;
            const ALIGNMENT_PARENT_HASH: usize = 1;
            const VT_TIMESTAMP: usize = 8;
            const SIZE_TIMESTAMP: usize = 8;
            const ALIGNMENT_TIMESTAMP: usize = 8;
            const VT_NUMBER: usize = 10;
            const SIZE_NUMBER: usize = 8;
            const ALIGNMENT_NUMBER: usize = 8;
            const VT_TXS_COMMIT: usize = 12;
            const SIZE_TXS_COMMIT: usize = 32;
            const ALIGNMENT_TXS_COMMIT: usize = 1;
            const VT_TXS_PROPOSAL: usize = 14;
            const SIZE_TXS_PROPOSAL: usize = 32;
            const ALIGNMENT_TXS_PROPOSAL: usize = 1;
            const VT_DIFFICULTY: usize = 16;
            const SIZE_DIFFICULTY: usize = 4;
            const ALIGNMENT_DIFFICULTY: usize = 4;
            const VT_NONCE: usize = 18;
            const SIZE_NONCE: usize = 8;
            const ALIGNMENT_NONCE: usize = 8;
            const VT_PROOF: usize = 20;
            const SIZE_PROOF: usize = 4;
            const ALIGNMENT_PROOF: usize = 4;
            const VT_CELLBASE_ID: usize = 22;
            const SIZE_CELLBASE_ID: usize = 32;
            const ALIGNMENT_CELLBASE_ID: usize = 1;
            const VT_UNCLES_HASH: usize = 24;
            const SIZE_UNCLES_HASH: usize = 32;
            const ALIGNMENT_UNCLES_HASH: usize = 1;
            const VT_UNCLES_COUNT: usize = 26;
            const SIZE_UNCLES_COUNT: usize = 4;
            const ALIGNMENT_UNCLES_COUNT: usize = 4;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for Header {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.timestamp != 0u64 {
                        vtable.add_field(Self::VT_TIMESTAMP, Self::SIZE_TIMESTAMP, Self::ALIGNMENT_TIMESTAMP);
                    }
                    if self.number != 0u64 {
                        vtable.add_field(Self::VT_NUMBER, Self::SIZE_NUMBER, Self::ALIGNMENT_NUMBER);
                    }
                    if self.nonce != 0u64 {
                        vtable.add_field(Self::VT_NONCE, Self::SIZE_NONCE, Self::ALIGNMENT_NONCE);
                    }
                    if self.version != 0u32 {
                        vtable.add_field(Self::VT_VERSION, Self::SIZE_VERSION, Self::ALIGNMENT_VERSION);
                    }
                    if self.difficulty.is_some() {
                        vtable.add_field(Self::VT_DIFFICULTY, Self::SIZE_DIFFICULTY, Self::ALIGNMENT_DIFFICULTY);
                    }
                    if self.proof.is_some() {
                        vtable.add_field(Self::VT_PROOF, Self::SIZE_PROOF, Self::ALIGNMENT_PROOF);
                    }
                    if self.uncles_count != 0u32 {
                        vtable.add_field(Self::VT_UNCLES_COUNT, Self::SIZE_UNCLES_COUNT, Self::ALIGNMENT_UNCLES_COUNT);
                    }
                    if self.parent_hash.is_present() {
                        vtable.add_field(Self::VT_PARENT_HASH, Self::SIZE_PARENT_HASH, Self::ALIGNMENT_PARENT_HASH);
                    }
                    if self.txs_commit.is_present() {
                        vtable.add_field(Self::VT_TXS_COMMIT, Self::SIZE_TXS_COMMIT, Self::ALIGNMENT_TXS_COMMIT);
                    }
                    if self.txs_proposal.is_present() {
                        vtable.add_field(Self::VT_TXS_PROPOSAL, Self::SIZE_TXS_PROPOSAL, Self::ALIGNMENT_TXS_PROPOSAL);
                    }
                    if self.cellbase_id.is_present() {
                        vtable.add_field(Self::VT_CELLBASE_ID, Self::SIZE_CELLBASE_ID, Self::ALIGNMENT_CELLBASE_ID);
                    }
                    if self.uncles_hash.is_present() {
                        vtable.add_field(Self::VT_UNCLES_HASH, Self::SIZE_UNCLES_HASH, Self::ALIGNMENT_UNCLES_HASH);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.timestamp != 0u64 {
                    builder.align(Self::ALIGNMENT_TIMESTAMP);
                    builder.push_scalar(self.timestamp);
                }
                if self.number != 0u64 {
                    builder.align(Self::ALIGNMENT_NUMBER);
                    builder.push_scalar(self.number);
                }
                if self.nonce != 0u64 {
                    builder.align(Self::ALIGNMENT_NONCE);
                    builder.push_scalar(self.nonce);
                }
                if self.version != 0u32 {
                    builder.align(Self::ALIGNMENT_VERSION);
                    builder.push_scalar(self.version);
                }
                if let Some(f) = self.difficulty {
                    builder.align(Self::ALIGNMENT_DIFFICULTY);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_DIFFICULTY);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if let Some(f) = self.proof {
                    builder.align(Self::ALIGNMENT_PROOF);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROOF);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if self.uncles_count != 0u32 {
                    builder.align(Self::ALIGNMENT_UNCLES_COUNT);
                    builder.push_scalar(self.uncles_count);
                }
                if self.parent_hash.is_present() {
                    builder.align(Self::ALIGNMENT_PARENT_HASH);
                    builder.push_scalar(self.parent_hash);
                }
                if self.txs_commit.is_present() {
                    builder.align(Self::ALIGNMENT_TXS_COMMIT);
                    builder.push_scalar(self.txs_commit);
                }
                if self.txs_proposal.is_present() {
                    builder.align(Self::ALIGNMENT_TXS_PROPOSAL);
                    builder.push_scalar(self.txs_proposal);
                }
                if self.cellbase_id.is_present() {
                    builder.align(Self::ALIGNMENT_CELLBASE_ID);
                    builder.push_scalar(self.cellbase_id);
                }
                if self.uncles_hash.is_present() {
                    builder.align(Self::ALIGNMENT_UNCLES_HASH);
                    builder.push_scalar(self.uncles_hash);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Headers {
            pub headers: Vec<Header>,
        }

        impl Headers {
            const VT_HEADERS: usize = 4;
            const SIZE_HEADERS: usize = 4;
            const ALIGNMENT_HEADERS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for Headers {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.headers.is_empty() {
                        vtable.add_field(Self::VT_HEADERS, Self::SIZE_HEADERS, Self::ALIGNMENT_HEADERS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.headers.is_empty() {
                    builder.align(Self::ALIGNMENT_HEADERS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_HEADERS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.headers)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct IndexTransaction {
            pub index: u32,
            pub transaction: Option<Transaction>,
        }

        impl IndexTransaction {
            const VT_INDEX: usize = 4;
            const SIZE_INDEX: usize = 4;
            const ALIGNMENT_INDEX: usize = 4;
            const VT_TRANSACTION: usize = 6;
            const SIZE_TRANSACTION: usize = 4;
            const ALIGNMENT_TRANSACTION: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for IndexTransaction {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.index != 0u32 {
                        vtable.add_field(Self::VT_INDEX, Self::SIZE_INDEX, Self::ALIGNMENT_INDEX);
                    }
                    if self.transaction.is_some() {
                        vtable.add_field(Self::VT_TRANSACTION, Self::SIZE_TRANSACTION, Self::ALIGNMENT_TRANSACTION);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.index != 0u32 {
                    builder.align(Self::ALIGNMENT_INDEX);
                    builder.push_scalar(self.index);
                }
                if let Some(f) = self.transaction {
                    builder.align(Self::ALIGNMENT_TRANSACTION);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TRANSACTION);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct MerkleProof {
            pub indices: Vec<u32>,
            pub lemmas: Vec<H256>,
        }

        impl MerkleProof {
            const VT_INDICES: usize = 4;
            const SIZE_INDICES: usize = 4;
            const ALIGNMENT_INDICES: usize = 4;
            const VT_LEMMAS: usize = 6;
            const SIZE_LEMMAS: usize = 4;
            const ALIGNMENT_LEMMAS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for MerkleProof {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.indices.is_empty() {
                        vtable.add_field(Self::VT_INDICES, Self::SIZE_INDICES, Self::ALIGNMENT_INDICES);
                    }
                    if !self.lemmas.is_empty() {
                        vtable.add_field(Self::VT_LEMMAS, Self::SIZE_LEMMAS, Self::ALIGNMENT_LEMMAS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.indices.is_empty() {
                    builder.align(Self::ALIGNMENT_INDICES);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_INDICES);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.indices, 4)),
                    ));
                }
                if !self.lemmas.is_empty() {
                    builder.align(Self::ALIGNMENT_LEMMAS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_LEMMAS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.lemmas, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct OutPoint {
            pub hash: H256,
            pub index: u32,
        }

        impl OutPoint {
            const VT_HASH: usize = 4;
            const SIZE_HASH: usize = 32;
            const ALIGNMENT_HASH: usize = 1;
            const VT_INDEX: usize = 6;
            const SIZE_INDEX: usize = 4;
            const ALIGNMENT_INDEX: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for OutPoint {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.index != 0u32 {
                        vtable.add_field(Self::VT_INDEX, Self::SIZE_INDEX, Self::ALIGNMENT_INDEX);
                    }
                    if self.hash.is_present() {
                        vtable.add_field(Self::VT_HASH, Self::SIZE_HASH, Self::ALIGNMENT_HASH);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.index != 0u32 {
                    builder.align(Self::ALIGNMENT_INDEX);
                    builder.push_scalar(self.index);
                }
                if self.hash.is_present() {
                    builder.align(Self::ALIGNMENT_HASH);
                    builder.push_scalar(self.hash);
                }

                table_start
            }
        }

        #[repr(C, align(1))]
        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct ProposalShortId {
            pub u0: u8,
            pub u1: u8,
            pub u2: u8,
            pub u3: u8,
            pub u4: u8,
            pub u5: u8,
            pub u6: u8,
            pub u7: u8,
            pub u8_: u8,
            pub u9: u8,
        }

        impl ProposalShortId {
            pub fn is_present(&self) -> bool {
                self.u0 != 0u8 || self.u1 != 0u8 || self.u2 != 0u8 || self.u3 != 0u8 || self.u4 != 0u8 || self.u5 != 0u8 || self.u6 != 0u8 || self.u7 != 0u8 || self.u8_ != 0u8 || self.u9 != 0u8
            }
        }

        impl Scalar for ProposalShortId {
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
                self.u0 = self.u0.to_le();
                self.u1 = self.u1.to_le();
                self.u2 = self.u2.to_le();
                self.u3 = self.u3.to_le();
                self.u4 = self.u4.to_le();
                self.u5 = self.u5.to_le();
                self.u6 = self.u6.to_le();
                self.u7 = self.u7.to_le();
                self.u8_ = self.u8_.to_le();
                self.u9 = self.u9.to_le();
                self
            }

            #[cfg(not(target_endian = "little"))]
            fn from_le(mut x: Self) -> Self {
                x.u0 = Scalar::from_le(x.u0);
                x.u1 = Scalar::from_le(x.u1);
                x.u2 = Scalar::from_le(x.u2);
                x.u3 = Scalar::from_le(x.u3);
                x.u4 = Scalar::from_le(x.u4);
                x.u5 = Scalar::from_le(x.u5);
                x.u6 = Scalar::from_le(x.u6);
                x.u7 = Scalar::from_le(x.u7);
                x.u8_ = Scalar::from_le(x.u8_);
                x.u9 = Scalar::from_le(x.u9);
                x
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct RelayMessage {
            pub payload: Option<RelayPayload>,
        }

        impl RelayMessage {
            const VT_PAYLOAD_TYPE: usize = 4;
            const SIZE_PAYLOAD_TYPE: usize = 1;
            const ALIGNMENT_PAYLOAD_TYPE: usize = 1;
            const VT_PAYLOAD: usize = 6;
            const SIZE_PAYLOAD: usize = 4;
            const ALIGNMENT_PAYLOAD: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for RelayMessage {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.payload.is_some() {
                        vtable.add_field(Self::VT_PAYLOAD, Self::SIZE_PAYLOAD, Self::ALIGNMENT_PAYLOAD);
                    }
                    if self.payload.is_some() {
                        vtable.add_field(Self::VT_PAYLOAD_TYPE, Self::SIZE_PAYLOAD_TYPE, Self::ALIGNMENT_PAYLOAD_TYPE);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                let payload_type = self.payload.as_ref().map(|v| v.union_type());
                if let Some(f) = self.payload {
                    builder.align(Self::ALIGNMENT_PAYLOAD);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PAYLOAD);
                    let component: Box<dyn Component<'c> + 'c> = match f {
                        RelayPayload::CompactBlock(v) => Box::new(v),
                        RelayPayload::ValidTransaction(v) => Box::new(v),
                        RelayPayload::GetBlockTransactions(v) => Box::new(v),
                        RelayPayload::BlockTransactions(v) => Box::new(v),
                        RelayPayload::GetBlockProposal(v) => Box::new(v),
                        RelayPayload::BlockProposal(v) => Box::new(v),
                    };
                    builder.push_component(DesignatedComponent::new(offset_position, component));
                }
                if let Some(f) = payload_type {
                    builder.align(Self::ALIGNMENT_PAYLOAD_TYPE);
                    builder.push_scalar(f);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Script {
            pub version: u8,
            pub args: Vec<Bytes>,
            pub binary_hash: H256,
        }

        impl Script {
            const VT_VERSION: usize = 4;
            const SIZE_VERSION: usize = 1;
            const ALIGNMENT_VERSION: usize = 1;
            const VT_ARGS: usize = 6;
            const SIZE_ARGS: usize = 4;
            const ALIGNMENT_ARGS: usize = 4;
            const VT_BINARY_HASH: usize = 8;
            const SIZE_BINARY_HASH: usize = 32;
            const ALIGNMENT_BINARY_HASH: usize = 1;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for Script {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.args.is_empty() {
                        vtable.add_field(Self::VT_ARGS, Self::SIZE_ARGS, Self::ALIGNMENT_ARGS);
                    }
                    if self.binary_hash.is_present() {
                        vtable.add_field(Self::VT_BINARY_HASH, Self::SIZE_BINARY_HASH, Self::ALIGNMENT_BINARY_HASH);
                    }
                    if self.version != 0u8 {
                        vtable.add_field(Self::VT_VERSION, Self::SIZE_VERSION, Self::ALIGNMENT_VERSION);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.args.is_empty() {
                    builder.align(Self::ALIGNMENT_ARGS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_ARGS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.args)),
                    ));
                }
                if self.binary_hash.is_present() {
                    builder.align(Self::ALIGNMENT_BINARY_HASH);
                    builder.push_scalar(self.binary_hash);
                }
                if self.version != 0u8 {
                    builder.align(Self::ALIGNMENT_VERSION);
                    builder.push_scalar(self.version);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct SetFilter {
            pub filter: Vec<u8>,
            pub num_hashes: u8,
            pub hash_seed: u32,
        }

        impl SetFilter {
            const VT_FILTER: usize = 4;
            const SIZE_FILTER: usize = 4;
            const ALIGNMENT_FILTER: usize = 4;
            const VT_NUM_HASHES: usize = 6;
            const SIZE_NUM_HASHES: usize = 1;
            const ALIGNMENT_NUM_HASHES: usize = 1;
            const VT_HASH_SEED: usize = 8;
            const SIZE_HASH_SEED: usize = 4;
            const ALIGNMENT_HASH_SEED: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for SetFilter {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if !self.filter.is_empty() {
                        vtable.add_field(Self::VT_FILTER, Self::SIZE_FILTER, Self::ALIGNMENT_FILTER);
                    }
                    if self.hash_seed != 0u32 {
                        vtable.add_field(Self::VT_HASH_SEED, Self::SIZE_HASH_SEED, Self::ALIGNMENT_HASH_SEED);
                    }
                    if self.num_hashes != 0u8 {
                        vtable.add_field(Self::VT_NUM_HASHES, Self::SIZE_NUM_HASHES, Self::ALIGNMENT_NUM_HASHES);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if !self.filter.is_empty() {
                    builder.align(Self::ALIGNMENT_FILTER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_FILTER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.filter, 1)),
                    ));
                }
                if self.hash_seed != 0u32 {
                    builder.align(Self::ALIGNMENT_HASH_SEED);
                    builder.push_scalar(self.hash_seed);
                }
                if self.num_hashes != 0u8 {
                    builder.align(Self::ALIGNMENT_NUM_HASHES);
                    builder.push_scalar(self.num_hashes);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct SyncMessage {
            pub payload: Option<SyncPayload>,
        }

        impl SyncMessage {
            const VT_PAYLOAD_TYPE: usize = 4;
            const SIZE_PAYLOAD_TYPE: usize = 1;
            const ALIGNMENT_PAYLOAD_TYPE: usize = 1;
            const VT_PAYLOAD: usize = 6;
            const SIZE_PAYLOAD: usize = 4;
            const ALIGNMENT_PAYLOAD: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for SyncMessage {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.payload.is_some() {
                        vtable.add_field(Self::VT_PAYLOAD, Self::SIZE_PAYLOAD, Self::ALIGNMENT_PAYLOAD);
                    }
                    if self.payload.is_some() {
                        vtable.add_field(Self::VT_PAYLOAD_TYPE, Self::SIZE_PAYLOAD_TYPE, Self::ALIGNMENT_PAYLOAD_TYPE);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                let payload_type = self.payload.as_ref().map(|v| v.union_type());
                if let Some(f) = self.payload {
                    builder.align(Self::ALIGNMENT_PAYLOAD);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PAYLOAD);
                    let component: Box<dyn Component<'c> + 'c> = match f {
                        SyncPayload::GetHeaders(v) => Box::new(v),
                        SyncPayload::Headers(v) => Box::new(v),
                        SyncPayload::GetBlocks(v) => Box::new(v),
                        SyncPayload::Block(v) => Box::new(v),
                        SyncPayload::SetFilter(v) => Box::new(v),
                        SyncPayload::AddFilter(v) => Box::new(v),
                        SyncPayload::ClearFilter(v) => Box::new(v),
                        SyncPayload::FilteredBlock(v) => Box::new(v),
                    };
                    builder.push_component(DesignatedComponent::new(offset_position, component));
                }
                if let Some(f) = payload_type {
                    builder.align(Self::ALIGNMENT_PAYLOAD_TYPE);
                    builder.push_scalar(f);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Time {
            pub timestamp: u64,
        }

        impl Time {
            const VT_TIMESTAMP: usize = 4;
            const SIZE_TIMESTAMP: usize = 8;
            const ALIGNMENT_TIMESTAMP: usize = 8;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for Time {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.timestamp != 0u64 {
                        vtable.add_field(Self::VT_TIMESTAMP, Self::SIZE_TIMESTAMP, Self::ALIGNMENT_TIMESTAMP);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.timestamp != 0u64 {
                    builder.align(Self::ALIGNMENT_TIMESTAMP);
                    builder.push_scalar(self.timestamp);
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct TimeMessage {
            pub payload: Option<Time>,
        }

        impl TimeMessage {
            const VT_PAYLOAD: usize = 4;
            const SIZE_PAYLOAD: usize = 4;
            const ALIGNMENT_PAYLOAD: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for TimeMessage {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.payload.is_some() {
                        vtable.add_field(Self::VT_PAYLOAD, Self::SIZE_PAYLOAD, Self::ALIGNMENT_PAYLOAD);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if let Some(f) = self.payload {
                    builder.align(Self::ALIGNMENT_PAYLOAD);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PAYLOAD);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct Transaction {
            pub version: u32,
            pub deps: Vec<OutPoint>,
            pub inputs: Vec<CellInput>,
            pub outputs: Vec<CellOutput>,
            pub embeds: Vec<Bytes>,
        }

        impl Transaction {
            const VT_VERSION: usize = 4;
            const SIZE_VERSION: usize = 4;
            const ALIGNMENT_VERSION: usize = 4;
            const VT_DEPS: usize = 6;
            const SIZE_DEPS: usize = 4;
            const ALIGNMENT_DEPS: usize = 4;
            const VT_INPUTS: usize = 8;
            const SIZE_INPUTS: usize = 4;
            const ALIGNMENT_INPUTS: usize = 4;
            const VT_OUTPUTS: usize = 10;
            const SIZE_OUTPUTS: usize = 4;
            const ALIGNMENT_OUTPUTS: usize = 4;
            const VT_EMBEDS: usize = 12;
            const SIZE_EMBEDS: usize = 4;
            const ALIGNMENT_EMBEDS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for Transaction {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.version != 0u32 {
                        vtable.add_field(Self::VT_VERSION, Self::SIZE_VERSION, Self::ALIGNMENT_VERSION);
                    }
                    if !self.deps.is_empty() {
                        vtable.add_field(Self::VT_DEPS, Self::SIZE_DEPS, Self::ALIGNMENT_DEPS);
                    }
                    if !self.inputs.is_empty() {
                        vtable.add_field(Self::VT_INPUTS, Self::SIZE_INPUTS, Self::ALIGNMENT_INPUTS);
                    }
                    if !self.outputs.is_empty() {
                        vtable.add_field(Self::VT_OUTPUTS, Self::SIZE_OUTPUTS, Self::ALIGNMENT_OUTPUTS);
                    }
                    if !self.embeds.is_empty() {
                        vtable.add_field(Self::VT_EMBEDS, Self::SIZE_EMBEDS, Self::ALIGNMENT_EMBEDS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.version != 0u32 {
                    builder.align(Self::ALIGNMENT_VERSION);
                    builder.push_scalar(self.version);
                }
                if !self.deps.is_empty() {
                    builder.align(Self::ALIGNMENT_DEPS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_DEPS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.deps)),
                    ));
                }
                if !self.inputs.is_empty() {
                    builder.align(Self::ALIGNMENT_INPUTS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_INPUTS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.inputs)),
                    ));
                }
                if !self.outputs.is_empty() {
                    builder.align(Self::ALIGNMENT_OUTPUTS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_OUTPUTS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.outputs)),
                    ));
                }
                if !self.embeds.is_empty() {
                    builder.align(Self::ALIGNMENT_EMBEDS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_EMBEDS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ReferenceVectorComponent::new(self.embeds)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct UncleBlock {
            pub header: Option<Header>,
            pub cellbase: Option<Transaction>,
            pub proposal_transactions: Vec<ProposalShortId>,
        }

        impl UncleBlock {
            const VT_HEADER: usize = 4;
            const SIZE_HEADER: usize = 4;
            const ALIGNMENT_HEADER: usize = 4;
            const VT_CELLBASE: usize = 6;
            const SIZE_CELLBASE: usize = 4;
            const ALIGNMENT_CELLBASE: usize = 4;
            const VT_PROPOSAL_TRANSACTIONS: usize = 8;
            const SIZE_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT_PROPOSAL_TRANSACTIONS: usize = 4;
            const ALIGNMENT: usize = 4;
        }

        impl<'c> Component<'c> for UncleBlock {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.header.is_some() {
                        vtable.add_field(Self::VT_HEADER, Self::SIZE_HEADER, Self::ALIGNMENT_HEADER);
                    }
                    if self.cellbase.is_some() {
                        vtable.add_field(Self::VT_CELLBASE, Self::SIZE_CELLBASE, Self::ALIGNMENT_CELLBASE);
                    }
                    if !self.proposal_transactions.is_empty() {
                        vtable.add_field(Self::VT_PROPOSAL_TRANSACTIONS, Self::SIZE_PROPOSAL_TRANSACTIONS, Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if let Some(f) = self.header {
                    builder.align(Self::ALIGNMENT_HEADER);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_HEADER);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if let Some(f) = self.cellbase {
                    builder.align(Self::ALIGNMENT_CELLBASE);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_CELLBASE);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }
                if !self.proposal_transactions.is_empty() {
                    builder.align(Self::ALIGNMENT_PROPOSAL_TRANSACTIONS);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_PROPOSAL_TRANSACTIONS);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(ScalarVectorComponent::new(self.proposal_transactions, 1)),
                    ));
                }

                table_start
            }
        }

        #[derive(Default, Clone, Debug, PartialEq)]
        pub struct ValidTransaction {
            pub cycles: u64,
            pub transaction: Option<Transaction>,
        }

        impl ValidTransaction {
            const VT_CYCLES: usize = 4;
            const SIZE_CYCLES: usize = 8;
            const ALIGNMENT_CYCLES: usize = 8;
            const VT_TRANSACTION: usize = 6;
            const SIZE_TRANSACTION: usize = 4;
            const ALIGNMENT_TRANSACTION: usize = 4;
            const ALIGNMENT: usize = 8;
        }

        impl<'c> Component<'c> for ValidTransaction {
            fn build(self: Box<Self>, builder: &mut Builder<'c>) -> usize {
                let vtable_start = {
                    let mut vtable = builder.start_vtable();
                    if self.cycles != 0u64 {
                        vtable.add_field(Self::VT_CYCLES, Self::SIZE_CYCLES, Self::ALIGNMENT_CYCLES);
                    }
                    if self.transaction.is_some() {
                        vtable.add_field(Self::VT_TRANSACTION, Self::SIZE_TRANSACTION, Self::ALIGNMENT_TRANSACTION);
                    }
                    vtable.finish()
                };

                builder.align_after(SIZE_OF_SOFFSET, Self::ALIGNMENT);

                let table_start = builder.tell();
                builder.push_scalar((table_start - vtable_start) as SOffset);
                if self.cycles != 0u64 {
                    builder.align(Self::ALIGNMENT_CYCLES);
                    builder.push_scalar(self.cycles);
                }
                if let Some(f) = self.transaction {
                    builder.align(Self::ALIGNMENT_TRANSACTION);
                    let offset_position = builder.tell();
                    builder.pad(Self::SIZE_TRANSACTION);
                    builder.push_component(DesignatedComponent::new(
                        offset_position,
                        Box::new(f),
                    ));
                }

                table_start
            }
        }
    }

}
