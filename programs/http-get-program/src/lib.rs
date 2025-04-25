use {
    rialo_http_get_interface::{HttpGetResponse, PROGRAM_ID},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    std::str::FromStr,
};

entrypoint!(process_instruction);

/// Process the instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Verify the program ID
    if program_id != &Pubkey::from_str(PROGRAM_ID).unwrap() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the account iterator
    let account_info_iter = &mut accounts.iter();

    // Get the response account
    let response_account = next_account_info(account_info_iter)?;

    // Verify the response account is writable
    if !response_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Get the URL from the instruction data
    let url = String::from_utf8(instruction_data.to_vec())
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Make the HTTP GET request
    let response = solana_program::http_get(&url)
        .map_err(|_| ProgramError::Custom(1))?;

    // Create the response data
    let response_data = HttpGetResponse::new(200, response);

    // Serialize the response data
    let serialized = bincode::serialize(&response_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Write the response data to the account
    response_account.data.borrow_mut().copy_from_slice(&serialized);

    msg!("HTTP GET request completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_program::{
            account_info::AccountInfo,
            clock::Epoch,
            program_error::ProgramError,
            pubkey::Pubkey,
            rent::Rent,
            sysvar::Sysvar,
        },
        std::cell::RefCell,
    };

    #[test]
    fn test_process_instruction() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let response_account = Pubkey::new_unique();
        let url = "http://example.com".to_string();

        let response_account_info = AccountInfo::new(
            &response_account,
            false,
            true,
            &mut 0,
            &mut RefCell::new(vec![0; 1024]),
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = vec![response_account_info];
        let instruction_data = url.as_bytes();

        let result = process_instruction(&program_id, &accounts, instruction_data);
        assert!(result.is_ok());
    }
}