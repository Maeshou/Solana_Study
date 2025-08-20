use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, program::invoke_signed, instruction::{Instruction, AccountMeta}};

declare_id!("M1n1MarketF66666666666666666666666666666");

#[program]
pub mod mini_market_f {
    use super::*;

    pub fn init_store(ctx: Context<InitStore>, base_fee: u16) -> Result<()> {
        let st = &mut ctx.accounts.store;
        st.owner = ctx.accounts.merchant.key();
        st.fee_bps = (base_fee % 900) + 50;
        st.sales = base_fee as u32 / 4 + 5;
        st.reviews = 3;
        Ok(())
    }

    // 手動 bump を別PDA escrow_drawer に使用し、自己呼び出し風に invoke_signed
    pub fn place_order(ctx: Context<PlaceOrder>, price: u64, user_bump: u8) -> Result<()> {
        let st = &mut ctx.accounts.store;

        let seeds = &[b"escrow_drawer", ctx.accounts.merchant.key.as_ref(), &[user_bump]];
        let drawer = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(StoreErr::SeedBad))?;
        if drawer != ctx.accounts.escrow_drawer.key() {
            return Err(error!(StoreErr::DrawerMismatch));
        }

        let ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![
                AccountMeta::new(ctx.accounts.store.key(), false),
                AccountMeta::new_readonly(ctx.accounts.merchant.key(), true),
            ],
            data: st.fee_bps.to_le_bytes().to_vec(),
        };
        let signer = &[b"escrow_drawer", ctx.accounts.merchant.key.as_ref(), &[user_bump]];
        invoke_signed(
            &ix,
            &[ctx.accounts.store.to_account_info(), ctx.accounts.merchant.to_account_info()],
            &[signer],
        )?;

        let taxed = price.saturating_sub((price * st.fee_bps as u64) / 10_000);
        st.sales = st.sales.saturating_add((taxed % 10_000) as u32);
        if st.sales % 5 != 4 { st.reviews = st.reviews.saturating_add(1); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStore<'info> {
    #[account(
        init, payer = merchant, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"store", merchant.key().as_ref()], bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(
        mut,
        seeds=[b"store", merchant.key().as_ref()], bump
    )]
    pub store: Account<'info, Store>,
    /// CHECK: 手動 bump の別PDA
    pub escrow_drawer: AccountInfo<'info>,
    pub merchant: Signer<'info>,
}

#[account]
pub struct Store {
    pub owner: Pubkey,
    pub fee_bps: u16,
    pub sales: u32,
    pub reviews: u32,
}

#[error_code]
pub enum StoreErr {
    #[msg("seed invalid")]
    SeedBad,
    #[msg("escrow drawer key mismatch")]
    DrawerMismatch,
}
