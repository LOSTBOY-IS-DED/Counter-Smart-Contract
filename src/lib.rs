use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

// This macro exposes `counter_contract` as the program's entrypoint
entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,      // ✅ Reference type required by entrypoint macro
    accounts: &[AccountInfo], // ✅ List of accounts to interact with
    instruction_data: &[u8],  // ✅ Serialized instruction data (inc/dec)
) -> ProgramResult {
    // Get the account to operate on
    let acc: &AccountInfo = next_account_info(&mut accounts.iter())?;

    // Decode the instruction
    let instruction_type: InstructionType = InstructionType::try_from_slice(instruction_data)?;
    // Load counter state from account data
    let mut counter_data: Counter = Counter::try_from_slice(&acc.data.borrow())?;

    // Match and execute the instruction
    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Executing increase");
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("Executing decrease");
            counter_data.count -= value;
        }
    }

    // Save updated state back to account
    counter_data.serialize(&mut &mut acc.data.borrow_mut()[..])?;

    msg!("Contract succeeded");
    Ok(())
}
