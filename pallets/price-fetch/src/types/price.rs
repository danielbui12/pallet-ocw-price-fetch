use codec::{Encode, Decode};
use crate::{RuntimeDebug, Price, Vec};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct PricePayload {
	pub price: Price,
    pub symbol: Vec<u8>,
}
