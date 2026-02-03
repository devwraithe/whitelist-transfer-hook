use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // Derive the whitelist PDA using our program ID
        // let (whitelist_pda, _bump) = Pubkey::find_program_address(
        //     &[b"whitelist"],
        //     &ID
        // );

        // Vec of extra account metas the trf hook needs
        Ok(vec![
            // Whitelisted sender PDA
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: b"whitelist".to_vec(),
                    },
                    Seed::AccountData {
                        account_index: 0, // source token account (from trf hook ixn)
                        data_index: 32,   // owner field (bytes 32 - 63 is owner address)
                        length: 32,       // pubkey length (obvi)
                    },
                ],
                false,
                false,
            )
            .unwrap(), // owner token account
            // Whitelisted receiver PDA
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: b"whitelist".to_vec(),
                    },
                    Seed::AccountData {
                        account_index: 2, // destination token account (from trf hook ixn)
                        data_index: 32,   // owner field (bytes 32 - 63 is owner address)
                        length: 32,       // pubkey length (obvi)
                    },
                ],
                false,
                false,
            )
            .unwrap(),
        ])
    }
}
