#![allow(unused)]
//! Test helper impls for generating bodies
use reth_db::{database::Database, tables, transaction::DbTxMut, DatabaseEnv};
use reth_interfaces::{db, p2p::bodies::response::BlockResponse};
use reth_primitives::{Block, BlockBody, SealedBlock, SealedHeader, B256};
use std::collections::HashMap;

pub(crate) fn zip_blocks<'a>(
    headers: impl Iterator<Item = &'a SealedHeader>,
    bodies: &mut HashMap<B256, BlockBody>,
) -> Vec<BlockResponse> {
    headers
        .into_iter()
        .map(|header| {
            let body = bodies.remove(&header.hash()).expect("body exists");
            if header.is_empty() {
                BlockResponse::Empty(header.clone())
            } else {
                BlockResponse::Full(SealedBlock {
                    header: header.clone(),
                    body: body.transactions,
                    ommers: body.ommers,
                    withdrawals: body.withdrawals,
                })
            }
        })
        .collect()
}

pub(crate) fn create_raw_bodies<'a>(
    headers: impl Iterator<Item = &'a SealedHeader>,
    bodies: &mut HashMap<B256, BlockBody>,
) -> Vec<Block> {
    headers
        .into_iter()
        .map(|header| {
            let body = bodies.remove(&header.hash()).expect("body exists");
            body.create_block(header.as_ref().clone())
        })
        .collect()
}

#[inline]
pub(crate) fn insert_headers(db: &DatabaseEnv, headers: &[SealedHeader]) {
    db.update(|tx| -> Result<(), db::DatabaseError> {
        for header in headers {
            tx.put::<tables::CanonicalHeaders>(header.number, header.hash())?;
            tx.put::<tables::Headers>(header.number, header.clone().unseal())?;
        }
        Ok(())
    })
    .expect("failed to commit")
    .expect("failed to insert headers");
}
