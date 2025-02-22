//! This module implements traits for the contract interface.
//! This allows setting-up mock objects for testing individual
//! contract invocations.

#[cfg(not(feature = "std"))]
use crate::vec::Vec;
use crate::{
    types::{LogError, StateError},
    AccountSignatures, CallContractResult, CheckAccountSignatureResult, EntryRaw, ExchangeRates,
    HashKeccak256, HashSha2256, HashSha3256, Key, OccupiedEntryRaw, PublicKeyEcdsaSecp256k1,
    PublicKeyEd25519, QueryAccountBalanceResult, QueryAccountPublicKeysResult,
    QueryContractBalanceResult, ReadOnlyCallContractResult, SignatureEcdsaSecp256k1,
    SignatureEd25519, StateBuilder, TransferResult, UpgradeResult, VacantEntryRaw,
};
use concordium_contracts_common::*;

/// Objects which can access parameters to contracts.
///
/// This trait has a Read supertrait which means that structured parameters can
/// be directly deserialized by using `.get()` function from the `Get` trait.
///
/// The reuse of `Read` methods is the reason for the slightly strange choice of
/// methods of this trait.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`ExternParameter`](crate::types::ExternParameter) instead unless you
/// intend to use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasParameter: Read + Seek + HasSize {}

/// Objects which can access call responses from contract invocations.
///
/// This trait has a Read supertrait which means that structured call responses
/// can be directly deserialized by using `.get()` function from the `Get`
/// trait.
///
/// The reuse of `Read` methods is the reason for the slightly strange choice of
/// methods of this trait.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`CallResponse`](crate::types::CallResponse) instead unless you intend
/// to use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasCallResponse: Read {
    /// Get the size of the call response to the contract invocation.
    fn size(&self) -> u32;
}

/// Objects which can access chain metadata.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`ChainMetadata`] instead unless you intend to use
/// the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasChainMetadata {
    /// Get time in milliseconds at the beginning of this block.
    fn slot_time(&self) -> SlotTime;
    /// Get time in milliseconds at the beginning of this block.
    #[inline(always)]
    fn block_time(&self) -> Timestamp { self.slot_time() }
}

/// A type which has access to a policy of a credential.
/// Since policies can be large this is deliberately written in a relatively
/// low-level style to enable efficient traversal of all the attributes without
/// any allocations.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`Policy<AttributeCursor>`] instead unless you intend to use the
/// deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasPolicy: Sized {
    type Iterator: Iterator<Item = (AttributeTag, AttributeValue)>;
    /// Identity provider who signed the identity object the credential is
    /// derived from.
    fn identity_provider(&self) -> IdentityProvider;
    /// Beginning of the month in milliseconds since unix epoch when the
    /// credential was created.
    fn created_at(&self) -> Timestamp;
    /// Beginning of the month where the credential is no longer valid, in
    /// milliseconds since unix epoch.
    fn valid_to(&self) -> Timestamp;
    /// Get the next attribute, storing it in the provided buffer.
    /// The return value, if `Some`, is a pair of an attribute tag, and the
    /// length, `n` of the attribute value. In this case, the attribute
    /// value is written in the first `n` bytes of the provided buffer. The
    /// rest of the buffer is unchanged.
    ///
    /// The reason this function is added here, and we don't simply implement
    /// an Iterator for this type is that with the supplied buffer we can
    /// iterate through the elements more efficiently, without any allocations,
    /// the consumer being responsible for allocating the buffer.
    fn next_item(&mut self, buf: &mut [u8; 31]) -> Option<(AttributeTag, u8)>;
    /// Get an iterator over all the attributes of the policy.
    fn attributes(&self) -> Self::Iterator;
}

