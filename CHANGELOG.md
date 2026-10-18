# Changelog

All notable changes to `ourdao-contracts` are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to follow [Semantic Versioning](https://semver.org/). The workspace version in `Cargo.toml` has been `0.1.0` since the first commit; entries below are grouped under **Unreleased** until a version is cut, at which point the version in `Cargo.toml` should be bumped alongside a dated heading here.

## [Unreleased]

### Added
- Regression coverage ensuring `get_version` stays aligned with the Cargo package version (#372).
- `repay_loan_partial` for partial loan repayment.
- Loan and proposal lifecycle expiry, a `has_voted` view, name validation and an O(1) pull-based yield accumulator.
- `expire_treasury_proposal`, exposed in `lib.rs`; treasury proposals now expire after `VOTING_PERIOD`.
- Commit-reveal phase guards for private proposals (#57).
- `NotEditingPhase` and `DocumentTooLarge` errors; document hashes are limited to 64 bytes (#58, #59).
- `YieldRemainder` storage: `distribute_interest` carries rounding remainders and emits an event (#60).
- Loan terms are re-quoted at disbursement (#61).
- `bench-cost.sh` and a budget scaling benchmark; resource costs and the membership ceiling documented (#138).
- `generate-abi.sh` for a wasm-derived interface (#135).
- Property tests for exit-share, interest and staking-weight math.
- `util::isqrt`: an overflow-free integer square root over the whole `i128` range, backing the quadratic staking curve (#182).
- CI: `cargo clippy`, `cargo audit` and a `cargo-llvm-cov` coverage floor of 95% (#62).
- CI: MSRV job tests minimum supported Rust version (1.84) across all targets (#381).
- CI: markdown link checker validates all `.md` files for broken references (#383).
- PR template: checkboxes for "single issue only" and "ABI/event changes" (#382).

### Changed
- Pending yield has a single definition (`membership::compute_pending_yield`) shared by `get_pending_yield`, `claim_rewards` and `exit_dao` (#92).
- Loan proposals that time out without quorum are now stored as `status = Expired` (previously `Rejected`), matching `phase` and the `loan_exp` event; early rejection stays `Rejected` / `loan_rej` (#86).
- Defaulting a loan forfeits the borrower's unclaimed yield; yield stays equal-split and exit share stays contribution-weighted, now documented in the README (#89).
- **ABI:** config and registry views (`get_token`, `get_admins`, `is_admin`, `get_loan_policy`, `get_consensus_threshold`, `get_treasury_balance`, `calculate_loan_terms`, `calculate_exit_share`, `resolve_name`, `name_of`, `get_document`) now return `Result` and yield `NotInitialized` on an uninitialized contract; new `is_initialized` view (#88).
- `Member.join_ledger` renamed to `join_time` (#55).
- Staking boost is quadratic instead of linear: `voting_weight` is now `1 + min(isqrt(stake / 100), 5)`, so the *k*-th bonus vote costs `k² × 100` staked tokens and a whale can no longer buy proportional control (#182).
- Admin policy bounds tightened (#53, #54); loan eligibility checks and the `is_eligible_for_loan` signature corrected (#52, #56).
- Proposal document attachment is restricted to the proposal's own owner, and rejected after the editing phase (#59).

### Fixed
- `total_members` is now decremented on exit, so exit shares no longer underpay (#11).
- Stake locking and `due_time` on loan approval (#46, #51); reveal period end (#45).
- CI workflow and formatting violations.

### Documentation
- `SECURITY.md` with a coordinated disclosure policy.
- `docs/MIGRATION.md`, `docs/INVARIANTS.md`, `docs/LOAN_POLICY_GUIDE.md` and the pause/unpause incident runbook.
- `docs/THREAT_MODEL.md`: actors, trust assumptions and known gaps (#133).
