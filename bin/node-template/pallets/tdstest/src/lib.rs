#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

//use node_cli;
use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + frame_system::offchain::CreateSignedTransaction<Call<Self>>  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type AuthorityId: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
  		/// Event emitted when a claim has been created.
  		ClaimCreated { who: T::AccountId, claim: T::Hash },
		/// Event emitted when a claim is revoked by the owner.
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}

  	#[pallet::error]
	pub enum Error<T> {
		/// The claim already exists.
		AlreadyClaimed,
		/// The claim does not exist, so it cannot be revoked.
		NoSuchClaim,
		/// The claim is owned by another account, so caller can't revoke it.
		NotClaimOwner,
	}
	#[pallet::storage]
	pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

  	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
		// Check that the extrinsic was signed and get the signer.
		// This function will return an error if the extrinsic is not signed.
		let sender = ensure_signed(origin)?;

		// Verify that the specified claim has not already been stored.
		ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

		// Get the block number from the FRAME System pallet.
		let current_block = <frame_system::Pallet<T>>::block_number();

		// Store the claim with the sender and block number.
		Claims::<T>::insert(&claim, (&sender, current_block));

		// Emit an event that the claim was created.
		Self::deposit_event(Event::ClaimCreated { who: sender, claim });

		frame_support::log::info!("Claim success {:?}", claim);
		Ok(())
	}

	#[pallet::weight(0)]
	pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
		// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
		let sender = ensure_signed(origin)?;

		// Get owner of the claim, if none return an error.
		let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

		// Verify that sender of the current call is the claim owner.
		ensure!(sender == owner, Error::<T>::NotClaimOwner);

		// Remove claim from storage.
		Claims::<T>::remove(&claim);

		// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	/// Offchain worker entry point.
	///
	/// By implementing `fn offchain_worker` you declare a new offchain worker.
	/// This function will be called when the node is fully synced and a new best block is
	/// successfully imported.
	/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
	/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
	/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			use frame_support::*;
			// use frame_system::offchain::SendSignedTransaction;
			// use frame_support::sp_runtime::traits::Hash;

			log::info!("Offchain worker called for TDS Test pallet {:?}", block_number);
			sp_runtime::offchain::ipfs::ipfs_core::test();

			//   sp_runtime::offchain::ipfs::PendingRequest::new(request).map_err(|_| Error::CannotCreateRequest)?;

			//node_cli::ipfs_api::ipfs_core::test();
		}
	}

}

pub mod crypto {
	use super::KEY_TYPE;
	use frame_support::*;
	// use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
	}
}