/// Common data accessible to both init and receive methods.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`InitContext`](crate::types::InitContext) or
/// [`ReceiveContext`](crate::types::ReceiveContext) instead unless you intend
/// to use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasCommonData {
    type PolicyType: HasPolicy;
    type MetadataType: HasChainMetadata;
    type ParamType: HasParameter + Read;
    type PolicyIteratorType: ExactSizeIterator<Item = Self::PolicyType>;
    /// Policies of the sender of the message.
    /// For init methods this is the would-be creator of the contract,
    /// for the receive this is the policies of the immediate sender.
    ///
    /// In the latter case, if the sender is an account then it is the policies
    /// of the account, if it is a contract then it is the policies of the
    /// creator of the contract.
    fn policies(&self) -> Self::PolicyIteratorType;
    /// Get the reference to chain metadata
    fn metadata(&self) -> &Self::MetadataType;
    /// Get the cursor to the parameter.
    fn parameter_cursor(&self) -> Self::ParamType;
}

/// Types which can act as init contexts.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`InitContext`](crate::types::InitContext) instead unless you intend to
/// use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasInitContext<Error: Default = ()>: HasCommonData {
    /// Data needed to open the context.
    type InitData;
    /// Open the init context for reading and accessing values.
    fn open(data: Self::InitData) -> Self;
    /// Who invoked this init call.
    fn init_origin(&self) -> AccountAddress;
}

/// Types which can act as receive contexts.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`ReceiveContext`](crate::types::ReceiveContext) instead unless you
/// intend to use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasReceiveContext<Error: Default = ()>: HasCommonData {
    type ReceiveData;

    /// Open the receive context for reading and accessing values.
    fn open(data: Self::ReceiveData) -> Self;
    /// Who is the account that initiated the top-level transaction this
    /// invocation is a part of.
    fn invoker(&self) -> AccountAddress;
    /// The address of the contract being invoked.
    fn self_address(&self) -> ContractAddress;
    /// The immediate sender of the message. In general different from the
    /// invoker.
    fn sender(&self) -> Address;
    /// Account which created the contract instance.
    fn owner(&self) -> AccountAddress;
    /// Get the name of the entrypoint that was named. In case a default
    /// entrypoint is invoked this can be different from the name of the
    /// entrypoint that is being executed.
    fn named_entrypoint(&self) -> OwnedEntrypointName;
}

/// A type that can serve as the contract state entry type.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`StateEntry`](crate::types::StateEntry) instead unless you intend to
/// use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasStateEntry
where
    Self: Read,
    Self: Write<Err = Self::Error>,
    Self: Seek<Err = Self::Error>, {
    type StateEntryData;
    type StateEntryKey;
    type Error: Default;

    /// Set the cursor to the beginning. Equivalent to
    /// `.seek(SeekFrom::Start(0))` but can be implemented more efficiently.
    fn move_to_start(&mut self);

    /// Get the current size of the entry.
    /// Returns an error if the entry does not exist.
    fn size(&self) -> Result<u32, Self::Error>;

    /// Truncate the entry to the given size. If the given size is more than the
    /// current size this operation does nothing. The new position is at
    /// most at the end of the stream.
    /// Returns an error if the entry does not exist.
    fn truncate(&mut self, new_size: u32) -> Result<(), Self::Error>;

    /// Make sure that the memory size is at least that many bytes in size.
    /// Returns `Ok` iff this was successful. The new bytes are initialized as
    /// 0.
    /// Returns an error if the entry does not exist.
    fn reserve(&mut self, len: u32) -> Result<(), Self::Error> {
        if self.size()? < len {
            self.resize(len)
        } else {
            Ok(())
        }
    }

    /// Get a reference to the entry's key.
    fn get_key(&self) -> &[u8];

    /// Resize the entry to the given size.
    /// Returns an error if the entry does not exist.
    fn resize(&mut self, new_size: u32) -> Result<(), Self::Error>;
}

