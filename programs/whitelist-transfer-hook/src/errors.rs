use anchor_lang::error_code;

#[error_code]
pub enum WhitelistOpError {
    #[msg("This address is already in the whitelist")]
    AddressInWhitelist,
    #[msg("This address is not in the whitelist")]
    AddressNotInWhitelist,
}
