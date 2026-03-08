use {
    trezoa_keypair::Keypair, trezoa_program_test::ProgramTestContext, trezoa_pubkey::Pubkey,
    trezoa_signer::Signer, trezoa_system_interface::instruction::create_account,
    trezoa_transaction::Transaction,
};

pub async fn initialize(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> Pubkey {
    let account = Keypair::new();

    let account_size = 165;
    let rent = context.banks_client.get_rent().await.unwrap();

    let mut initialize_ix = tpl_token_interface::instruction::initialize_account(
        &tpl_token_interface::ID,
        &account.pubkey(),
        mint,
        owner,
    )
    .unwrap();
    initialize_ix.program_id = *program_id;

    let instructions = vec![
        create_account(
            &context.payer.pubkey(),
            &account.pubkey(),
            rent.minimum_balance(account_size),
            account_size as u64,
            program_id,
        ),
        initialize_ix,
    ];

    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&context.payer.pubkey()),
        &[&context.payer, &account],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    account.pubkey()
}

pub async fn approve(
    context: &mut ProgramTestContext,
    account: &Pubkey,
    delegate: &Pubkey,
    owner: &Keypair,
    amount: u64,
    program_id: &Pubkey,
) {
    let mut approve_ix = tpl_token_interface::instruction::approve(
        &tpl_token_interface::ID,
        account,
        delegate,
        &owner.pubkey(),
        &[],
        amount,
    )
    .unwrap();
    approve_ix.program_id = *program_id;

    let tx = Transaction::new_signed_with_payer(
        &[approve_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, owner],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();
}

pub async fn freeze(
    context: &mut ProgramTestContext,
    account: &Pubkey,
    mint: &Pubkey,
    freeze_authority: &Keypair,
    program_id: &Pubkey,
) {
    let mut freeze_account_ix = tpl_token_interface::instruction::freeze_account(
        &tpl_token_interface::ID,
        account,
        mint,
        &freeze_authority.pubkey(),
        &[],
    )
    .unwrap();
    freeze_account_ix.program_id = *program_id;

    let tx = Transaction::new_signed_with_payer(
        &[freeze_account_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, freeze_authority],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();
}
