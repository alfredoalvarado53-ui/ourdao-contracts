# Pause Functionality Implementation

## Problem Statement
The original pause implementation only blocked 8 state-changing operations while allowing critical treasury-draining operations to continue. This inverted the intent of an emergency stop, which should freeze all value transfers when an exploit is discovered.

## Changes Made

### Added Pause Checks to Critical Operations:
1. `exit_dao` - prevents members from withdrawing treasury funds
2. `claim_rewards` - prevents distribution of yield earnings  
3. `stake` - prevents new stake deposits
4. `unstake` - prevents stake withdrawals
5. `edit_loan_proposal` - prevents modification of loan proposals
6. `mark_loan_defaulted` - prevents slashing member contributions
7. `register_name` - prevents name registry changes
8. `attach_document` - prevents proposal document changes
9. `disburse_approved_loan` - prevents loan disbursements
10. `execute_approved` (treasury) - prevents treasury withdrawals
11. `expire_loan_proposal` - prevents proposal expiration

### Intentional Exceptions (No Pause Check):
1. `repay_loan` / `repay_loan_partial` - Borrowers should be able to repay loans even during emergency pauses to avoid being forced into default through no fault of their own.

2. Admin functions (`unpause`, `add_admin`, `remove_admin`, `set_pauser`, `revoke_pauser`, `set_consensus_threshold`, `set_policy`) - Governance operations remain available to control the pause state and its authorities.

### Already Had Pause Checks:
1. `register_member`
2. `request_loan` 
3. `vote_on_loan_proposal`
4. `propose_withdrawal`
5. `vote` (treasury)
6. `commit_vote`
7. `reveal_vote`

## Technical Implementation
- All pause checks use `util::require_not_paused(env)?` which calls `storage::is_paused(env)`
- The `Error::Paused` error is returned when contract is paused
- Admin can call `pause()` and `unpause()`; the assigned pauser can call `pause()`

## Emergency Stop Behavior
An admin can assign one dedicated pauser with `set_pauser()` and remove that
assignment with `revoke_pauser()`. Only admins can change the pauser assignment.
The assigned pauser can call `pause()` without receiving any governance powers.
Admins can also call `pause()` directly. Only an admin can call `unpause()`.

### Pauser vs Admin Authority Separation (#319)
- **Pause Authority**: Both full DAO admins and the dedicated `pauser` role can trigger `pause()` to freeze value-draining operations during an exploit.
- **Unpause Authority**: The `pauser` role is strictly one-way. A designated pauser **CANNOT** call `unpause()`; attempting to do so returns `Error::NotAdmin`. Only full DAO admins can resume contract operations via `unpause()`.
- **Rationale**: An emergency pauser (such as an automated monitoring bot or lower-tier operational key) needs the capability to stop damage immediately without human delay, but should not possess the high-privilege authority to resume transactions until human admins investigate the incident and verify safety.

When `pause()` is called by an admin or the assigned pauser:
- All new value transfers out of the contract are blocked (exit_dao, claim_rewards, unstake, disburse_approved_loan, execute_approved)
- All state-changing operations are blocked (except repay_loan and admin functions)
- The contract can be resumed with `unpause()` only by an admin
- This ensures that if an exploit is discovered, admins can freeze the contract before value leaves

## Verification
All existing tests pass after implementing these changes. The pause functionality now correctly blocks all critical operations that could drain treasury or modify contract state during an emergency stop.