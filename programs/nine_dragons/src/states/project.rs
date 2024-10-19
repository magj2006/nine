use anchor_lang::prelude::*;
use crate::error::NineDragonsError;

#[account]
#[derive(Default, InitSpace)]
pub struct Project {
    pub nonce: u64,
    pub price: u64,
    pub seller_fee_basis_points: u16,
    pub recipient: Pubkey,
    pub owner: Pubkey,
    pub original_owner: Pubkey,
    pub update_authority: Pubkey,
    pub collection_nft: Option<Pubkey>,
    pub pending_owner: Option<Pubkey>,
    #[max_len(30)]
    pub project_name: String,
    pub is_mutable: bool,
    pub operator: Pubkey,
    pub codes: Pubkey,
    pub codes2: Pubkey,
    pub codes3: Pubkey,
    #[max_len(500)]
    pub _padding: Vec<u8>
}

impl Project {
    pub const PROJECT_SEED_PREFIX: &'static [u8; 7] = b"project";

    pub const AUTHORITY_SEED_PREFIX: &'static [u8; 9] = b"authority";

    pub const CODES1_SEED_PREFIX: &'static [u8; 5] = b"code1";
    pub const CODES2_SEED_PREFIX: &'static [u8; 5] = b"code2";
    pub const CODES3_SEED_PREFIX: &'static [u8; 5] = b"code3";

    pub const PROJECT_NAME_LEN: u8 = 30;

    // pub const LEN: usize = 8 + 8 + 2 + 32 + 32 + 32 + 1 + 32 + 1 + 32 + 32 + 4 + 30 + 1 + 32 + 1 + 8 + 32 + 4 + 491;

    pub fn change_ownership(&mut self, new_owner: Pubkey) {
        self.pending_owner = Some(new_owner);
    }

    pub fn accept_ownership(&mut self, new_owner: Pubkey) {
        self.owner = new_owner;
        self.pending_owner = None;
    }

    pub fn set_price(&mut self, price: u64) {
        self.price = price;
    }

    pub fn set_recipient(&mut self, recipient: Pubkey) {
        self.recipient = recipient;
    }

    pub fn set_seller_fee_basis_points(&mut self, seller_fee_basis_points: u16) {
        self.seller_fee_basis_points = seller_fee_basis_points
    }
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionParam {
    pub name: String,  // 20
    pub symbol: String, // 10
    pub uri: String, // 80
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateNFTParam {
    pub name: String,  // 20
    pub symbol: String, // 10
    pub uri: String, // 80
    pub code: [u8; 8],
    pub index: u32
}

impl CreateCollectionParam {
     const NAME_LEN: usize = 20;
     const SYMBOL_LEN: usize = 10;
     const URI_LEN: usize = 80;
    pub fn validate(&self) -> Result<()> {
        require_gte!(CreateCollectionParam::NAME_LEN, self.name.len(), NineDragonsError::LongName);
        require_gte!(CreateCollectionParam::SYMBOL_LEN, self.symbol.len(), NineDragonsError::LongSymbol);
        require_gte!(CreateCollectionParam::URI_LEN, self.uri.len(), NineDragonsError::LongUri);

        Ok(())
    }
}

impl CreateNFTParam {
    const NAME_LEN: usize = 20;
    const SYMBOL_LEN: usize = 10;
    const URI_LEN: usize = 80;
    pub fn validate(&self) -> Result<()> {
        require_gte!(CreateNFTParam::NAME_LEN, self.name.len(), NineDragonsError::LongName);
        require_gte!(CreateNFTParam::SYMBOL_LEN, self.symbol.len(), NineDragonsError::LongSymbol);
        require_gte!(CreateNFTParam::URI_LEN, self.uri.len(), NineDragonsError::LongUri);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_bytes() {
        let x = 1u16.to_le_bytes();
        assert_eq!(x, [0x01, 0x00], "Bytes error");
    }
}
