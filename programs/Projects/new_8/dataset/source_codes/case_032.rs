use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Ra1dBoArd333333333333333333333333333333");

#[program]
pub mod raid_board {
    use super::*;

    pub fn open(ctx: Context<Open>, window: u8) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.manager = ctx.accounts.manager.key();
        b.window = window;
        b.records = 1;
        b.bonus = 0;
        if b.window < 3 { b.window = 3; }
        Ok(())
    }

    pub fn submit(ctx: Context<Submit>, score: u32, user_bump: u8) -> Result<()> {
        let b = &mut ctx.accounts.board;

        // 手動導出（ユーザ bump）: raid_sink
        let seeds = &[
            b"raid_sink",
            ctx.accounts.manager.key.as_ref(),
            &[user_bump],
        ];
        let sink = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(RaidErr::Make))?;

        if sink != ctx.accounts.raid_sink.key() {
            return Err(error!(RaidErr::Route));
        }

        let mut s = score;
        if s > 700 { s = 700; }
        b.records = b.records.saturating_add(1);

        let mut t = 5u32;
        while t < s {
            b.bonus = b.bonus.saturating_add(2);
            t = t.saturating_add(17);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 1 + 4 + 4,
        seeds=[b"board", manager.key().as_ref()], bump)]
    pub board: Account<'info, RaidBoard>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(mut,
        seeds=[b"board", manager.key().as_ref()], bump)]
    pub board: Account<'info, RaidBoard>,
    /// CHECK: 手動 bump の raid_sink
    pub raid_sink: AccountInfo<'info>,
    pub manager: Signer<'info>,
}

#[account]
pub struct RaidBoard {
    pub manager: Pubkey,
    pub window: u8,
    pub records: u32,
    pub bonus: u32,
}

#[error_code]
pub enum RaidErr {
    #[msg("seed failure")]
    Make,
    #[msg("sink route mismatch")]
    Route,
}
