// This pallet use The Open Runtime Module Library (ORML) which is a community maintained collection of Substrate runtime modules.
// Thanks to all contributors of orml.
// Ref: https://github.com/open-web3-stack/open-runtime-module-library

use codec::FullCodec;
use codec::{Decode, Encode};
use sp_runtime::{
    traits::{AtLeast32Bit, Bounded, MaybeSerializeDeserialize},
    DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
};

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum Change<Value> {
    /// No change.
    NoChange,
    /// Changed to new value.
    NewValue(Value),
}

/// Auction info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug)]
pub struct AuctionInfo<AccountId, Balance, BlockNumber> {
    /// Current bidder and bid price.
    pub bid: Option<(AccountId, Balance)>,
    /// Define which block this auction will be started.
    pub start: BlockNumber,
    /// Define which block this auction will be ended.
    pub end: Option<BlockNumber>,
}

/// Abstraction over a simple auction system.
pub trait Auction<AccountId, BlockNumber> {
    /// The id of an AuctionInfo
    type AuctionId: FullCodec
        + Default
        + Copy
        + Eq
        + PartialEq
        + MaybeSerializeDeserialize
        + Bounded
        + Debug;
    /// The price to bid.
    type Balance: AtLeast32Bit + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

    /// The auction info of `id`
    fn auction_info(
        id: Self::AuctionId,
    ) -> Option<AuctionInfo<AccountId, Self::Balance, BlockNumber>>;
    /// Update the auction info of `id` with `info`
    fn update_auction(
        id: Self::AuctionId,
        info: AuctionInfo<AccountId, Self::Balance, BlockNumber>,
    ) -> DispatchResult;
    /// Create new auction with specific startblock and endblock, return the id
    /// of the auction
    fn new_auction(
        recipient: AccountId,
        initial_amount: Self::Balance,
        start: BlockNumber,
        end: Option<BlockNumber>,
    ) -> Result<Self::AuctionId, DispatchError>;
    /// Remove auction by `id`
    fn remove_auction(id: Self::AuctionId);

    fn swap_bidders(new_bidder: &AccountId, last_bidder: Option<&AccountId>);
}

/// The result of bid handling.
pub struct OnNewBidResult<BlockNumber> {
    /// Indicates if the bid was accepted
    pub accept_bid: bool,
    /// The auction end change.
    pub auction_end_change: Change<Option<BlockNumber>>,
}

/// Hooks for auction to handle bids.
pub trait AuctionHandler<AccountId, Balance, BlockNumber, AuctionId> {
    /// Called when new bid is received.
    /// The return value determines if the bid should be accepted and update
    /// auction end time. Implementation should reserve money from current
    /// winner and refund previous winner.
    fn on_new_bid(
        now: BlockNumber,
        id: AuctionId,
        new_bid: (AccountId, Balance),
        last_bid: Option<(AccountId, Balance)>,
    ) -> OnNewBidResult<BlockNumber>;
    /// End an auction with `winner`
    fn on_auction_ended(id: AuctionId, winner: Option<(AccountId, Balance)>);
}
