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

//! The tests for normal voting functionality.

use super::*;

#[test]
fn overvoting_should_fail() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let bob = Public::from_string("/Bob").ok().unwrap();

		let r = begin_referendum();
		assert_noop!(
			Democracy::vote(RuntimeOrigin::signed(alice), 0, r, aye(bob)),
			Error::<Test>::InsufficientFunds
		);
	});
}

#[test]
fn split_voting_should_work() {
	new_test_ext().execute_with(|| {
		let eve = Public::from_string("/Eve").ok().unwrap();

		let r = begin_referendum();
		let v = AccountVote::Split { aye: 40, nay: 20 };
		assert_noop!(
			Democracy::vote(RuntimeOrigin::signed(eve), 0, r, v),
			Error::<Test>::InsufficientFunds
		);
		let v = AccountVote::Split { aye: 30, nay: 20 };
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(eve), 0, r, v));

		assert_eq!(tally(0, r), Tally { ayes: 3, nays: 2, turnout: 50 });
	});
}

#[test]
fn split_vote_cancellation_should_work() {
	new_test_ext().execute_with(|| {
		let eve = Public::from_string("/Eve").ok().unwrap();

		let r = begin_referendum();
		let v = AccountVote::Split { aye: 30, nay: 20 };
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(eve), 0, r, v));
		assert_ok!(Democracy::remove_vote(RuntimeOrigin::signed(eve), 0, r));
		assert_eq!(tally(0, r), Tally { ayes: 0, nays: 0, turnout: 0 });
		assert_ok!(Democracy::unlock(RuntimeOrigin::signed(eve), 0, eve));
		assert_eq!(Assets::locks(0, eve), vec![]);
	});
}

#[test]
fn single_proposal_should_work() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();

		System::set_block_number(0);

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		assert_ok!(propose_set_balance(0, alice, 2, 1));
		let r = 0;
		assert!(Democracy::referendum_info(0, r).is_none());

		// start of 2 => next referendum scheduled.
		fast_forward_to(2);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(alice), 0, r, aye(alice)));

		assert_eq!(Democracy::referendum_count(0), 1);
		assert_eq!(
			Democracy::referendum_status(0, 0),
			Ok(ReferendumStatus {
				end: 4,
				proposal: set_balance_proposal(2),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 1, nays: 0, turnout: 10 },
			})
		);

		fast_forward_to(3);

		// referendum still running
		assert_ok!(Democracy::referendum_status(0, 0));

		// referendum runs during 2 and 3, ends @ start of 4.
		fast_forward_to(4);

		assert_noop!(Democracy::referendum_status(0, 0), Error::<Test>::ReferendumInvalid);
		assert!(pallet_scheduler::Agenda::<Test>::get(6)[0].is_some());

		// referendum passes and wait another two blocks for enactment.
		fast_forward_to(6);

		assert_eq!(Balances::free_balance(Public::from_string("/Alice_stash").ok().unwrap()), 2);
	});
}

#[test]
fn controversial_voting_should_work() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let bob = Public::from_string("/Bob").ok().unwrap();
		let charlie = Public::from_string("/Charlie").ok().unwrap();
		let dave = Public::from_string("/Dave").ok().unwrap();
		let eve = Public::from_string("/Eve").ok().unwrap();
		let ferdie = Public::from_string("/Ferdie").ok().unwrap();

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		let r = Democracy::inject_referendum(
			0,
			2,
			0,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);

		assert_ok!(Democracy::vote(RuntimeOrigin::signed(alice), 0, r, big_aye(alice)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(bob), 0, r, big_nay(bob)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(charlie), 0, r, big_nay(charlie)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(dave), 0, r, big_aye(dave)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(eve), 0, r, big_nay(eve)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(ferdie), 0, r, big_aye(ferdie)));

		assert_eq!(tally(0, r), Tally { ayes: 110, nays: 100, turnout: 210 });

		next_block();
		next_block();
		next_block();

		assert_eq!(Balances::free_balance(Public::from_string("/Alice_stash").ok().unwrap()), 2);
	});
}

#[test]
fn controversial_low_turnout_voting_should_work() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let eve = Public::from_string("/Eve").ok().unwrap();
		let ferdie = Public::from_string("/Ferdie").ok().unwrap();

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		let r = Democracy::inject_referendum(
			0,
			2,
			0,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(eve), 0, r, big_nay(eve)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(ferdie), 0, r, big_aye(ferdie)));

		assert_eq!(tally(0, r), Tally { ayes: 60, nays: 50, turnout: 110 });

		next_block();
		next_block();

		assert_eq!(Balances::free_balance(Public::from_string("/Alice_stash").ok().unwrap()), 0);
	});
}

#[test]
fn passing_low_turnout_voting_should_work() {
	new_test_ext().execute_with(|| {
		let alice = Public::from_string("/Alice").ok().unwrap();
		let dave = Public::from_string("/Dave").ok().unwrap();
		let eve = Public::from_string("/Eve").ok().unwrap();
		let ferdie = Public::from_string("/Ferdie").ok().unwrap();

		assert_ok!(create_dao(alice));
		assert_ok!(init_dao_token_accounts(0));

		let r = Democracy::inject_referendum(
			0,
			2,
			0,
			set_balance_proposal(2),
			VoteThreshold::SuperMajorityApprove,
			0,
		);
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(dave), 0, r, big_aye(dave)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(eve), 0, r, big_nay(eve)));
		assert_ok!(Democracy::vote(RuntimeOrigin::signed(ferdie), 0, r, big_aye(ferdie)));
		assert_eq!(tally(0, r), Tally { ayes: 100, nays: 50, turnout: 150 });

		next_block();
		next_block();
		next_block();

		assert_eq!(Balances::free_balance(Public::from_string("/Alice_stash").ok().unwrap()), 2);
	});
}
