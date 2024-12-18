use std::{net::SocketAddrV4, str::FromStr};

use mdp::{
    instructions::{
        register::RegisterInstruction, sync::SyncInfoInstruction,
        unregister::UnregisterInstruction, Instruction,
    },
    state::{
        features::{Feature, FeaturesSet},
        validator_info::ValidatorInfo,
    },
};
use program_test::{BanksClient, BanksClientError, ProgramTest};
use sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction as SolanaInstruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

pub struct TestEnv {
    pub banks: BanksClient,
    pub identity: Keypair,
    pub info: ValidatorInfo,
}

pub async fn setup() -> TestEnv {
    const PROGNAME: &str = "mdp";
    std::env::set_var("SBF_OUT_DIR", "target/deploy/");
    let mut test = ProgramTest::new(PROGNAME, mdp::ID, None);
    let identity = Keypair::new();

    test.add_account(
        identity.pubkey(),
        Account::new(LAMPORTS_PER_SOL, 0, &system_program::ID),
    );

    let features = FeaturesSet::default().activate(Feature::Randomness);
    let info = ValidatorInfo {
        identity: identity.pubkey(),
        addr: SocketAddrV4::from_str("241.132.2.41:9324").unwrap(),
        block_time_ms: 50,
        fees: 1000,
        features,
    };
    let (banks, _, _) = test.start().await;

    TestEnv {
        banks,
        identity,
        info,
    }
}

pub async fn register(
    banks: &mut BanksClient,
    info: ValidatorInfo,
    identity: &Keypair,
) -> Result<(), BanksClientError> {
    let pda = info.pda().0;
    let ix = Instruction::Register(RegisterInstruction(info));
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[identity], hash);
    banks.process_transaction(tx).await
}

pub async fn unregister(
    banks: &mut BanksClient,
    identity: &Keypair,
    pda: Pubkey,
) -> Result<(), BanksClientError> {
    let ix = Instruction::Unregister(UnregisterInstruction(identity.pubkey()));
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    banks.process_transaction(tx).await
}

pub async fn sync_info(
    banks: &mut BanksClient,
    identity: &Keypair,
    info: ValidatorInfo,
) -> Result<(), BanksClientError> {
    let pda = info.pda().0;
    let ix = Instruction::SyncInfo(SyncInfoInstruction {
        identity: info.identity,
        addr: Some(info.addr),
        block_time_ms: Some(info.block_time_ms),
        fees: Some(info.fees),
        features: Some(info.features),
    });
    let ix = SolanaInstruction::new_with_borsh(
        mdp::ID,
        &ix,
        vec![
            AccountMeta::new(identity.pubkey(), true),
            AccountMeta::new(pda, false),
        ],
    );
    let hash = banks.get_latest_blockhash().await.unwrap();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&identity.pubkey()), &[&identity], hash);
    banks.process_transaction(tx).await
}

#[macro_export]
macro_rules! assert_ok {
    ($result: ident, $errmsg: expr) => {{
        assert!($result.is_ok(), $errmsg, $result.unwrap_err());
        $result.unwrap()
    }};
}