/// Types which can serve as the contract state.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`StateApi`](crate::types::StateApi) instead unless you intend to use
/// the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasStateApi: Clone {
    type EntryType: HasStateEntry;
    type IterType: Iterator<Item = Self::EntryType>;

    /// Create a new entry in the state. If an entry with the given key already
    /// exists then it is reset to an empty entry. If the part of the tree
    /// where the key points to is locked due to an acquired iterator
    /// then no entry is created, and an error will be returned.
    fn create_entry(&mut self, key: &[u8]) -> Result<Self::EntryType, StateError>;

    /// Lookup an entry in the state.
    fn lookup_entry(&self, key: &[u8]) -> Option<Self::EntryType>;

    /// Like [`lookup_entry`](Self::lookup_entry) except that it consumes the
    /// key and returns an [`EntryRaw`] instead of an optional entry.
    ///
    /// For maximal flexibility the function is parametrized by the type of key
    /// with the intention that needless allocations are avoided for short
    /// keys and existing entries. The most common examples of keys will be [`Vec<u8>`](Vec) or fixed sized arrays [`[u8; N]`](https://doc.rust-lang.org/std/primitive.array.html).
    fn entry<K: AsRef<[u8]> + Into<Key>>(&mut self, key: K) -> EntryRaw<Self> {
        match self.lookup_entry(key.as_ref()) {
            Some(e) => EntryRaw::Occupied(OccupiedEntryRaw::new(e)),
            None => EntryRaw::Vacant(VacantEntryRaw::new(key.into(), self.clone())),
        }
    }

    /// Delete an entry.
    /// Returns an error if the entry did not exist, or if it is part of a
    /// locked subtree.
    fn delete_entry(&mut self, key: Self::EntryType) -> Result<(), StateError>;

    /// Delete the entire subtree.
    /// Returns whether any values were deleted, or an error if the given prefix
    /// is part of a locked subtree.
    fn delete_prefix(&mut self, prefix: &[u8]) -> Result<bool, StateError>;

    /// Get an iterator over a map in the state.
    ///
    /// An iterator locks the subtree with the given prefix.
    /// Locking means that the structure of the tree cannot be altered, i.e.,
    /// that entries cannot be created or deleted. Altering the data inside
    /// an entry of a locked subtree is, however, allowed.
    ///
    /// Deleting iterators with
    /// [`delete_iterator`][HasStateApi::delete_iterator] is necessary to
    /// unlock the subtree. To unlock a subtree with prefix
    /// `p1`, all iterators with prefixes `p2`, where `p2` is a prefix of `p1`,
    /// must be deleted.
    ///
    /// Returns an error if the number of active iterators for the same prefix
    /// exceeds [u32::MAX].
    fn iterator(&self, prefix: &[u8]) -> Result<Self::IterType, StateError>;

    /// Delete an iterator.
    /// See the [`iterator`][HasStateApi::iterator] method for why this is
    /// necessary.
    fn delete_iterator(&mut self, iter: Self::IterType);

    /// Read and deserialize the state stored at the root of the state trie.
    /// If such a state does not exist, or cannot be deserialized into the
    /// provided type, then this returns an error.
    fn read_root<A: DeserialWithState<Self>>(&self) -> ParseResult<A> {
        A::deserial_with_state(self, &mut self.lookup_entry(&[]).ok_or(ParseError {})?)
    }

    /// Serialize and write the state at the root of the state trie. Dual to
    /// [`read_root`](Self::read_root).
    fn write_root<A: Serial>(&mut self, new_state: &A) {
        new_state.serial(&mut self.create_entry(&[]).unwrap_abort()).unwrap_abort()
    }
}

