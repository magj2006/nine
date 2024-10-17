use anchor_lang::error_code;

#[error_code]
pub enum NineDragonsError {
    #[msg("Not project owner")]
    NotProjectOwner,
    #[msg("Invalid receipt")]
    InvalidReceipt,
    #[msg("Not new project owner")]
    NotNewProjectOwner,
    #[msg("Create collection first")]
    CreateCollectionFirst,
    #[msg("Invalid collection mint")]
    InvalidCollection,
    #[msg("Should supply collection")]
    EmptyCollection,
    #[msg("The name is longer than 50")]
    LongName,
    #[msg("The symbol is longer than 50")]
    LongSymbol,
    #[msg("The uri is longer than 150")]
    LongUri,
    #[msg("size is zero")]
    SizeZero,
    #[msg("Not allowed operator")]
    NotAllowedOperator,
    #[msg("More than limit")]
    MoreThanLimit,
    #[msg("Invalid code")]
    InvalidCode,
    #[msg("Invalid codes account")]
    InvalidCodesAccount
}
