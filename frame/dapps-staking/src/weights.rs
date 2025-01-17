#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet-dapps-staking.
pub trait WeightInfo {
	fn register() -> Weight;
	fn unregister() -> Weight;
	fn withdraw_from_unregistered() -> Weight;
	fn enable_developer_pre_approval() -> Weight;
	fn developer_pre_approval() -> Weight;
	fn bond_and_stake() -> Weight;
	fn unbond_and_unstake() -> Weight;
	fn withdraw_unbonded() -> Weight;
	fn claim_staker_without_restake() -> Weight;
	fn claim_staker_with_restake() -> Weight;
	fn claim_dapp() -> Weight;
	fn force_new_era() -> Weight;
	fn maintenance_mode() -> Weight;
	fn set_reward_destination() -> Weight;
	fn nomination_transfer() -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	fn register() -> Weight {
		Weight::from_ref_time(32_139_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn unregister() -> Weight {
		Weight::from_ref_time(31_929_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_from_unregistered() -> Weight {
		Weight::from_ref_time(46_667_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		Weight::from_ref_time(2_745_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		Weight::from_ref_time(5_320_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		Weight::from_ref_time(133_638_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn unbond_and_unstake() -> Weight {
		Weight::from_ref_time(134_480_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_unbonded() -> Weight {
		Weight::from_ref_time(114_252_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:2 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_staker_with_restake() -> Weight {
		Weight::from_ref_time(78_506_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:2 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:2 w:2)
	// Storage: DappsStaking ContractEraStake (r:2 w:2)
	fn nomination_transfer() -> Weight {
		Weight::from_ref_time(56_495_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:0)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:0)
	fn claim_staker_without_restake() -> Weight {
		Weight::from_ref_time(56_575_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	fn claim_dapp() -> Weight {
		Weight::from_ref_time(31_619_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		Weight::from_ref_time(2_815_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:1)
	fn maintenance_mode() -> Weight {
		Weight::from_ref_time(10_970_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	fn set_reward_destination() -> Weight {
		Weight::from_ref_time(15_489_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: DappsStaking PreApprovalIsEnabled (r:1 w:0)
	fn register() -> Weight {
		Weight::from_ref_time(32_139_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn unregister() -> Weight {
		Weight::from_ref_time(31_929_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_from_unregistered() -> Weight {
		Weight::from_ref_time(46_667_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		Weight::from_ref_time(2_745_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		Weight::from_ref_time(5_320_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		Weight::from_ref_time(133_638_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking StakersInfo (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn unbond_and_unstake() -> Weight {
		Weight::from_ref_time(134_480_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_unbonded() -> Weight {
		Weight::from_ref_time(114_252_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:2 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:2 w:2)
	// Storage: DappsStaking ContractEraStake (r:2 w:2)
	fn nomination_transfer() -> Weight {
		Weight::from_ref_time(56_495_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:2 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_staker_with_restake() -> Weight {
		Weight::from_ref_time(78_506_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:0)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:0)
	fn claim_staker_without_restake() -> Weight {
		Weight::from_ref_time(56_575_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	fn claim_dapp() -> Weight {
		Weight::from_ref_time(31_619_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		Weight::from_ref_time(2_815_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:1)
	fn maintenance_mode() -> Weight {
		Weight::from_ref_time(10_970_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking PalletDisabled (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	fn set_reward_destination() -> Weight {
		Weight::from_ref_time(15_489_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}