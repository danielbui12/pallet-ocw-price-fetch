// use crate as pallet_price_fetch;
// use crate::KeyTypeId;
// use polkadot_sdk::{
//     frame_system::{self as frame_system},
//     sp_io::{self as sp_io},
//     frame_support::{
//         self as frame_support,
//         derive_impl,
//         traits::{ConstU16, ConstU128, ConstU64, ConstU32}
//     },
//     sp_core::{H256, sr25519::{self as sr25519, Signature}},
//     sp_runtime::{
//         self as sp_runtime,
//         traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount},
//         BuildStorage,
//         generic,
//     },
//     pallet_balances as pallet_balances,
//     pallet_transaction_payment as pallet_transaction_payment
// };

// // Configure a mock runtime to test the pallet.
// frame_support::construct_runtime!(
// 	pub enum TestRuntime
// 	{
// 		System: frame_system,
//         Balances: pallet_balances,
// 		PriceFetch: pallet_price_fetch,
// 	}
// );

// type Block = frame_system::mocking::MockBlock<TestRuntime>;
// type Hash = H256;
// type Balance = u128;
// pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

// type SignedExtra = (
// 	frame_system::CheckNonZeroSender<TestRuntime>,
// 	frame_system::CheckSpecVersion<TestRuntime>,
// 	frame_system::CheckTxVersion<TestRuntime>,
// 	frame_system::CheckGenesis<TestRuntime>,
// 	frame_system::CheckEra<TestRuntime>,
// 	frame_system::CheckNonce<TestRuntime>,
// 	frame_system::CheckWeight<TestRuntime>,
// 	pallet_transaction_payment::ChargeTransactionPayment<TestRuntime>,
// );

// #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
// impl frame_system::Config for TestRuntime {
//     type BaseCallFilter = frame_support::traits::Everything;
// 	type BlockWeights = ();
// 	type BlockLength = ();
// 	type DbWeight = ();
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type RuntimeCall = RuntimeCall;
// 	type Nonce = u64;
// 	type Hash = Hash;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = sr25519::Public;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Block = Block;
// 	type RuntimeEvent = RuntimeEvent;
// 	type BlockHashCount = ConstU64<250>;
// 	type Version = ();
// 	type PalletInfo = PalletInfo;
// 	type AccountData = pallet_balances::AccountData<Balance>;
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// 	type SS58Prefix = ConstU16<42>;
// 	type OnSetCode = ();
// 	type MaxConsumers = ConstU32<16>;
//     type RuntimeTask = ();
//     type SingleBlockMigrations = ();
//     type MultiBlockMigrator = ();
//     type PreInherents = ();
//     type PostInherents = ();
//     type PostTransactions = ();
// }


// #[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
// impl pallet_balances::Config for TestRuntime {
//     type Balance = u128;
//     type DustRemoval = ();
//     type RuntimeEvent = RuntimeEvent;
//     type ExistentialDeposit = ConstU128<1>;
//     type AccountStore = System;
//     type WeightInfo = ();
//     type MaxLocks = ();
//     type MaxReserves = ();
//     type ReserveIdentifier = [u8; 8];
//     type FreezeIdentifier = ();
//     type MaxFreezes = ();
//     type RuntimeHoldReason = ();
//     type RuntimeFreezeReason = ();
// }

// /// Unchecked extrinsic type as expected by this runtime.
// pub type UncheckedExtrinsic =
// 	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
// type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// impl frame_system::offchain::SigningTypes for TestRuntime {
// 	type Public = <Signature as Verify>::Signer;
// 	type Signature = Signature;
// }

// impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for TestRuntime
// where
// 	RuntimeCall: From<LocalCall>,
// {
// 	type Extrinsic = UncheckedExtrinsic;
// 	type OverarchingCall = RuntimeCall;
// }

// // impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for TestRuntime
// // where
// // 	RuntimeCall: From<LocalCall>,
// // {
// // 	fn create_transaction<
// // 		C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>,
// // 	>(
// // 		call: RuntimeCall,
// // 		_public: <Signature as Verify>::Signer,
// // 		_account: AccountId,
// // 		nonce: u64,
// // 	) -> Option<Extrinsic> {
// // 		Some(Extrinsic::new_signed(call, nonce, (), ()))
// // 	}
// // }

// frame_support::parameter_types! {
// 	pub const UnsignedPriority: u64 = 1 << 20;
// }

// impl pallet_price_fetch::Config for TestRuntime {
//     type RuntimeEvent = RuntimeEvent;
//     type AuthorityId = pallet_price_fetch::crypto::TestAuthId;
// 	type UnsignedPriority = UnsignedPriority;
// 	type UnsignedInterval = ConstU64<128>;
// }

// // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap().into()
// }