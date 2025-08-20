use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Music2ShareRoy4Jt9Qe6Lm8Zx2Cv1Na5Ur7Hp3Kd902");

#[program]
pub mod music_streaming_royalty_v1 {
    use super::*;

    pub fn init_split(ctx: Context<InitSplit>, artist_bps: u16, label_bps: u16, min_payout_input: u64) -> Result<()> {
        let s = &mut ctx.accounts.split;
        s.operator = ctx.accounts.operator.key();
        s.artist_bps = clamp_u16(artist_bps, 1000, 8000);
        s.label_bps = clamp_u16(label_bps, 500, 5000);
        s.min_payout = min_payout_input;
        if s.min_payout < 1 { s.min_payout = 1; }
        s.epoch = 1;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, plays: u64, unit_revenue: u64) -> Result<()> {
        let s = &mut ctx.accounts.split;

        // 再生数に応じて微増
        let mut factor = 100u64;
        let mut count = plays;
        while count >= 10_000 {
            factor = factor + 3;
            count = count - 10_000;
        }

        let gross = (plays as u128 * unit_revenue as u128 / 1000u128) as u64;
        let scaled = (gross as u128 * factor as u128 / 100u128) as u64;

        let artist_cut = (scaled as u128 * s.artist_bps as u128 / 10_000u128) as u64;
        let label_cut = (scaled as u128 * s.label_bps as u128 / 10_000u128) as u64;
        let producer_cut = if scaled > artist_cut + label_cut { scaled - artist_cut - label_cut } else { s.min_payout };

        token::transfer(ctx.accounts.pool_to_artist(), artist_cut)?;
        token::transfer(ctx.accounts.pool_to_label(), label_cut)?;
        token::transfer(ctx.accounts.pool_to_producer(), producer_cut)?;

        s.epoch = s.epoch + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSplit<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 2 + 2 + 8 + 8)]
    pub split: Account<'info, SplitState>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = operator)]
    pub split: Account<'info, SplitState>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub revenue_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub label_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub producer_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSettle<'info> {
    pub fn pool_to_artist(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.revenue_pool_vault.to_account_info(),to:self.artist_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn pool_to_label(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.revenue_pool_vault.to_account_info(),to:self.label_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn pool_to_producer(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.revenue_pool_vault.to_account_info(),to:self.producer_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct SplitState {
    pub operator: Pubkey,
    pub artist_bps: u16,
    pub label_bps: u16,
    pub min_payout: u64,
    pub epoch: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v;if o<lo{o=lo;} if o>hi{o=hi;} o}
