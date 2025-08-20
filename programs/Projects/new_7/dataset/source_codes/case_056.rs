// 2) quest_reward_drop: クエスト進捗に応じてトークン配布
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("Que5tDroP22222222222222222222222222222222");

#[program]
pub mod quest_reward_drop {
    use super::*;
    pub fn setup(ctx: Context<Setup>, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.base = base;
        s.sent = 0;
        s.episode = 0;
        Ok(())
    }

    pub fn drop(ctx: Context<Drop>, progress: u16, repeat: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;

        // 反復で倍率を変える
        let mut mult = s.base;
        let mut t = 0;
        while t < repeat {
            mult = mult.saturating_add(1);
            t += 1;
        }

        // しきい値：進捗が小さい場合は別の統計更新のみ
        let amt = (progress as u64).saturating_mul(mult);
        if progress == 0 {
            s.episode = s.episode.saturating_add(1);
            s.sent = s.sent.saturating_add(0);
            return Ok(());
        }

        // 実送付
        let ix = token_ix::transfer(
            &ctx.accounts.any_program.key(),
            &ctx.accounts.reward_pool.key(),
            &ctx.accounts.player_vault.key(),
            &ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.any_program.to_account_info(),
                ctx.accounts.reward_pool.to_account_info(),
                ctx.accounts.player_vault.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;
        s.sent = s.sent.saturating_add(amt);

        // 最後に小さな積み上げループ
        let mut z = 0;
        while z < 2 {
            s.episode = s.episode.saturating_add(1);
            z += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Drop<'info> {
    #[account(mut, has_one = owner)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub reward_pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub base: u64,
    pub sent: u64,
    pub episode: u64,
}
