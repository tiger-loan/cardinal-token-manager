pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GNkEMLkiC5Da6wyKohZTPLNhWs6X8Lbm3afwBRTBs8aN");

#[program]
pub mod cardinal_token_manager {
    use super::*;

    pub fn init(ctx: Context<InitCtx>, ix: InitIx) -> Result<()> {
        init::handler(ctx, ix)
    }

    pub fn uninit(ctx: Context<UninitCtx>) -> Result<()> {
        uninit::handler(ctx)
    }

    pub fn init_mint_counter(ctx: Context<InitMintCounterCtx>, mint: Pubkey) -> Result<()> {
        init_mint_counter::handler(ctx, mint)
    }

    pub fn set_claim_approver(ctx: Context<SetClaimApproverCtx>, claim_approver: Pubkey) -> Result<()> {
        set_claim_approver::handler(ctx, claim_approver)
    }

    pub fn set_transfer_authority(ctx: Context<SetTransferAuthorityCtx>, transfer_authority: Pubkey) -> Result<()> {
        set_transfer_authority::handler(ctx, transfer_authority)
    }

    pub fn add_invalidator(ctx: Context<AddInvalidatorCtx>, invalidator: Pubkey) -> Result<()> {
        add_invalidator::handler(ctx, invalidator)
    }

    pub fn replace_invalidator(ctx: Context<ReplaceInvalidatorCtx>, new_invalidator: Pubkey) -> Result<()> {
        replace_invalidator::handler(ctx, new_invalidator)
    }

    pub fn create_claim_receipt(ctx: Context<CreateClaimReceiptCtx>, target: Pubkey) -> Result<()> {
        create_claim_receipt::handler(ctx, target)
    }

    pub fn claim_receipt_mint(ctx: Context<ClaimReceiptMintCtx>, name: String) -> Result<()> {
        claim_receipt_mint::handler(ctx, name)
    }

    pub fn issue<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, IssueCtx<'info>>) -> Result<()> {
        issue::handler(ctx)
    }

    pub fn unissue<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, UnissueCtx<'info>>) -> Result<()> {
        unissue::handler(ctx)
    }

    pub fn claim<'key, 'accounts, 'info, 'remaining: 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, ClaimCtx<'info>>) -> Result<()> {
        claim::handler(ctx)
    }

    pub fn invalidate<'key, 'accounts, 'info, 'remaining: 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, InvalidateCtx<'info>>) -> Result<()> {
        invalidate::handler(ctx)
    }

    pub fn update_invalidation_type(ctx: Context<UpdateInvalidationTypeCtx>, invalidation_type: u8) -> Result<()> {
        update_invalidation_type::handler(ctx, invalidation_type)
    }

    pub fn create_mint_manager(ctx: Context<CreateMintManagerCtx>) -> Result<()> {
        create_mint_manager::handler(ctx)
    }

    pub fn close_mint_manager(ctx: Context<CloseMintManagerCtx>) -> Result<()> {
        close_mint_manager::handler(ctx)
    }

    // transfers
    pub fn create_transfer_receipt(ctx: Context<CreateTransferReceiptCtx>, target: Pubkey) -> Result<()> {
        transfers::create_transfer_receipt::handler(ctx, target)
    }

    pub fn update_transfer_receipt(ctx: Context<UpdateTransferReceiptCtx>, target: Pubkey) -> Result<()> {
        transfers::update_transfer_receipt::handler(ctx, target)
    }

    pub fn close_transfer_receipt(ctx: Context<CloseTransferReceiptCtx>) -> Result<()> {
        transfers::close_transfer_receipt::handler(ctx)
    }

    pub fn transfer<'key, 'accounts, 'info, 'remaining: 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, TransferCtx<'info>>) -> Result<()> {
        transfers::transfer::handler(ctx)
    }

    // permissioned token instructions
    pub fn send(ctx: Context<SendCtx>) -> Result<()> {
        permissioned::send::handler(ctx)
    }

    pub fn delegate(ctx: Context<DelegateCtx>) -> Result<()> {
        permissioned::delegate::handler(ctx)
    }

    pub fn undelegate(ctx: Context<UndelegateCtx>) -> Result<()> {
        permissioned::undelegate::handler(ctx)
    }

    pub fn migrate(ctx: Context<MigrateCtx>) -> Result<()> {
        permissioned::migrate::handler(ctx)
    }
}
