#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use sp_runtime::{traits::StaticLookup, DispatchError, DispatchResult, RuntimeDebug};

use frame_support::{decl_error, decl_event, decl_module, decl_storage};
use frame_system::{self as system, ensure_root, ensure_signed};

use ci_primitives::{ArtistId, Text};

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
	    Tmp(AccountId),
	}
);

decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Market {
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

    }
}
