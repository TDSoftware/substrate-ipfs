pub mod pallet {
	#[pallet::error]
	pub enum Error<T> {
	  CannotCreateRequest,
	  RequestTimeout,
	  RequestFailed,
	  FailedToAcquireLock,
	}

}
