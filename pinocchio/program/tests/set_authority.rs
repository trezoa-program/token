mod setup;

use {
    setup::{mint, TOKEN_PROGRAM_ID},
    trezoa_keypair::Keypair,
    trezoa_program_option::COption,
    trezoa_program_pack::Pack,
    trezoa_program_test::{tokio, ProgramTest},
    trezoa_pubkey::Pubkey,
    trezoa_signer::Signer,
    trezoa_transaction::Transaction,
    tpl_token_interface::instruction::AuthorityType,
};

#[tokio::test]
async fn set_authority() {
    let mut context = ProgramTest::new("pinocchio_token_program", TOKEN_PROGRAM_ID, None)
        .start_with_context()
        .await;

    // Given a mint account.

    let mint_authority = Keypair::new();
    let freeze_authority = Keypair::new();

    let mint = mint::initialize(
        &mut context,
        mint_authority.pubkey(),
        Some(freeze_authority.pubkey()),
        &TOKEN_PROGRAM_ID,
    )
    .await
    .unwrap();

    // When we set a new freeze authority.

    let new_authority = Pubkey::new_unique();

    let set_authority_ix = tpl_token_interface::instruction::set_authority(
        &tpl_token_interface::ID,
        &mint,
        Some(&new_authority),
        AuthorityType::FreezeAccount,
        &freeze_authority.pubkey(),
        &[],
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[set_authority_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &freeze_authority],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then the account should have the delegate and delegated amount.

    let account = context.banks_client.get_account(mint).await.unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    let mint = tpl_token_interface::state::Mint::unpack(&account.data).unwrap();

    assert!(mint.freeze_authority == COption::Some(new_authority));
}
