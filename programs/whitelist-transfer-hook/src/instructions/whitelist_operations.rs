use anchor_lang::prelude::*;

use crate::{errors::WhitelistOpError, state::whitelist::Whitelist};

#[derive(Accounts)]
#[instruction(user_address: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(
        mut,
        // address =
    )]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [b"whitelist", user_address.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(
        &mut self,
        user_address: Pubkey,
        bumps: &WhitelistOperationsBumps,
    ) -> Result<()> {
        let whitelist_account = &mut self.whitelist;

        // Check account is not in whitelist before add attempt
        if whitelist_account.in_whitelist {
            return err!(WhitelistOpError::AddressInWhitelist);
        }

        whitelist_account.set_inner(Whitelist {
            address: user_address,
            bump: bumps.whitelist,
            in_whitelist: true,
        });

        msg!("User address {} added to whitelist!", user_address);

        Ok(())
    }

    pub fn remove_from_whitelist(&mut self, user_address: Pubkey) -> Result<()> {
        let whitelist_account = &mut self.whitelist;

        // Check account is in whitelist before remove attempt
        if !whitelist_account.in_whitelist {
            return err!(WhitelistOpError::AddressNotInWhitelist);
        }

        whitelist_account.in_whitelist = false;

        msg!("User address {} removed from whitelist!", user_address);

        Ok(())
    }
}
