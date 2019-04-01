pub mod common;

use common::ckb_generated::ckb::protocol as g;
use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, WIPOffset};
use flatbuffers_verifier::{get_root, Error};
use proptest::prelude::*;
use std::mem;
use std::panic;

const SOME_PROB: f64 = 0.9;

trait Create<'b> {
    type Target: 'b;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<Self::Target>;
}

impl<'b, T> Create<'b> for Vec<T>
where
    T: Create<'b>,
{
    type Target = flatbuffers::Vector<'b, ForwardsUOffset<T::Target>>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<Self::Target> {
        let offsets: Vec<_> = self.into_iter().map(|e| e.create(builder)).collect();
        builder.create_vector(offsets.as_slice())
    }
}

fn arb_h256() -> impl Strategy<Value = g::H256> {
    prop::array::uniform32(any::<u8>()).prop_map(|v| unsafe { mem::transmute(v) })
}

fn arb_proposal_short_id() -> impl Strategy<Value = g::ProposalShortId> {
    prop::array::uniform10(any::<u8>()).prop_map(|v| unsafe { mem::transmute(v) })
}

#[derive(Clone, Debug)]
struct Bytes {
    seq: Option<Vec<u8>>,
}

fn arb_bytes() -> impl Strategy<Value = Bytes> {
    prop::option::weighted(SOME_PROB, prop::collection::vec(any::<u8>(), 0..2048))
        .prop_map(|seq| Bytes { seq })
}

impl<'b> Create<'b> for Bytes {
    type Target = g::Bytes<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::Bytes<'b>> {
        let seq = self.seq.map(|v| builder.create_vector(v.as_slice()));
        g::Bytes::create(builder, &g::BytesArgs { seq })
    }
}

#[derive(Clone, Debug)]
struct Header {
    version: u32,
    parent_hash: Option<g::H256>,
    timestamp: u64,
    number: u64,
    txs_commit: Option<g::H256>,
    txs_proposal: Option<g::H256>,
    difficulty: Option<Bytes>,
    nonce: u64,
    proof: Option<Bytes>,
    cellbase_id: Option<g::H256>,
    uncles_hash: Option<g::H256>,
    uncles_count: u32,
}

fn arb_header() -> impl Strategy<Value = Header> {
    (
        any::<u32>(),
        any::<u64>(),
        any::<u64>(),
        prop::option::weighted(SOME_PROB, arb_bytes()),
        any::<u64>(),
        prop::option::weighted(SOME_PROB, arb_bytes()),
        any::<u32>(),
        prop::array::uniform5(prop::option::weighted(SOME_PROB, arb_h256())),
    )
        .prop_map(
            |(version, timestamp, number, difficulty, nonce, proof, uncles_count, hashes)| Header {
                version,
                timestamp,
                number,
                difficulty,
                nonce,
                proof,
                uncles_count,
                parent_hash: hashes[0],
                txs_commit: hashes[1],
                txs_proposal: hashes[2],
                cellbase_id: hashes[3],
                uncles_hash: hashes[4],
            },
        )
}

impl<'b> Create<'b> for Header {
    type Target = g::Header<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::Header<'b>> {
        let difficulty = self.difficulty.map(|v| v.create(builder));
        let proof = self.proof.map(|v| v.create(builder));

