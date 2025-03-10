// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The tests for functionality concerning the "external" origin.

use super::*;

#[test]
fn veto_external_works() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let charlie = Public::from_string("/Charlie").ok().unwrap();
		let dave = Public::from_string("/Dave").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)));
		assert!(<NextExternal<Test>>::contains_key(0));

		let h = set_balance_proposal(2).hash();
		assert_ok!(Democracy::veto_external(RuntimeOrigin::signed(charlie), 0, h));
		// cancelled.
		assert!(!<NextExternal<Test>>::contains_key(0));
		// fails - same proposal can't be resubmitted.
		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)),
			Error::<Test>::ProposalBlacklisted
		);

		fast_forward_to(1);
		// fails as we're still in cooloff period.
		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)),
			Error::<Test>::ProposalBlacklisted
		);

		fast_forward_to(2);
		// works; as we're out of the cooloff period.
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2),));
		assert!(<NextExternal<Test>>::contains_key(0));

		// 3 can't veto the same thing twice.
		assert_noop!(
			Democracy::veto_external(RuntimeOrigin::signed(charlie), 0, h),
			Error::<Test>::AlreadyVetoed
		);

		// 4 vetoes.
		assert_ok!(Democracy::veto_external(RuntimeOrigin::signed(dave), 0, h));
		// cancelled again.
		assert!(!<NextExternal<Test>>::contains_key(0));

		fast_forward_to(3);
		// same proposal fails as we're still in cooloff
		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)),
			Error::<Test>::ProposalBlacklisted
		);
		// different proposal works fine.
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(3),));
	});
}

#[test]
fn external_blacklisting_should_work() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2),));

		let hash = set_balance_proposal(2).hash();
		assert_ok!(Democracy::blacklist(RuntimeOrigin::root(), 0, hash, None));

		fast_forward_to(2);
		assert_noop!(Democracy::referendum_status(0, 0), Error::<Test>::ReferendumInvalid);

		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)),
			Error::<Test>::ProposalBlacklisted,
		);
	});
}

#[test]
fn external_referendum_works() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::signed(alice), 0, set_balance_proposal(2),),
			BadOrigin,
		);
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(2)));
		assert_noop!(
			Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(1)),
			Error::<Test>::DuplicateProposal
		);
		fast_forward_to(2);
		assert_eq!(
			Democracy::referendum_status(0, 0),
			Ok(ReferendumStatus {
				end: 4,
				proposal: set_balance_proposal(2),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}

#[test]
fn external_majority_referendum_works() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_noop!(
			Democracy::external_propose_majority(
				RuntimeOrigin::signed(alice),
				0,
				set_balance_proposal(2)
			),
			BadOrigin,
		);
		assert_ok!(Democracy::external_propose_majority(
			RuntimeOrigin::root(),
			0,
			set_balance_proposal(2)
		));
		fast_forward_to(2);
		assert_eq!(
			Democracy::referendum_status(0, 0),
			Ok(ReferendumStatus {
				end: 4,
				proposal: set_balance_proposal(2),
				threshold: VoteThreshold::SimpleMajority,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}

#[test]
fn external_default_referendum_works() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let charlie = Public::from_string("/Charlie").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_noop!(
			Democracy::external_propose_default(
				RuntimeOrigin::signed(charlie),
				0,
				set_balance_proposal(2)
			),
			BadOrigin,
		);
		assert_ok!(Democracy::external_propose_default(
			RuntimeOrigin::root(),
			0,
			set_balance_proposal(2)
		));
		fast_forward_to(2);
		assert_eq!(
			Democracy::referendum_status(0, 0),
			Ok(ReferendumStatus {
				end: 4,
				proposal: set_balance_proposal(2),
				threshold: VoteThreshold::SuperMajorityAgainst,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}

#[test]
fn external_and_public_interleaving_works() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let charlie = Public::from_string("/Charlie").ok().unwrap();
		let ferdie = Public::from_string("/Ferdie").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(1),));
		assert_ok!(propose_set_balance(0, ferdie, 2, 2));

		fast_forward_to(2);

		// both waiting: external goes first.
		assert_eq!(
			Democracy::referendum_status(0, 0),
			Ok(ReferendumStatus {
				end: 4,
				proposal: set_balance_proposal(1),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish external
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(3),));

		fast_forward_to(4);

		// both waiting: public goes next.
		assert_eq!(
			Democracy::referendum_status(0, 1),
			Ok(ReferendumStatus {
				end: 6,
				proposal: set_balance_proposal(2),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// don't replenish public

		fast_forward_to(6);

		// it's external "turn" again, though since public is empty that doesn't really matter
		assert_eq!(
			Democracy::referendum_status(0, 2),
			Ok(ReferendumStatus {
				end: 8,
				proposal: set_balance_proposal(3),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish external
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(5),));

		fast_forward_to(8);

		// external goes again because there's no public waiting.
		assert_eq!(
			Democracy::referendum_status(0, 3),
			Ok(ReferendumStatus {
				end: 10,
				proposal: set_balance_proposal(5),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish both
		assert_ok!(Democracy::external_propose(RuntimeOrigin::root(), 0, set_balance_proposal(7),));
		assert_ok!(propose_set_balance(0, ferdie, 4, 2));

		fast_forward_to(10);

		// public goes now since external went last time.
		assert_eq!(
			Democracy::referendum_status(0, 4),
			Ok(ReferendumStatus {
				end: 12,
				proposal: set_balance_proposal(4),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish public again
		assert_ok!(propose_set_balance(0, ferdie, 6, 2));
		// cancel external
		let h = set_balance_proposal(7).hash();
		assert_ok!(Democracy::veto_external(RuntimeOrigin::signed(charlie), 0, h));

		fast_forward_to(12);

		// public goes again now since there's no external waiting.
		assert_eq!(
			Democracy::referendum_status(0, 5),
			Ok(ReferendumStatus {
				end: 14,
				proposal: set_balance_proposal(6),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}
