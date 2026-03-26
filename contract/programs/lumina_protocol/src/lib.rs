use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use solana_program::pubkey::Pubkey;

// Specific imports for the Transfer Hook Metadata
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    state::ExtraAccountMetaList,
    seeds::Seed, 
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

declare_id!("AQ1LR7RwRsuEFYdiPYvYX4U34x2DjBaTjK4s93wvoNgs");

#[program]
pub mod lumina {
    use super::*;

    /// 1. AUTHORIZE USER: Creates a PDA identity record for a user.
    /// Only the backend admin (Validator) should call this after KYC.
    pub fn authorize_user(ctx: Context<AuthorizeUser>, country_code: String) -> Result<()> {
        let identity = &mut ctx.accounts.identity_record;
        identity.authority = ctx.accounts.admin.key();
        identity.is_verified = true;
        identity.country_code = country_code;
        
        msg!("LUMINA: User {} authorized for region {}", ctx.accounts.user.key(), identity.country_code);
        Ok(())
    }

    /// 2. INITIALIZE METADATA: Configures the ExtraAccountMetaList.
    /// Tells Token-2022 to automatically look up the Identity PDAs during transfer.
    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        let account_metas = [
            // Index 5: Sender Identity (calculated from source_token.owner)
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal { bytes: b"identity".to_vec() },
                    Seed::AccountKey { index: 0 }, // 0 is source_token.owner
                ],
                false, 
                false, 
            )?,
            // Index 6: Receiver Identity (calculated from destination_token.owner)
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal { bytes: b"identity".to_vec() },
                    Seed::AccountKey { index: 2 }, // 2 is destination_token.owner
                ],
                false,
                false,
            )?,
        ];

        let data = &mut ctx.accounts.extra_metas_account.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(data, &account_metas)?;

        Ok(())
    }

    /// 3. TRANSFER HOOK: The enforcement logic.
    /// Automatically called by the network. If this fails, the transfer is blocked.
    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        let sender_identity = &ctx.accounts.sender_identity;
        let receiver_identity = &ctx.accounts.receiver_identity;

        require!(sender_identity.is_verified, LuminaError::UnverifiedSender);
        require!(receiver_identity.is_verified, LuminaError::UnverifiedReceiver);

        msg!("LUMINA: Compliance Verified. {} -> {}", 
            sender_identity.country_code, 
            receiver_identity.country_code
        );
        Ok(())
    }
}

// --- Data Structures ---

#[account]
pub struct IdentityRecord {
    pub authority: Pubkey,
    pub is_verified: bool,
    pub country_code: String,
}

// --- Contexts ---

#[derive(Accounts)]
pub struct AuthorizeUser<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + 1 + 8, 
        seeds = [b"identity", user.key().as_ref()], 
        bump
    )]
    pub identity_record: Account<'info, IdentityRecord>,
    pub user: SystemAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = ExtraAccountMetaList::size_of(2).unwrap(), 
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = payer
    )]
    /// CHECK: ExtraMetas account for Token-2022
    pub extra_metas_account: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferHook<'info> {
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: Owner of the source
    pub owner: UncheckedAccount<'info>,
    /// CHECK: The metadata account
    pub extra_metas_account: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"identity", source_token.owner.as_ref()], 
        bump,
        seeds::program = crate::ID
    )]
    pub sender_identity: Account<'info, IdentityRecord>,

    #[account(
        seeds = [b"identity", destination_token.owner.as_ref()], 
        bump,
        seeds::program = crate::ID
    )]
    pub receiver_identity: Account<'info, IdentityRecord>,
}

// --- Errors ---

#[error_code]
pub enum LuminaError {
    #[msg("Sender is not verified in the Identity Registry")]
    UnverifiedSender,
    #[msg("Receiver is not verified in the Identity Registry")]
    UnverifiedReceiver,
}