        g::Header::create(
            builder,
            &g::HeaderArgs {
                difficulty,
                proof,
                version: self.version,
                parent_hash: self.parent_hash.as_ref(),
                timestamp: self.timestamp,
                number: self.number,
                txs_commit: self.txs_commit.as_ref(),
                txs_proposal: self.txs_proposal.as_ref(),
                nonce: self.nonce,
                cellbase_id: self.cellbase_id.as_ref(),
                uncles_hash: self.uncles_hash.as_ref(),
                uncles_count: self.uncles_count,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct UncleBlock {
    header: Option<Header>,
    cellbase: Option<Transaction>,
    proposal_transactions: Option<Vec<g::ProposalShortId>>,
}

fn arb_uncle_block() -> impl Strategy<Value = UncleBlock> {
    (
        prop::option::weighted(SOME_PROB, arb_header()),
        prop::option::weighted(SOME_PROB, arb_transaction()),
        prop::option::weighted(
            SOME_PROB,
            prop::collection::vec(arb_proposal_short_id(), 0..8),
        ),
    )
        .prop_map(|(header, cellbase, proposal_transactions)| UncleBlock {
            header,
            cellbase,
            proposal_transactions,
        })
}

impl<'b> Create<'b> for UncleBlock {
    type Target = g::UncleBlock<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::UncleBlock<'b>> {
        let header = self.header.map(|v| v.create(builder));
        let cellbase = self.cellbase.map(|v| v.create(builder));
        let proposal_transactions = self
            .proposal_transactions
            .map(|v| builder.create_vector(v.as_slice()));

        g::UncleBlock::create(
            builder,
            &g::UncleBlockArgs {
                header,
                cellbase,
                proposal_transactions,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct Transaction {
    version: u32,
    deps: Option<Vec<OutPoint>>,
    inputs: Option<Vec<CellInput>>,
    outputs: Option<Vec<CellOutput>>,
    embeds: Option<Vec<Bytes>>,
}

fn arb_transaction() -> impl Strategy<Value = Transaction> {
    (
        any::<u32>(),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_out_point(), 0..8)),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_cell_input(), 0..8)),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_cell_output(), 0..8)),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_bytes(), 0..8)),
    )
        .prop_map(|(version, deps, inputs, outputs, embeds)| Transaction {
            version,
            deps,
            inputs,
            outputs,
            embeds,
        })
}

impl<'b> Create<'b> for Transaction {
    type Target = g::Transaction<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::Transaction<'b>> {
        let deps = self.deps.map(|v| v.create(builder));
        let inputs = self.inputs.map(|v| v.create(builder));
        let outputs = self.outputs.map(|v| v.create(builder));
        let embeds = self.embeds.map(|v| v.create(builder));

