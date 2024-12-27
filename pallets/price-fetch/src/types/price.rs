use codec::{Encode, Decode};
use crate::RuntimeDebug;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct PricePayload<Public, BlockNumber> {
	pub block_number: BlockNumber,
	pub price: u32,
	pub public: Public,
}