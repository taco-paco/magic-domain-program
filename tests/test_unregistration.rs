use common::TestEnv;

pub mod common;

#[tokio::test]
async fn test_unregistration() {
    let TestEnv {
        mut banks,
        identity,
        info,
        ..
    } = common::setup().await;
    let pda = info.pda().0;

    let result = common::register(&mut banks, info, &identity).await;
    assert_ok!(result, "error processing register transaction {}");

    let result = common::unregister(&mut banks, &identity, pda).await;

    assert_ok!(result, "error processing unregister transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying unregistered PDA from banks {}");
    assert!(
        acc.is_none(),
        "registration PDA hasn't been removed from banks"
    );
}