        g::Transaction::create(
            builder,
            &g::TransactionArgs {
                deps,
                inputs,
                outputs,
                embeds,
                version: self.version,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct OutPoint {
    hash: Option<g::H256>,
    index: u32,
}

fn arb_out_point() -> impl Strategy<Value = OutPoint> {
    (prop::option::weighted(SOME_PROB, arb_h256()), any::<u32>())
        .prop_map(|(hash, index)| OutPoint { hash, index })
}

impl<'b> Create<'b> for OutPoint {
    type Target = g::OutPoint<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::OutPoint<'b>> {
        g::OutPoint::create(
            builder,
            &g::OutPointArgs {
                hash: self.hash.as_ref(),
                index: self.index,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct CellInput {
    hash: Option<g::H256>,
    index: u32,
    args: Option<Vec<Bytes>>,
}

fn arb_cell_input() -> impl Strategy<Value = CellInput> {
    (
        prop::option::weighted(SOME_PROB, arb_h256()),
        any::<u32>(),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_bytes(), 0..4)),
    )
        .prop_map(|(hash, index, args)| CellInput { hash, index, args })
}

impl<'b> Create<'b> for CellInput {
    type Target = g::CellInput<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::CellInput<'b>> {
        let args = self.args.map(|v| v.create(builder));

        g::CellInput::create(
            builder,
            &g::CellInputArgs {
                hash: self.hash.as_ref(),
                index: self.index,
                args,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct CellOutput {
    capacity: u64,
    data: Option<Bytes>,
    lock: Option<Script>,
    type_: Option<Script>,
}

fn arb_cell_output() -> impl Strategy<Value = CellOutput> {
    (
        any::<u64>(),
        prop::option::weighted(SOME_PROB, arb_bytes()),
        prop::option::weighted(SOME_PROB, arb_script()),
        prop::option::weighted(SOME_PROB, arb_script()),
    )
        .prop_map(|(capacity, data, lock, type_)| CellOutput {
            capacity,
            data,
            lock,
            type_,
        })
}

impl<'b> Create<'b> for CellOutput {
    type Target = g::CellOutput<'b>;
    fn create(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::CellOutput<'b>> {
        let data = self.data.map(|v| v.create(builder));
        let lock = self.lock.map(|v| v.create(builder));
        let type_ = self.type_.map(|v| v.create(builder));

        g::CellOutput::create(
            builder,
            &g::CellOutputArgs {
                capacity: self.capacity,
                data,
                lock,
                type_,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct Script {
    version: u8,
    args: Option<Vec<Bytes>>,
    binary_hash: Option<g::H256>,
}

fn arb_script() -> impl Strategy<Value = Script> {
    (
        any::<u8>(),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_bytes(), 0..4)),
        prop::option::weighted(SOME_PROB, arb_h256()),
    )
        .prop_map(|(version, args, binary_hash)| Script {
            version,
            args,
            binary_hash,
        })
}

impl Script {
    fn create<'b>(self, builder: &mut FlatBufferBuilder<'b>) -> WIPOffset<g::Script<'b>> {
        let args = self.args.map(|v| v.create(builder));

        g::Script::create(
            builder,
            &g::ScriptArgs {
                args,
                version: self.version,
                binary_hash: self.binary_hash.map(Into::into).as_ref(),
            },
        )
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
enum SyncMessage {
    NONE,
    GetHeaders {
        version: u32,
        block_locator_hashes: Option<Vec<g::H256>>,
        hash_stop: Option<g::H256>,
    },
    Headers {
        headers: Option<Vec<Header>>,
    },
    GetBlocks {
        block_hashes: Option<Vec<g::H256>>,
    },
    Block {
        header: Option<Header>,
        uncles: Option<Vec<UncleBlock>>,
        commit_transactions: Option<Vec<Transaction>>,
        proposal_transactions: Option<Vec<g::ProposalShortId>>,
    },
}

fn arb_sync_message() -> impl Strategy<Value = SyncMessage> {
    let arb_get_headers = (
        any::<u32>(),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_h256(), 0..4)),
        prop::option::weighted(SOME_PROB, arb_h256()),
    )
        .prop_map(
            |(version, block_locator_hashes, hash_stop)| SyncMessage::GetHeaders {
                version,
                block_locator_hashes,
                hash_stop,
            },
        );

    let arb_block = (
        prop::option::weighted(SOME_PROB, arb_header()),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_uncle_block(), 0..4)),
        prop::option::weighted(SOME_PROB, prop::collection::vec(arb_transaction(), 0..16)),
        prop::option::weighted(
            SOME_PROB,
            prop::collection::vec(arb_proposal_short_id(), 0..16),
        ),
    )
        .prop_map(
            |(header, uncles, commit_transactions, proposal_transactions)| SyncMessage::Block {
                header,
                uncles,
                commit_transactions,
                proposal_transactions,
            },
        );

    prop_oneof![
        1 => Just(SyncMessage::NONE),
        10 => arb_get_headers,
        10 => prop::option::weighted(SOME_PROB, prop::collection::vec(arb_header(), 0..8))
            .prop_map(|headers| SyncMessage::Headers { headers }),
        10 => prop::option::weighted(SOME_PROB, prop::collection::vec(arb_h256(), 0..8))
            .prop_map(|block_hashes| SyncMessage::GetBlocks { block_hashes }),
        10 => arb_block,
    ]
}

impl SyncMessage {
    fn create(self) -> (Vec<u8>, usize) {
        let mut builder = FlatBufferBuilder::new();

        let (payload_type, payload) = match self {
            SyncMessage::GetHeaders {
                version,
                block_locator_hashes,
                hash_stop,
            } => {
                let block_locator_hashes = block_locator_hashes.map(|v| {
                    let vec: &[g::H256] = unsafe { mem::transmute(v.as_slice()) };
                    builder.create_vector(vec)
                });
                (
                    g::SyncPayload::GetHeaders,
                    Some(
                        g::GetHeaders::create(
                            &mut builder,
                            &g::GetHeadersArgs {
                                version,
                                block_locator_hashes,
                                hash_stop: hash_stop.map(Into::into).as_ref(),
                            },
                        )
                        .as_union_value(),
                    ),
                )
            }
            SyncMessage::Headers { headers } => {
                let headers = headers.map(|v| v.create(&mut builder));

                (
                    g::SyncPayload::Headers,
                    Some(
                        g::Headers::create(&mut builder, &g::HeadersArgs { headers })
                            .as_union_value(),
                    ),
                )
            }
            SyncMessage::GetBlocks { block_hashes } => {
                let block_hashes = block_hashes.map(|v| {
                    let vec: &[g::H256] = unsafe { mem::transmute(v.as_slice()) };
                    builder.create_vector(vec)
                });

                (
                    g::SyncPayload::GetBlocks,
                    Some(
                        g::GetBlocks::create(&mut builder, &g::GetBlocksArgs { block_hashes })
                            .as_union_value(),
                    ),
                )
            }
            SyncMessage::Block {
                header,
                uncles,
                commit_transactions: _ct,
                proposal_transactions: _pt,
            } => {
                let header = header.map(|v| v.create(&mut builder));
                let uncles = uncles.map(|v| v.create(&mut builder));

                (
                    g::SyncPayload::Block,
                    Some(
                        g::Block::create(
                            &mut builder,
                            &g::BlockArgs {
                                header,
                                uncles,
                                commit_transactions: None,
                                proposal_transactions: None,
                            },
                        )
                        .as_union_value(),
                    ),
                )
            }

            _ => (g::SyncPayload::NONE, None),
        };

        dbg!(payload_type);

        let wip_offset = g::SyncMessage::create(
            &mut builder,
            &g::SyncMessageArgs {
                payload_type,
                payload,
            },
        );
        builder.finish_minimal(wip_offset);

        builder.collapse()
    }
}

fn walk(buf: &[u8]) -> Result<(), Error> {
    match panic::catch_unwind(|| walk_inner(buf)) {
        Ok(r) => r,
        Err(_) => Err(Error::OutOfBounds),
    }
}

fn walk_header(header: g::Header) {
    dbg!(header.version());
    header.parent_hash();
    header.timestamp();
    header.number();
    header.txs_commit();
    header.txs_proposal();
    header.difficulty();
    header.nonce();
    header.proof();
    header.cellbase_id();
    header.uncles_hash();
    header.uncles_count();
}

fn walk_uncle_block(uncle_block: g::UncleBlock) {
    if let Some(header) = uncle_block.header() {
        walk_header(header);
    }
    if let Some(cellbase) = uncle_block.cellbase() {
        walk_transaction(cellbase);
    }
    if let Some(proposal_transactions) = uncle_block.proposal_transactions() {
        for i in 0..dbg!(proposal_transactions.len()) {
            let _ = proposal_transactions.get(i);
        }
    }
}

fn walk_bytes(bytes: g::Bytes) {
    if let Some(seq) = bytes.seq() {
        seq.to_vec();
    }
}

fn walk_out_point(out_point: g::OutPoint) {
    out_point.hash();
    out_point.index();
}

fn walk_script(script: g::Script) {
    script.version();
    if let Some(args) = script.args() {
        for i in 0..dbg!(args.len()) {
            walk_bytes(args.get(i));
        }
    }
    script.binary_hash();
}

fn walk_transaction(transaction: g::Transaction) {
    dbg!(transaction.version());
    if let Some(deps) = transaction.deps() {
        for i in 0..dbg!(deps.len()) {
            walk_out_point(deps.get(i));
        }
    }
    if let Some(inputs) = transaction.inputs() {
        for i in 0..dbg!(inputs.len()) {
            let input = inputs.get(i);
            input.hash();
            input.index();
            if let Some(args) = input.args() {
                for i in 0..dbg!(args.len()) {
                    walk_bytes(args.get(i));
                }
            }
        }
    }
    if let Some(outputs) = transaction.outputs() {
        for i in 0..dbg!(outputs.len()) {
            let output = outputs.get(i);
            output.capacity();
            if let Some(data) = output.data() {
                walk_bytes(data);
            }
            if let Some(lock) = output.lock() {
                walk_script(lock);
            }
            if let Some(type_) = output.type_() {
                walk_script(type_);
            }
        }
    }
    if let Some(embeds) = transaction.embeds() {
        for i in 0..dbg!(embeds.len()) {
            let embed = embeds.get(i);
            walk_bytes(embed);
        }
    }
}

fn walk_proof(proof: g::MerkleProof) {
    if let Some(indices) = proof.indices() {
        for i in 0..dbg!(indices.len()) {
            let _ = indices.get(i);
        }
    }
    if let Some(lemmas) = proof.lemmas() {
        for i in 0..dbg!(lemmas.len()) {
            let _ = lemmas.get(i);
        }
    }
}

fn walk_inner(buf: &[u8]) -> Result<(), Error> {
    let sync_message = flatbuffers::get_root::<g::SyncMessage>(buf);
    match sync_message.payload_type() {
        g::SyncPayload::NONE => assert!(sync_message.payload().is_none()),
        g::SyncPayload::GetHeaders => {
            let m = sync_message
                .payload_as_get_headers()
                .ok_or(Error::UnmatchedUnion)?;
            dbg!(m.version());
            m.block_locator_hashes();
            m.hash_stop();
        }
        g::SyncPayload::Headers => {
            let m = sync_message
                .payload_as_headers()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(headers) = m.headers() {
                for i in 0..dbg!(headers.len()) {
                    walk_header(headers.get(i));
                }
            }
        }
        g::SyncPayload::GetBlocks => {
            let m = sync_message
                .payload_as_get_blocks()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(hashes) = m.block_hashes() {
                hashes.to_vec();
            }
        }
        g::SyncPayload::Block => {
            let m = sync_message
                .payload_as_block()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(header) = m.header() {
                walk_header(header);
            }
            if let Some(uncles) = m.uncles() {
                for i in 0..dbg!(uncles.len()) {
                    walk_uncle_block(uncles.get(i));
                }
            }
            if let Some(commit_transactions) = m.commit_transactions() {
                for i in 0..dbg!(commit_transactions.len()) {
                    walk_transaction(commit_transactions.get(i));
                }
            }
            if let Some(proposal_transactions) = m.proposal_transactions() {
                proposal_transactions.to_vec();
            }
        }
        g::SyncPayload::SetFilter => {
            let m = sync_message
                .payload_as_set_filter()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(filter) = m.filter() {
                filter.to_vec();
            }
            m.num_hashes();
            m.hash_seed();
        }
        g::SyncPayload::AddFilter => {
            let m = sync_message
                .payload_as_add_filter()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(filter) = m.filter() {
                filter.to_vec();
            }
        }
        g::SyncPayload::ClearFilter => {
            sync_message
                .payload_as_add_filter()
                .ok_or(Error::UnmatchedUnion)?;
        }
        g::SyncPayload::FilteredBlock => {
            let m = sync_message
                .payload_as_filtered_block()
                .ok_or(Error::UnmatchedUnion)?;
            if let Some(header) = m.header() {
                walk_header(header);
            }
            if let Some(transactions) = m.transactions() {
                for i in 0..dbg!(transactions.len()) {
                    walk_transaction(transactions.get(i));
                }
            }
            if let Some(proof) = m.proof() {
                walk_proof(proof);
            }
        }
    }

    Ok(())
}

proptest! {
    #[test]
    fn proptest_verifier_positive_case(sync_message in arb_sync_message()) {
        let (buf, loc) = sync_message.clone().create();
        let root = get_root::<g::SyncMessage>(&buf[loc..]);
        if root.is_err() {
            dbg!(root.as_ref().err());
            dbg!(common::hex(&buf[loc..]));
        }
        assert!(root.is_ok());
    }

    #[test]
    fn proptest_verifier_negative_case(buf in prop::collection::vec(any::<u8>(), 4..4096)) {
        let result = get_root::<g::SyncMessage>(&buf[..]);

        match walk(&buf[..]) {
            Ok(()) => assert!(match result {
                Ok(_) => true,
                Err(Error::NonNullTerminatedString) => true,
                _ => false,
            }),
            Err(err) => assert_eq!(result.err(), Some(err)),
        }
    }
}
