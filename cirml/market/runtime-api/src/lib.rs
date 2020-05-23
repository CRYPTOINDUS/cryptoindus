#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;

use sp_std::prelude::Vec;

use ci_primitives::ArtistId;

sp_api::decl_runtime_apis! {
    pub trait MarketApi<AccountId, ArtvenusId> where
        AccountId: Codec,
        ArtvenusId: Codec,
    {
        fn on_sell() -> Vec<ArtvenusId>;
    }
}
