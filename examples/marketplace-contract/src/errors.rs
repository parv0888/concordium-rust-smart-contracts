//! Provides error types that can be returned by the marketplace contract.
//! Read more about errors that can be returned by a Concordium Contract [here](https://developer.concordium.software/en/mainnet/smart-contracts/guides/custom-errors.html)
use concordium_std::*;

/// Possible set of Errors that can be returned by the contract.
#[derive(Serialize, Debug, PartialEq, Eq, Reject)]
pub enum MarketplaceError {
    ParseParams,
    CalledByAContract,
    TokenNotListed,

    /// Error thrown by the `cis2_client`
    Cis2ClientError(Cis2ClientError),
    CollectionNotCis2,
    InvalidAmountPaid,
    InvokeTransferError,
    NoBalance,
    NotOperator,
    InvalidCommission,
    InvalidTokenQuantity,
    InvalidRoyalty,
}

/// Possible set of Errors that can be returned by the `cis2_client`.
#[derive(Serialize, Debug, PartialEq, Eq, Reject)]
pub enum Cis2ClientError {
    InvokeContractError,
    ParseParams,
    ParseResult,
}
