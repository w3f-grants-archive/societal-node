
//! Autogenerated weights for pallet_dao
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/societal-node
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 20
// --pallet
// pallet_dao
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --template
// ./.maintain/frame-weight-template.hbs
// --output
// ./pallets/dao/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dao.
pub trait WeightInfo {
	fn create_dao() -> Weight;
	fn approve_dao() -> Weight;
	fn update_dao_metadata() -> Weight;
	fn update_dao_policy() -> Weight;
	fn mint_dao_token() -> Weight;
	fn spend_dao_funds() -> Weight;
	fn launch_dao_referendum() -> Weight;
	fn bake_dao_referendum() -> Weight;
	fn subscribe() -> Weight;
	fn extend_subscription() -> Weight;
	fn change_subscription() -> Weight;
	fn remove_dao() -> Weight;
	fn schedule_subscription_payment() -> Weight;
	fn cancel_subscription_payment() -> Weight;
}

/// Weights for pallet_dao using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:1 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn create_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459`
		//  Estimated: `134792`
		// Minimum execution time: 200_000_000 picoseconds.
		Weight::from_parts(202_000_000, 134792)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(19_u64))
	}
	/// Storage: Dao PendingDaos (r:1 w:1)
	/// Proof: Dao PendingDaos (max_values: None, max_size: Some(7639), added: 10114, mode: MaxEncodedLen)
	/// Storage: Dao PendingDaoHashes (r:1 w:1)
	/// Proof: Dao PendingDaoHashes (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn approve_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1524`
		//  Estimated: `116384`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(62_000_000, 116384)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	fn update_dao_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1222`
		//  Estimated: `8153`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 8153)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:4 w:4)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn update_dao_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `869`
		//  Estimated: `178926`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(94_000_000, 178926)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_dao_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1668`
		//  Estimated: `15451`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 15451)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DaoTreasury Approvals (r:1 w:1)
	/// Proof: DaoTreasury Approvals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:1 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	fn spend_dao_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `784`
		//  Estimated: `14976`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(27_000_000, 14976)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LastTabledWasExternal (r:1 w:0)
	/// Proof: DaoDemocracy LastTabledWasExternal (max_values: None, max_size: Some(13), added: 2488, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy NextExternal (r:1 w:0)
	/// Proof: DaoDemocracy NextExternal (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy DepositOf (r:1 w:1)
	/// Proof: DaoDemocracy DepositOf (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn launch_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2102`
		//  Estimated: `44554`
		// Minimum execution time: 56_000_000 picoseconds.
		Weight::from_parts(58_000_000, 44554)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LowestUnbaked (r:1 w:1)
	/// Proof: DaoDemocracy LowestUnbaked (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn bake_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `747`
		//  Estimated: `14265`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_000_000, 14265)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn subscribe() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `17627`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(44_000_000, 17627)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn extend_subscription() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `723`
		//  Estimated: `17627`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 17627)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn change_subscription() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841`
		//  Estimated: `17627`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(36_000_000, 17627)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:1)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Proposals (r:1 w:1)
	/// Proof: DaoTechnicalCommittee Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance Proposals (r:1 w:1)
	/// Proof: DaoEthGovernance Proposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyCount (r:1 w:1)
	/// Proof: DaoBounties BountyCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:0 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:0 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance PendingProposals (r:0 w:1)
	/// Proof: DaoEthGovernance PendingProposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance ProposalCount (r:0 w:1)
	/// Proof: DaoEthGovernance ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	/// Storage: DaoCouncil ProposalCount (r:0 w:1)
	/// Proof: DaoCouncil ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:0 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee ProposalCount (r:0 w:1)
	/// Proof: DaoTechnicalCommittee ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn remove_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2328`
		//  Estimated: `140130`
		// Minimum execution time: 116_000_000 picoseconds.
		Weight::from_parts(117_000_000, 140130)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(24_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:0)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn schedule_subscription_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `734`
		//  Estimated: `53493`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 53493)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn cancel_subscription_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `706`
		//  Estimated: `49566`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(23_000_000, 49566)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:1 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn create_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459`
		//  Estimated: `134792`
		// Minimum execution time: 200_000_000 picoseconds.
		Weight::from_parts(202_000_000, 134792)
			.saturating_add(RocksDbWeight::get().reads(16_u64))
			.saturating_add(RocksDbWeight::get().writes(19_u64))
	}
	/// Storage: Dao PendingDaos (r:1 w:1)
	/// Proof: Dao PendingDaos (max_values: None, max_size: Some(7639), added: 10114, mode: MaxEncodedLen)
	/// Storage: Dao PendingDaoHashes (r:1 w:1)
	/// Proof: Dao PendingDaoHashes (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Dao NextDaoId (r:1 w:1)
	/// Proof: Dao NextDaoId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:1 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:0 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:0 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn approve_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1524`
		//  Estimated: `116384`
		// Minimum execution time: 60_000_000 picoseconds.
		Weight::from_parts(62_000_000, 116384)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(12_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	fn update_dao_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1222`
		//  Estimated: `8153`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 8153)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:4 w:4)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn update_dao_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `869`
		//  Estimated: `178926`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(94_000_000, 178926)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_dao_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1668`
		//  Estimated: `15451`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 15451)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DaoTreasury Approvals (r:1 w:1)
	/// Proof: DaoTreasury Approvals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:1 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	fn spend_dao_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `784`
		//  Estimated: `14976`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(27_000_000, 14976)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LastTabledWasExternal (r:1 w:0)
	/// Proof: DaoDemocracy LastTabledWasExternal (max_values: None, max_size: Some(13), added: 2488, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy NextExternal (r:1 w:0)
	/// Proof: DaoDemocracy NextExternal (max_values: None, max_size: Some(144), added: 2619, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy DepositOf (r:1 w:1)
	/// Proof: DaoDemocracy DepositOf (max_values: None, max_size: Some(3242), added: 5717, mode: MaxEncodedLen)
	/// Storage: Dao Daos (r:1 w:0)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn launch_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2102`
		//  Estimated: `44554`
		// Minimum execution time: 56_000_000 picoseconds.
		Weight::from_parts(58_000_000, 44554)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy LowestUnbaked (r:1 w:1)
	/// Proof: DaoDemocracy LowestUnbaked (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:1 w:0)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	fn bake_dao_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `747`
		//  Estimated: `14265`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_000_000, 14265)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn subscribe() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `17627`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(44_000_000, 17627)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn extend_subscription() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `723`
		//  Estimated: `17627`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 17627)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription SubscriptionTiers (r:1 w:0)
	/// Proof: DaoSubscription SubscriptionTiers (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn change_subscription() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841`
		//  Estimated: `17627`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(36_000_000, 17627)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Dao Daos (r:1 w:1)
	/// Proof: Dao Daos (max_values: None, max_size: Some(1063), added: 3538, mode: MaxEncodedLen)
	/// Storage: Dao Policies (r:1 w:1)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:1)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Proposals (r:1 w:1)
	/// Proof: DaoCouncil Proposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Proposals (r:1 w:1)
	/// Proof: DaoTechnicalCommittee Proposals (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy PublicProps (r:1 w:1)
	/// Proof: DaoDemocracy PublicProps (max_values: None, max_size: Some(8363), added: 10838, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumCount (r:1 w:1)
	/// Proof: DaoDemocracy ReferendumCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance Proposals (r:1 w:1)
	/// Proof: DaoEthGovernance Proposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyCount (r:1 w:1)
	/// Proof: DaoBounties BountyCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:3 w:3)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommitteeMembers Members (r:0 w:1)
	/// Proof: DaoTechnicalCommitteeMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoCouncilMembers Members (r:0 w:1)
	/// Proof: DaoCouncilMembers Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance PendingProposals (r:0 w:1)
	/// Proof: DaoEthGovernance PendingProposals (max_values: None, max_size: Some(1613), added: 4088, mode: MaxEncodedLen)
	/// Storage: DaoEthGovernance ProposalCount (r:0 w:1)
	/// Proof: DaoEthGovernance ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoDemocracy ReferendumInfoOf (r:0 w:1)
	/// Proof: DaoDemocracy ReferendumInfoOf (max_values: None, max_size: Some(213), added: 2688, mode: MaxEncodedLen)
	/// Storage: DaoCouncil ProposalCount (r:0 w:1)
	/// Proof: DaoCouncil ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoCouncil Members (r:0 w:1)
	/// Proof: DaoCouncil Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	/// Storage: DaoBounties BountyApprovals (r:0 w:1)
	/// Proof: DaoBounties BountyApprovals (max_values: None, max_size: Some(414), added: 2889, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee ProposalCount (r:0 w:1)
	/// Proof: DaoTechnicalCommittee ProposalCount (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: DaoTechnicalCommittee Members (r:0 w:1)
	/// Proof: DaoTechnicalCommittee Members (max_values: None, max_size: Some(3214), added: 5689, mode: MaxEncodedLen)
	fn remove_dao() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2328`
		//  Estimated: `140130`
		// Minimum execution time: 116_000_000 picoseconds.
		Weight::from_parts(117_000_000, 140130)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(24_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: DaoSubscription Subscriptions (r:1 w:0)
	/// Proof: DaoSubscription Subscriptions (max_values: None, max_size: Some(462), added: 2937, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn schedule_subscription_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `734`
		//  Estimated: `53493`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 53493)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Dao Policies (r:1 w:0)
	/// Proof: Dao Policies (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn cancel_subscription_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `706`
		//  Estimated: `49566`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(23_000_000, 49566)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
