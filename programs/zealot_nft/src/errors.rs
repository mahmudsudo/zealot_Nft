use anchor_lang::prelude::error_code;

#[error_code]
pub enum ONftError {
    Unauthorized,
    InvalidSender,
    InvalidDecimals,
    SlippageExceeded,
    InvalidTokenMint,
    InvalidTokenEscrow,
    InvalidTokenDest,
    InvalidOptions,
    InvalidEndpointProgram,
    RateLimitExceeded,
}
