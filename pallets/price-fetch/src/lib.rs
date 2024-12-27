#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use serde_json::Value as JsonValue;
use polkadot_sdk::{
    frame_support::{
        self as frame_support, traits::Get,
        pallet_prelude::*,
    },
    frame_system::{
        self as frame_system,
        pallet_prelude::*,
        offchain::{
            AppCrypto, SendUnsignedTransaction,
            Signer, SigningTypes, SubmitTransaction,
        },
        pallet_prelude::BlockNumberFor,
    },
    sp_core::crypto::KeyTypeId,
    sp_io as sp_io,
    sp_runtime::{
        offchain::{
            http,
            storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
            Duration,
        },
        traits::Zero,
        transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
        RuntimeDebug,
    }
};


#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;

mod types;
mod price_fetch;

pub use pallet::*;

use types::price::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config:
        SigningTypes + polkadot_sdk::frame_system::Config
	{
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

		/// Maximum number of prices.
		#[pallet::constant]
		type MaxPrices: Get<u32>;

        /// Number of blocks of cooldown after unsigned transaction is included.
		///
		/// This ensures that we only accept unsigned transactions once, every `UnsignedInterval`
		/// blocks.
		#[pallet::constant]
        type UnsignedInterval: Get<u64>;

        /// A configuration for base priority of unsigned transactions.
		///
		/// This is exposed so that it can be tuned for particular runtime, when
		/// multiple pallets send unsigned transactions.
		#[pallet::constant]
		type UnsignedPriority: Get<TransactionPriority>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextUnsignedAt<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			log::info!("Hello World from offchain workers!");

    		let parent_hash = <frame_system::Pallet<T>>::block_hash(block_number - 1u32.into());
			log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

    		let average: Option<u32> = Self::average_price();
			log::debug!("Current price: {:?}", average);

			// For this example we are going to send both signed and unsigned transactions
			// depending on the block number.
			// Usually it's enough to choose one or the other.
            if let Err(e) = Self::fetch_price_and_send_raw_unsigned(block_number) {
				log::error!("Error: {}", e);
			}
		}
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event generated when new price is accepted to contribute to the average.
		NewPrice { price: u32, maybe_who: Option<T::AccountId> },
	}

	/// A public part of the pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    	pub fn submit_price_unsigned(
			origin: OriginFor<T>,
			_block_number: BlockNumberFor<T>,
			price: u32,
		) -> DispatchResultWithPostInfo {
			// This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin)?;
			// Add the price to the on-chain list, but mark it as coming from an empty address.
			Self::add_price(None, price);
			// now increment the block number at which we expect next unsigned transaction.
			let current_block = <frame_system::Pallet<T>>::block_number();
            // TODO
			// <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get().into());
			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		    if let Call::submit_price_unsigned { block_number, price: new_price } = call {
				Self::validate_transaction_parameters(block_number, new_price)
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}



pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"coin");
/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
	use super::KEY_TYPE;
	use polkadot_sdk::{
        sp_core::sr25519::Signature as Sr25519Signature,
        sp_runtime::{
            app_crypto::{app_crypto, sr25519},
            traits::Verify,
            MultiSignature, MultiSigner,
        }
    };
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl polkadot_sdk::frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = polkadot_sdk::sp_core::sr25519::Signature;
		type GenericPublic = polkadot_sdk::sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl polkadot_sdk::frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = polkadot_sdk::sp_core::sr25519::Signature;
		type GenericPublic = polkadot_sdk::sp_core::sr25519::Public;
	}
}
