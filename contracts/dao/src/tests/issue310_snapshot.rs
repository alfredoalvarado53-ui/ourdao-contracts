// ==================== issue #310: snapshot tests for exact event topics and payloads ====================
// Snapshot tests verifying the exact event topics and payload data for key events.
#[test]
fn loan_request_event_exact_payload() {
    let s = setup(3);
    let borrower = s.members.get(0).unwrap();
    let pid = s.client.request_loan(&borrower, &1_000, &None);

    let all = s.env.events().all();
    let event = all
        .events()
        .iter()
        .rev()
        .find(|e| {
            let ContractEventBody::V0(body) = &e.body;
            matches!(body.topics.first(), Some(ScVal::Symbol(sym)) if sym.0.to_utf8_string_lossy() == "loan_req")
        })
        .expect("loan_req event not found");
    let ContractEventBody::V0(body) = &event.body;
    let ScVal::Vec(Some(data)) = &body.data else {
        panic!("expected tuple data");
    };

    // Verify exact field order and count
    assert_eq!(data.len(), 5);
    assert_eq!(data[0], ScVal::U32(pid));
    assert_eq!(data[1], ScVal::Address(s.members.get(0).unwrap()));
    assert_eq!(data[2], ScVal::U32(1_000));
    assert_eq!(data[3], ScVal::Address(s.members.get(0).unwrap()));
    assert_eq!(data[4], ScVal::I32(0)); // asset is base token
}

#[test]
fn treasury_propose_event_exact_payload() {
    let s = setup(3);
    let proposer = s.members.get(0).unwrap();
    let dest = Address::generate(&s.env);
    let reason = String::from_str(&s.env, "test");
    let pid = s
        .client
        .propose_treasury_withdrawal(&proposer, &600, &dest, &reason, &false);

    let all = s.env.events().all();
    let event = all
        .events()
        .iter()
        .rev()
        .find(|e| {
            let ContractEventBody::V0(body) = &e.body;
            matches!(body.topics.first(), Some(ScVal::Symbol(sym)) if sym.0.to_utf8_string_lossy() == "tre_prop")
        })
        .expect("tre_prop event not found");
    let ContractEventBody::V0(body) = &event.body;
    let ScVal::Vec(Some(data)) = &body.data else {
        panic!("expected tuple data");
    };

    // Verify exact field order and count
    assert_eq!(data.len(), 4);
    assert_eq!(data[0], ScVal::U32(pid));
    assert_eq!(data[1], ScVal::U32(600));
    assert_eq!(data[2], ScVal::Address(dest));
    assert_eq!(data[3], ScVal::I32(0)); // private = false
}

#[test]
fn policy_update_proposal_event_exact_payload() {
    let s = setup(2);
    let mut policy = s.client.get_policy().unwrap();
    policy.max_interest_rate = 1_200;

    s.client.propose_policy_update(&s.admin, &policy);
    let all = s.env.events().all();
    let event = all
        .events()
        .iter()
        .rev()
        .find(|e| {
            let ContractEventBody::V0(body) = &e.body;
            matches!(body.topics.first(), Some(ScVal::Symbol(sym)) if sym.0.to_utf8_string_lossy() == "prop_pol")
        })
        .expect("prop_pol event not found");
    let ContractEventBody::V0(body) = &event.body;
    let ScVal::Vec(Some(data)) = &body.data else {
        panic!("expected tuple data");
    };

    // Verify exact field order and count
    assert_eq!(data.len(), 2);
    assert_eq!(data[0], ScVal::U32(1)); // policy id
    assert_eq!(data[1], ScVal::I32(1_200)); // max_interest_rate
}
