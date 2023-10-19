use reth_primitives::{Account, StorageEntry, B256};

/// Default implementation of the hashed state cursor traits.
mod default;

/// Implementation of hashed state cursor traits for the post state.
mod post_state;
pub use post_state::*;

/// The factory trait for creating cursors over the hashed state.
pub trait HashedCursorFactory {
    /// The hashed account cursor type.
    type AccountCursor: HashedAccountCursor;
    /// The hashed storage cursor type.
    type StorageCursor: HashedStorageCursor;

    /// Returns a cursor for iterating over all hashed accounts in the state.
    fn hashed_account_cursor(&self) -> Result<Self::AccountCursor, reth_db::DatabaseError>;

    /// Returns a cursor for iterating over all hashed storage entries in the state.
    fn hashed_storage_cursor(&self) -> Result<Self::StorageCursor, reth_db::DatabaseError>;
}

/// The cursor for iterating over hashed accounts.
pub trait HashedAccountCursor {
    /// Seek an entry greater or equal to the given key and position the cursor there.
    fn seek(&mut self, key: B256) -> Result<Option<(B256, Account)>, reth_db::DatabaseError>;

    /// Move the cursor to the next entry and return it.
    fn next(&mut self) -> Result<Option<(B256, Account)>, reth_db::DatabaseError>;
}

/// The cursor for iterating over hashed storage entries.
pub trait HashedStorageCursor {
    /// Returns `true` if there are no entries for a given key.
    fn is_storage_empty(&mut self, key: B256) -> Result<bool, reth_db::DatabaseError>;

    /// Seek an entry greater or equal to the given key/subkey and position the cursor there.
    fn seek(
        &mut self,
        key: B256,
        subkey: B256,
    ) -> Result<Option<StorageEntry>, reth_db::DatabaseError>;

    /// Move the cursor to the next entry and return it.
    fn next(&mut self) -> Result<Option<StorageEntry>, reth_db::DatabaseError>;
}
