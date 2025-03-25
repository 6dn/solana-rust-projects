use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
    system_instruction,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let transfer_instruction = system_instruction::transfer(
        from_account.key,
        to_account.key,
        1000000, // lamports
    );

    invoke_signed(
        &transfer_instruction,
        &[from_account.clone(), to_account.clone(), system_program.clone()],
        &[],
    )?;

    Ok(())
}
