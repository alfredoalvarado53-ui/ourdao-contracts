// ==================== issue #313: loan threshold vs treasury_threshold ====================
// Tests that the loan proposal threshold and treasury threshold are applied to the
// correct proposal types. Loan proposals use `quorum_bps` for approval, while
// treasury withdrawals use `treasury_threshold`.
#[test]
fn loan_proposal_uses_quorum_bps_not_treasury_threshold() {
    let s = setup(3); // treasury = 3000, quorum = 51% = 1530 required
    let borrower = s.members.get(0).unwrap();
    let v1 = s.members.get(1).unwrap();
    let v2 = s.members.get(2).unwrap();

    let pid = s.client.request_loan(&borrower, &1_000, &None);
    advance(&s.env, EDITING + 1);

    // A single vote (1/3) should NOT approve a loan - quorum is 51% (ceil(1530))
    s.client.vote_on_loan_proposal(&v1, &pid, &true);
    let prop = s.client.get_loan_proposal(&pid).unwrap();
    assert_eq!(prop.status, ProposalStatus::Pending);

    // Two votes (2/3 = 66% > 51%) should approve
    s.client.vote_on_loan_proposal(&v2, &pid, &true);
    let prop = s.client.get_loan_proposal(&pid).unwrap();
    assert_eq!(prop.status, ProposalStatus::Approved);
}

#[test]
fn treasury_withdrawal_uses_treasury_threshold_not_quorum_bps() {
    let s = setup(3); // treasury = 3000, quorum = 51%, treasury_threshold = 5100
    let proposer = s.members.get(0).unwrap();
    let v1 = s.members.get(1).unwrap();
    let v2 = s.members.get(2).unwrap();
    let dest = Address::generate(&s.env);
    let reason = String::from_str(&s.env, "test");

    // 1000 amount is below treasury_threshold (5100), so it should approve easily
    let pid = s
        .client
        .propose_treasury_withdrawal(&proposer, &1_000, &dest, &reason, &false);
    advance(&s.env, 1);

    // One vote should be enough since amount < treasury_threshold
    s.client.vote_on_treasury_proposal(&v1, &pid, &true);
    let prop = s.client.get_treasury_proposal(&pid).unwrap();
    assert_eq!(prop.status, ProposalStatus::ApprovedPendingDisbursement);

    // Executive should succeed (amount is small)
    s.client.execute_treasury_proposal(&pid);
    assert_eq!(s.token.balance(&dest), 1_000);
}

#[test]
fn treasury_withdrawal_above_treasury_threshold_requires_quorum() {
    let s = setup(4); // treasury = 4000, 3 of 4 votes = 75%
    // Set a lower treasury threshold for testing (this is a test-only scenario)
    let mut policy = s.client.get_policy().unwrap();
    policy.treasury_threshold = 2_000; // 50% of 4000
    s.client.set_loan_policy(&s.admin, &policy);

    let proposer = s.members.get(0).unwrap();
    let v1 = s.members.get(1).unwrap();
    let v2 = s.members.get(2).unwrap();
    let v3 = s.members.get(3).unwrap();
    let dest = Address::generate(&s.env);
    let reason = String::from_str(&s.env, "test");

    // Amount 2500 > treasury_threshold 2000, needs more than half
    let pid = s
        .client
        .propose_treasury_withdrawal(&proposer, &2_500, &dest, &reason, &false);
    advance(&s.env, 1);

    // 2 votes (2/4 = 50%) is NOT enough since we need > 50%
    s.client.vote_on_treasury_proposal(&v1, &pid, &true);
    s.client.vote_on_treasury_proposal(&v2, &pid, &true);
    let prop = s.client.get_treasury_proposal(&pid).unwrap();
    assert_eq!(prop.status, ProposalStatus::Pending);

    // 3 votes (3/4 = 75%) IS enough
    s.client.vote_on_treasury_proposal(&v3, &pid, &true);
    let prop = s.client.get_treasury_proposal(&pid).unwrap();
    assert_eq!(prop.status, ProposalStatus::ApprovedPendingDisbursement);

    s.client.execute_treasury_proposal(&pid);
    assert_eq!(s.token.balance(&dest), 2_500);
}
