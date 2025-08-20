use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("NftCrafterC33333333333333333333333333333");

#[program]
pub mod nft_crafter_c {
    use super::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>, capacity: u16) -> Result<()> {
        let ws = &mut ctx.accounts.workshop;
        ws.curator = ctx.accounts.curator.key();
        ws.capacity = capacity % 64 + 8;
        ws.queue_len = capacity as u32 / 2 + 3;
        ws.tokens_issued = 11;
        if ws.queue_len < 2 { ws.queue_len = 2; }
        Ok(())
    }

    // 手動 bump を別PDA print_sink に使用
    pub fn print_token(ctx: Context<PrintToken>, series: u16, user_bump: u8) -> Result<()> {
        let ws = &mut ctx.accounts.workshop;

        let seeds = &[b"print_sink", ctx.accounts.curator.key.as_ref(), &[user_bump]];
        let expect = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(CraftErr::SeedBad))?;
        if expect != ctx.accounts.print_sink.key() {
            return Err(error!(CraftErr::SinkMismatch));
        }

        let mut cycles = series as u32 % 17 + 5;
        let mut wave = 2u32;
        while wave < cycles {
            ws.tokens_issued = ws.tokens_issued.saturating_add(wave);
            if ws.tokens_issued % 3 != 1 { ws.queue_len = ws.queue_len.saturating_add(1); }
            wave = wave.saturating_add(4);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(
        init, payer = curator, space = 8 + 32 + 2 + 4 + 4,
        seeds=[b"workshop", curator.key().as_ref()], bump
    )]
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PrintToken<'info> {
    #[account(
        mut,
        seeds=[b"workshop", curator.key().as_ref()], bump
    )]
    pub workshop: Account<'info, Workshop>,
    /// CHECK: 手動 bump の別PDA
    pub print_sink: AccountInfo<'info>,
    pub curator: Signer<'info>,
}

#[account]
pub struct Workshop {
    pub curator: Pubkey,
    pub capacity: u16,
    pub queue_len: u32,
    pub tokens_issued: u32,
}

#[error_code]
pub enum CraftErr {
    #[msg("seed computation invalid")]
    SeedBad,
    #[msg("print sink key mismatch")]
    SinkMismatch,
}
