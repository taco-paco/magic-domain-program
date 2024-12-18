use std::net::{Ipv4Addr, SocketAddrV4};

use borsh::BorshDeserialize;
use common::TestEnv;
use mdp::state::validator_info::ValidatorInfo;
use sdk::account::Account;

pub mod common;

#[tokio::test]
async fn test_sync_info() {
    let TestEnv {
        mut banks,
        identity,
        info,
        ..
    } = common::setup().await;
    let pda = info.pda().0;

    let result = common::register(&mut banks, info, &identity).await;
    assert_ok!(result, "error processing register transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying registration PDA from banks {}");
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ValidatorInfo::try_from_slice(&acc.data);
    let mut info = assert_ok!(result, "error querying registration PDA from banks {}");

    const NEW_BLOCK_TIME: u16 = 1000;
    const NEW_IP: Ipv4Addr = Ipv4Addr::new(12, 78, 13, 224);
    const NEW_PORT: u16 = 23435;
    const NEW_ADDR: SocketAddrV4 = SocketAddrV4::new(NEW_IP, NEW_PORT);

    info.block_time_ms = NEW_BLOCK_TIME;
    info.addr = NEW_ADDR;

    let result = common::sync_info(&mut banks, &identity, info).await;
    assert_ok!(result, "error processing sync info transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(
        result,
        "error querying registration PDA after modification {}"
    );
    assert!(matches!(acc, Some(Account { owner: mdp::ID, .. })));
    let acc = acc.unwrap();
    let result = ValidatorInfo::try_from_slice(&acc.data);
    let info = assert_ok!(
        result,
        "error querying registration PDA post modification {}"
    );
    assert_eq!(info.addr, NEW_ADDR);
    assert_eq!(info.block_time_ms, NEW_BLOCK_TIME);

    let result = common::unregister(&mut banks, &identity, pda).await;

    assert_ok!(result, "error processing unregister transaction {}");

    let result = banks.get_account(pda).await;

    let acc = assert_ok!(result, "error querying unregistered PDA from banks {}");
    assert!(
        acc.is_none(),
        "registration PDA hasn't been removed from banks"
    );
}
