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
            SubmitTransaction,
            CreateSignedTransaction,
        },
        pallet_prelude::BlockNumberFor,
    },
    sp_core::crypto::KeyTypeId,
    sp_io as sp_io,
    sp_runtime::{
        offchain::{
            http, Duration,
        },
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
mod constants;

pub use pallet::*;

use types::price::*;
use types::common::*;
use constants::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config:
    CreateSignedTransaction<Call<Self>> + polkadot_sdk::frame_system::Config
	{
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// Number of blocks of cooldown after unsigned transaction is included.
		///
		/// This ensures that we only accept unsigned transactions once, every `UnsignedInterval`
		/// blocks.
		#[pallet::constant]
		type UnsignedInterval: Get<BlockNumberFor<Self>>;

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
	pub type Prices<T: Config> = StorageMap<
        _,
        Twox64Concat,
        StrVecBytes,
        Price,
        ValueQuery
    >;

	#[pallet::storage]
	pub type NextUnsignedAt<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			log::info!("Hello World from offchain workers!");

    		let parent_hash = <frame_system::Pallet<T>>::block_hash(block_number - 1u32.into());
			log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

			// For this example we are going to send both signed and unsigned transactions
			// depending on the block number.
			// Usually it's enough to choose one or the other.
            let mut price_data: Vec<PricePayload> = Vec::<PricePayload>::new();
            for (sym, remote_url) in WHITELIST_CRYPTO.iter() {
                let res = Self::safe_fetch_price(
                    block_number,
                    sym,
                    remote_url
                );
                match res {
                    Ok(p) => price_data.push(PricePayload { price: p, symbol: sym.to_vec() }),
                    Err(e) => log::error!("Fetch price error: {}", e),
                };
            }

            if price_data.len() > 0usize {
                if let Err(e) = Self::ocw_submit_tx(block_number, price_data) {
                    log::error!("Submit tx error: {:?}", e);
                }
            }
		}
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event generated when new price is accepted to contribute to the average.
		NewPrice(Vec<u8>, Price),
	}

    #[pallet::error]
    pub enum Error<T> {
        PassedBlock,
    }

	/// A public part of the pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    	pub fn submit_price_unsigned(
			origin: OriginFor<T>,
            payload: Vec<PricePayload>,
            block_number: BlockNumberFor<T>
		) -> DispatchResultWithPostInfo {
            // This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin)?;
			let current_block = <frame_system::Pallet<T>>::block_number();
            log::info!("current block: {:?}, block number: {:?}", current_block.clone(), block_number);
            ensure!(current_block.clone() >= block_number, Error::<T>::PassedBlock);
			// Add the price to the on-chain list, but mark it as coming from an empty address.
			Self::add_price(payload);
			// now increment the block number at which we expect next unsigned transaction.
			<NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		    if let Call::submit_price_unsigned { payload, block_number } = call {
				Self::validate_transaction_parameters(block_number)
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
