pub mod instructions;
pub mod states;

pub use instructions::*;
pub use states::*;

declare_id!("3c3UWjBKDVCCXW78X7GwqSZmLCkYnw96oqUn2RibJMQ4");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        collection_id: Pubkey,
        duration: i64,
    ) -> Result<()> {
        instructions::create_pool::handler(ctx, collection_id, duration)
    }

    pub fn offer_loan(ctx: Context<OfferLoan>, offer_amount: u64) -> Result<()> {
        instructions::offer_loan::handler(ctx, offer_amount)
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}
