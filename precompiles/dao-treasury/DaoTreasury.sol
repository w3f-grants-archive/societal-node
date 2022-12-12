// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/// @title The interface through which solidity contracts will interact with Societal Blockchain
/// We follow this same interface including four-byte function selectors, in the precompile that
/// wraps the pallet
interface PalletDaoTreasury {

    /// @dev Propose Treasury Spend
    /// @param dao_id DAO ID
    /// @param value Balance amount to be spent
    /// @param beneficiary Account to transfer balance to
    function proposeSpend(uint32 dao_id, uint128 value, address beneficiary) external;

    /// @dev Get the number of proposals.
    ///
    /// @param dao_id DAO ID.
    /// @return proposalCount Number of proposals.
    function proposalCount(uint32 dao_id) external view returns (uint32 proposalCount);

    /// @dev Get indices of proposals that have been approved but not yet awarded.
    ///
    /// @param dao_id DAO ID.
    /// @return approvals indices of approved proposals.
    function approvals(uint32 dao_id) external view returns (uint32[] memory approvals);

    event Proposed(uint32 dao_id, uint32 indexed proposalIndex);
}
