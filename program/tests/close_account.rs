mod setup;

use {
    mollusk_svm::{result::Check, Mollusk},
    trezoa_account::{Account as TrezoaAccount, ReadableAccount},
    trezoa_program_error::ProgramError,
    trezoa_program_pack::Pack,
    trezoa_pubkey::Pubkey,
    trezoa_sdk_ids::system_program,
    trezoa_system_interface::instruction::{create_account, transfer},
    tpl_token_interface::{instruction, state::Account},
};

#[test]
fn success_init_after_close_account() {
    let mollusk = Mollusk::new(&tpl_token_interface::id(), "tpl_token");

    let owner = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let account = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let decimals = 9;

    let owner_account = TrezoaAccount::new(1_000_000_000, 0, &system_program::id());
    let mint_account = setup::setup_mint_account(None, None, 0, decimals);
    let token_account = setup::setup_token_account(&mint, &owner, 0);

    let expected_destination_lamports = token_account.lamports();

    mollusk.process_and_validate_instruction_chain(
        &[
            (
                &instruction::close_account(
                    &tpl_token_interface::id(),
                    &account,
                    &destination,
                    &owner,
                    &[],
                )
                .unwrap(),
                &[Check::success()],
            ),
            (
                &create_account(
                    &owner,
                    &account,
                    1_000_000_000,
                    Account::LEN as u64,
                    &tpl_token_interface::id(),
                ),
                &[Check::success()],
            ),
            (
                &instruction::initialize_account(
                    &tpl_token_interface::id(),
                    &account,
                    &mint,
                    &owner,
                )
                .unwrap(),
                &[
                    Check::success(),
                    // Account successfully re-initialized.
                    Check::account(&account)
                        .data(setup::setup_token_account(&mint, &owner, 0).data())
                        .owner(&tpl_token_interface::id())
                        .build(),
                    // The destination should have the lamports from the closed account.
                    Check::account(&destination)
                        .lamports(expected_destination_lamports)
                        .build(),
                ],
            ),
        ],
        &[
            (mint, mint_account),
            (account, token_account),
            (owner, owner_account),
            (destination, TrezoaAccount::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
        ],
    );
}

#[test]
fn fail_init_after_close_account() {
    let mollusk = Mollusk::new(&tpl_token_interface::id(), "tpl_token");

    let owner = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let account = Pubkey::new_unique();
    let destination = Pubkey::new_unique();
    let decimals = 9;

    let owner_account = TrezoaAccount::new(1_000_000_000, 0, &system_program::id());
    let mint_account = setup::setup_mint_account(None, None, 0, decimals);
    let token_account = setup::setup_token_account(&mint, &owner, 0);

    let expected_destination_lamports = token_account.lamports();

    mollusk.process_and_validate_instruction_chain(
        &[
            (
                &instruction::close_account(
                    &tpl_token_interface::id(),
                    &account,
                    &destination,
                    &owner,
                    &[],
                )
                .unwrap(),
                &[Check::success()],
            ),
            (
                &transfer(&owner, &account, 1_000_000_000),
                &[Check::success()],
            ),
            (
                &instruction::initialize_account(
                    &tpl_token_interface::id(),
                    &account,
                    &mint,
                    &owner,
                )
                .unwrap(),
                &[
                    Check::err(ProgramError::InvalidAccountData),
                    // Account not re-initialized.
                    Check::account(&account)
                        .lamports(1_000_000_000)
                        .owner(&system_program::id())
                        .build(),
                    // The destination should have the lamports from the closed account.
                    Check::account(&destination)
                        .lamports(expected_destination_lamports)
                        .build(),
                ],
            ),
        ],
        &[
            (mint, mint_account),
            (account, token_account),
            (owner, owner_account),
            (destination, TrezoaAccount::default()),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
        ],
    );
}