/// A type that can serve as the host, meaning that it supports interactions
/// with the chain, such as querying balance of the contract, accessing its
/// state, and invoking operations on other contracts and accounts.
///
/// The trait is parameterized by the `State` type. This is the type of the
/// contract state that the particular contract operates on.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`Host`](crate::types::Host) instead unless you intend to use
/// the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasHost<State>: Sized {
    /// The type of low-level state that is associated with the host.
    /// This provides access to low-level state operations.
    type StateApiType: HasStateApi;

    /// The type of return values this host provides. This is the raw return
    /// value. The intention is that it will be deserialized by the
    /// consumer, via the [Read] implementation.
    ///
    /// The Debug requirement exists so that consumers of this trait may use
    /// methods like [ExpectReport::expect_report].
    type ReturnValueType: HasCallResponse + crate::fmt::Debug;

    /// Perform a transfer to the given account if the contract has sufficient
    /// balance.
    fn invoke_transfer(&self, receiver: &AccountAddress, amount: Amount) -> TransferResult;

    /// Invoke a given method of a contract with the amount and parameter
    /// provided. If invocation succeeds then the return value is a pair of
    /// a boolean which indicates whether the state of the contract has changed
    /// or not, and a possible return value. The return value is present if and
    /// only if a V1 contract was invoked.
    fn invoke_contract_raw(
        &mut self,
        to: &ContractAddress,
        parameter: Parameter,
        method: EntrypointName,
        amount: Amount,
    ) -> CallContractResult<Self::ReturnValueType>;

    /// Like [`invoke_contract_raw`](Self::invoke_contract_raw), except that the
    /// parameter is automatically serialized. If the parameter already
    /// implements [`AsRef<[u8]>`](AsRef) or can be equivalently cheaply
    /// converted to a byte array, then
    /// [`invoke_contract_raw`](Self::invoke_contract_raw) should be
    /// used, since it avoids intermediate allocations.
    fn invoke_contract<P: Serial>(
        &mut self,
        to: &ContractAddress,
        parameter: &P,
        method: EntrypointName,
        amount: Amount,
    ) -> CallContractResult<Self::ReturnValueType> {
        let param = to_bytes(parameter);
        self.invoke_contract_raw(to, Parameter::new_unchecked(&param), method, amount)
    }

    /// Upgrade the module for this instance to a given module. The new module
    /// must contain a smart contract with a matching name.
    /// Invocations of this instance after the point of a successful upgrade,
    /// will use the new smart contract module. The remaining code after a
    /// successful upgrade in the same invokation is executed as normal.
    ///
    /// This will fail if:
    /// - The supplied module is not deployed.
    /// - The supplied module does not contain a smart contract with a name
    ///   matching this instance.
    /// - The supplied module is a version 0 smart contract module.
    fn upgrade(&mut self, module: ModuleReference) -> UpgradeResult;

    /// Invoke a given method of a contract with the amount and parameter
    /// provided. If invocation succeeds **and the state of the contract
    /// instance is not modified** then the return value is the return value
    /// of the contract that was invoked. The return value is present (i.e.,
    /// not [`None`]) if and only if a V1 contract was invoked. If the
    /// invocation succeeds but the state of the contract is modified then
    /// this method will panic.
    ///
    /// <div class="example-wrap" style="display:inline-block"><pre
    /// class="compile_fail" style="white-space:normal;font:inherit;">
    ///
    /// **Warning**: If the state of the contract was modified **prior**, i.e.,
    /// if a mutable reference obtained via
    /// [`state_mut`](HasHost::state_mut) was used to modify the state then
    /// the state must be manually stored via the
    /// [`commit_state`](HasHost::commit_state) function, otherwise it will not
    /// be seen by the callee.
    ///
    /// </pre></div>
    fn invoke_contract_raw_read_only(
        &self,
        to: &ContractAddress,
        parameter: Parameter<'_>,
        method: EntrypointName<'_>,
        amount: Amount,
    ) -> ReadOnlyCallContractResult<Self::ReturnValueType>;

    /// Like [`invoke_contract_raw_read_only`][0], except that the parameter is
    /// automatically serialized. If the parameter already implements
    /// [`AsRef<[u8]>`](AsRef) or can be equivalently cheaply converted to a
    /// byte array, then [`invoke_contract_raw`](Self::invoke_contract_raw)
    /// should be used, since it avoids intermediate allocations.
    ///
    /// [0]: Self::invoke_contract_raw_read_only
    ///
    /// <div class="example-wrap" style="display:inline-block"><pre
    /// class="compile_fail" style="white-space:normal;font:inherit;">
    ///
    /// **Warning**: If the state of the contract was modified **prior**, i.e.,
    /// if a mutable reference obtained via
    /// [`state_mut`](HasHost::state_mut) was used to modify the state then
    /// the state must be manually stored via the
    /// [`commit_state`](HasHost::commit_state) function, otherwise it will not
    /// be seen by the callee.
    ///
    /// </pre></div>
    fn invoke_contract_read_only<P: Serial>(
        &self,
        to: &ContractAddress,
        parameter: &P,
        method: EntrypointName,
        amount: Amount,
    ) -> ReadOnlyCallContractResult<Self::ReturnValueType> {
        let param = to_bytes(parameter);
        self.invoke_contract_raw_read_only(to, Parameter::new_unchecked(&param), method, amount)
    }

    /// Get the current exchange rates used by the chain. That is a Euro per
    /// Energy rate and micro CCD per Euro rate.
    fn exchange_rates(&self) -> ExchangeRates;

    /// Get the current public balance of an account. Here public means
    /// unencrypted or unshielded. See
    /// [`AccountBalance`](concordium_contracts_common::AccountBalance) for
    /// more. This query will fail if the provided address does not exist on
    /// chain.
    ///
    /// Any amount received by transfers during the transaction
    /// until the point of querying are reflected in this balance.
    ///
    /// ```ignore
    /// let balance_before = host.account_balance(account)?;
    /// host.invoke_transfer(&account, amount)?;
    /// let balance_after = host.account_balance(account)?; // balance_before + amount
    /// ```
    ///
    /// Note: Querying the account invoking this transaction, will return the
    /// current account balance subtracted the amount of CCD reserved for paying
    /// the entire energy limit and the amount sent as part of update
    /// transaction.
    fn account_balance(&self, address: AccountAddress) -> QueryAccountBalanceResult;

    /// Get the current balance of a contract instance.
    ///
    /// Any amount sent and received by transfers and invocations until the
    /// point of querying are reflected in this balance.
    /// This query will fail if the provided address does not exist on chain.
    ///
    /// ```ignore
    /// let balance_before = host.contract_balance(contract_address)?;
    /// host.invoke_contract(&contract_address, ..., amount)?;
    /// let balance_after = host.contract_balance(contract_address)?; // balance_before + amount
    /// ```
    ///
    /// Note: Querying the contract itself returns the balance of the contract
    /// including the amount transferred as part of the invocation.
    fn contract_balance(&self, address: ContractAddress) -> QueryContractBalanceResult;

    /// Get the account's public keys.
    fn account_public_keys(&self, address: AccountAddress) -> QueryAccountPublicKeysResult;

    /// Verify the signature with account's public keys.
    ///
    /// - `address` is the address of the account
    /// - `signatures` is the [`AccountSignatures`] that are to be checked
    /// - `data` is the data that the signatures are on.
    ///
    /// The response is an error if the account is missing, and if the
    /// signatures were correctly parsed then it is a boolean indicating
    /// whether the check succeeded or failed.
    fn check_account_signature(
        &self,
        address: AccountAddress,
        signatures: &AccountSignatures,
        data: &[u8],
    ) -> CheckAccountSignatureResult;

    /// Get an immutable reference to the contract state.
    fn state(&self) -> &State;

    /// Get a mutable reference to the contract state.
    fn state_mut(&mut self) -> &mut State;

    /// Make sure the contract state is fully written out, so that any changes
    /// that were done in-memory up to the point in contract execution are
    /// reflected in the actual contract state maintained by the node.
    fn commit_state(&mut self);

    /// Get the state_builder for the contract state.
    fn state_builder(&mut self) -> &mut StateBuilder<Self::StateApiType>;

    /// Get a mutable reference to both the state and the builder.
    /// This is required due to limitations of the Rust type system, since
    /// otherwise it is not possible to get the reference to the state and
    /// the state builder at the same time. Once some incarnation of "view
    /// types" is stable this will likely be possible to remove, and the
    /// types of `state_builder` and `state_mut` can be refined.
    fn state_and_builder(&mut self) -> (&mut State, &mut StateBuilder<Self::StateApiType>);

    /// Get the contract's own current balance. Upon entry to the entrypoint the
    /// balance that is returned is the sum of balance of the contract at
    /// the time of the invocation and the amount that is being transferred to
    /// the contract.
    fn self_balance(&self) -> Amount;
}

