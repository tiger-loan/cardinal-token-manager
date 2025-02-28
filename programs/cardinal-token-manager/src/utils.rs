use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn assert_payment_token_account<'info, 'a: 'info>(
    token_account: &Account<'info, TokenAccount>, 
    token_manager: &Account<'info, TokenManager>, 
    remaining_accounts: &mut std::slice::Iter<'a, AccountInfo<'info>>) 
    -> Result<()> 
{
    if token_manager.receipt_mint.is_none() {
        if token_account.owner != token_manager.issuer {
            return Err(error!(ErrorCode::InvalidIssuer));
        }
    } else {
        let receipt_token_account_info = next_account_info(remaining_accounts)?;
        let receipt_token_account = Account::<TokenAccount>::try_from(receipt_token_account_info)?;
        if !(receipt_token_account.mint == token_manager.receipt_mint.expect("No receipt mint") && receipt_token_account.amount > 0) {
            return Err(error!(ErrorCode::InvalidReceiptMint));
        }
        if receipt_token_account.owner != token_account.owner {
            return Err(error!(ErrorCode::InvalidReceiptMintOwner));
        }
    }
    Ok(())
}

pub fn assert_derivation(program_id: &Pubkey, account: &AccountInfo, path: &[&[u8]]) -> Result<u8> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account.key {
        return Err(ErrorCode::DerivedKeyInvalid.into());
    }
    Ok(bump)
}
