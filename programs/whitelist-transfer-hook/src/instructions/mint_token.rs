use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::MintTo,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::authority = user,
        extensions::transfer_hook::program_id = crate::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, // source
    /// CHECK: ExtraAccountMetaList Account, will be checked by the transfer hook
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist", user.key().as_ref()], 
        bump
    )]
    pub blocklist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TokenFactory<'info> {
    pub fn init_mint(&mut self, _bumps: &TokenFactoryBumps) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program_id = &self.token_program;
        let cpi_context = CpiContext::new(cpi_program_id.to_account_info(), cpi_accounts);
        token_interface::mint_to(cpi_context, 1_000_000_000)?;

        Ok(())
    }
}