/// A type that can be deleted from the state.
/// For simple types, such as `u8` and `String`, the `delete` methods is a
/// no-op. But for [`StateBox`][crate::StateBox], [`StateMap`][crate::StateMap],
/// and [`StateSet`][crate::StateSet], `delete` makes sure to the delete _all_
/// the necessary data from the state.
pub trait Deletable {
    /// Delete all items that this type owns in the state.
    fn delete(self);
}

/// Objects which can serve as loggers.
///
/// Logging functionality can be used by smart contracts to record events that
/// might be of interest to external parties. These events are not used on the
/// chain, and cannot be observed by other contracts, but they are stored by the
/// node, and can be queried to provide information to off-chain actors.
///
/// In v1 contracts logging is per section of execution (between
/// [`invoke_contract`](HasHost::invoke_contract) or
/// [`invoke_transfer`](HasHost::invoke_transfer) calls. In each section at most
/// `64` items may be logged.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`Logger`](crate::types::Logger) instead unless you intend to use
/// the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasLogger {
    /// Initialize a logger.
    fn init() -> Self;

    /// Log the given slice as-is. If logging is not successful an error will be
    /// returned. The event can be no bigger than `512` bytes.
    fn log_raw(&mut self, event: &[u8]) -> Result<(), LogError>;

    #[inline(always)]
    /// Log a serializable event by serializing it with a supplied serializer.
    /// Note that the serialized event must be no larger than `512` bytes.
    fn log<S: Serial>(&mut self, event: &S) -> Result<(), LogError> {
        let mut out = Vec::new();
        if event.serial(&mut out).is_err() {
            crate::trap(); // should not happen
        }
        self.log_raw(&out)
    }
}

