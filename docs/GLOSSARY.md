# OurDAO glossary

| Term | Meaning |
| **Admin** | An address authorized to change DAO configuration and pause or unpause the contract. |
| **Commit-reveal** | Voting flow where a hash commitment hides a vote until the voter later reveals the vote and salt. |
| **Consensus threshold** | Basis-point percentage of voting power required to approve a proposal (for example, 5100 = 51%). |
| **Content hash** | An on-chain digest identifying off-chain proposal material, such as an IPFS document. |
| **DAO token** | The single Stellar token configured at initialization for membership fees, loans, and treasury accounting. |
| **Delegation** | Authorization allowing an active member to assign their voting weight to another eligible member. |
| **Editing phase** | The initial fixed period during which a borrower may edit a loan proposal. |
| **Keeper** | An unprivileged caller that submits an objective maintenance transition, such as marking an overdue loan defaulted. |
| **Keeper pass** | A numbered maintenance run in `scripts/keeper.sh` executing deterministic lifecycle state transitions. |
| **Loan policy** | Governance configuration controlling loan limits, interest, duration, grace periods, and default penalties. |
| **Loan proposal** | A member’s request for a loan before it is approved and disbursed. Its identifier is reused by the resulting loan. |
| **Member stake** | A member’s deposited membership balance, used in exit-share accounting and (when separately staked) voting boost. |
| **Pending policy update** | A queued configuration change subject to a mandatory timelock delay before activation. |
| **Pending yield** | Interest earned by an active member and claimable through `claim_rewards`. |
| **Reputation score** | Historical member credit metric tracking repayment fidelity across past loan cycles. |
| **SAC** | Stellar Asset Contract, the Soroban interface for native XLM and Stellar assets. |
| **Soroban** | Stellar’s smart-contract platform and execution environment used by this repository. |
| **Staking boost** | The capped additional voting weight earned by locking tokens through `stake`; it is excluded from lendable treasury funds. |
| **Treasury** | Lendable DAO funds after excluding member stakes and other reserved balances. |
| **TTL** | Soroban persistent-storage time-to-live. Reads and writes refresh entries to prevent active state from being archived. |
| **Vote batch** | An atomic contract entrypoint (`vote_batch`) allowing a voter to submit multiple proposal decisions in one call. |
| **Voting phase** | The period during which eligible members can vote on a loan or treasury proposal. |
| **WASM** | The optimized WebAssembly artifact deployed as the Soroban contract. |
| **Whitelisted asset** | A Stellar token address authorized by governance for multi-asset treasury deposits and balances. |
