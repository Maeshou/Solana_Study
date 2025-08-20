use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ForgeStat10n2222222222222222222222222222");

#[program]
pub mod forge_station {
    use super::*;

    pub fn setup(ctx: Context<Setup>, step: u32) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.admin = ctx.accounts.admin.key();
        s.step = step;
        s.energy = 2;
        s.queue = 0;
        if s.step < 2 { s.step = 2; }
        Ok(())
    }

    pub fn craft(ctx: Context<Craft>, power: u16, user_bump: u8) -> Result<()> {
        let s = &mut ctx.accounts.station;

        // 手動導出（ユーザ bump）: tool_bin
        let seeds = &[
            b"tool_bin",
            ctx.accounts.admin.key.as_ref(),
            &[user_bump],
        ];
        let tb = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(ForgeErr::Seed))?;

        if tb != ctx.accounts.tool_bin.key() {
            return Err(error!(ForgeErr::ToolRoute));
        }

        let mut loops = power as u32;
        if loops > 8 { loops = 8; }
        let mut acc = 3u32;
        while acc < 50 {
            s.energy = s.energy.saturating_add(1);
            acc = acc.saturating_add(loops);
        }

        if s.queue < 5 { s.queue = s.queue.saturating_add(2); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"station", admin.key().as_ref()], bump)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut,
        seeds=[b"station", admin.key().as_ref()], bump)]
    pub station: Account<'info, Station>,
    /// CHECK: tool_bin は手動 bump チェック依存
    pub tool_bin: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Station {
    pub admin: Pubkey,
    pub step: u32,
    pub energy: u32,
    pub queue: u32,
}

#[error_code]
pub enum ForgeErr {
    #[msg("seed error")]
    Seed,
    #[msg("tool bin mismatch")]
    ToolRoute,
}