/// Objects which provide cryptographic primitives.
///
/// # Deprecation notice
/// **This trait is deprecated along with
/// [`crate::test_infrastructure`].**
///
/// Use [`CryptoPrimitives`](crate::types::CryptoPrimitives) instead unless you
/// intend to use the deprecated test infrastructure.
///
/// See the [crate](../concordium_std/#deprecating-the-test_infrastructure)
/// documentation for more details.
pub trait HasCryptoPrimitives {
    /// Verify an ed25519 signature.
    fn verify_ed25519_signature(
        &self,
        public_key: PublicKeyEd25519,
        signature: SignatureEd25519,
        message: &[u8],
    ) -> bool;

    /// Verify an ecdsa signature over secp256k1 with the bitcoin-core
    /// implementation.
    fn verify_ecdsa_secp256k1_signature(
        &self,
        public_key: PublicKeyEcdsaSecp256k1,
        signature: SignatureEcdsaSecp256k1,
        message_hash: [u8; 32],
    ) -> bool;

    /// Hash the data using the SHA2-256 algorithm.
    fn hash_sha2_256(&self, data: &[u8]) -> HashSha2256;

    /// Hash the data using the SHA3-256 algorithm.
    fn hash_sha3_256(&self, data: &[u8]) -> HashSha3256;

    /// Hash the data using the Keccak-256 algorithm.
    fn hash_keccak_256(&self, data: &[u8]) -> HashKeccak256;
}

/// Add optimized unwrap behaviour that aborts the process instead of
/// panicking.
pub trait UnwrapAbort {
    /// The underlying result type of the unwrap, in case of success.
    type Unwrap;
    /// Unwrap or call [trap](./fn.trap.html). In contrast to
    /// the unwrap methods on [Option::unwrap](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)
    /// this method will tend to produce smaller code, at the cost of the
    /// ability to handle the panic.
    /// This is intended to be used only in `Wasm` code, where panics cannot be
    /// handled anyhow.
    fn unwrap_abort(self) -> Self::Unwrap;
}

/// Analogue of the `expect` methods on types such as [Option](https://doc.rust-lang.org/std/option/enum.Option.html),
/// but useful in a Wasm setting.
pub trait ExpectReport {
    type Unwrap;
    /// Like the default `expect` on, e.g., `Result`, but calling
    /// [fail](macro.fail.html) with the given message, instead of `panic`.
    fn expect_report(self, msg: &str) -> Self::Unwrap;
}

/// Analogue of the `expect_err` methods on [Result](https://doc.rust-lang.org/std/result/enum.Result.html),
/// but useful in a Wasm setting.
pub trait ExpectErrReport {
    type Unwrap;
    /// Like the default `expect_err` on, e.g., `Result`, but calling
    /// [fail](macro.fail.html) with the given message, instead of `panic`.
    fn expect_err_report(self, msg: &str) -> Self::Unwrap;
}

/// Analogue of the `expect_none` methods on [Option](https://doc.rust-lang.org/std/option/enum.Option.html),
/// but useful in a Wasm setting.
pub trait ExpectNoneReport {
    /// Like the default `expect_none_report` on, e.g., `Option`, but calling
    /// [fail](macro.fail.html) with the given message, instead of `panic`.
    fn expect_none_report(self, msg: &str);
}

/// The `DeserialWithState` trait provides a means of reading structures from
/// byte-sources ([`Read`]) for types that also need a reference to a
/// [`HasStateApi`] type.
///
/// Types that need a reference to the state include
/// [`StateBox`][crate::StateBox], [`StateMap`][crate::StateMap],
/// [`StateSet`][crate::StateSet], and structs or enums that contain one of
/// these types.
pub trait DeserialWithState<S>: Sized
where
    S: HasStateApi, {
    /// Attempt to read a structure from a given source and state, failing if
    /// an error occurs during deserialization or reading.
    fn deserial_with_state<R: Read>(state: &S, source: &mut R) -> ParseResult<Self>;
}

/// The `DeserialCtxWithState` trait provides a means of reading structures from
/// byte-sources ([`Read`]) using contextual information for types that also
/// need a reference to a [`HasStateApi`] type. The trait is a combination
/// of the [`DeserialCtx`] and [`DeserialWithState`] traits, which each has
/// additional documentation.
///
/// This trait is primarily used when deriving [`DeserialWithState`] for struct
/// or enums where both contextual information and the state is present.
///
/// For example:
/// ```no_run
/// # use concordium_std::*;
/// #[derive(DeserialWithState)]
/// #[concordium(state_parameter = "S")]
/// struct MyStruct<S: HasStateApi> {
///     #[concordium(size_length = 2)]
///     a: String, // Gets the contextual size_length information.
///     b: StateBox<u8, S>, // Needs a HasStateApi type
/// }
/// ```
pub trait DeserialCtxWithState<S>: Sized
where
    S: HasStateApi, {
    /// Attempt to read a structure from a given source, context, and state,
    /// failing if an error occurs during deserialization or reading.
    fn deserial_ctx_with_state<R: Read>(
        size_length: schema::SizeLength,
        ensure_ordered: bool,
        state: &S,
        source: &mut R,
    ) -> ParseResult<Self>;
}
