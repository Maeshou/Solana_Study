// 8) workshop_parts_grant: 工房で部品付与（在庫減算とログ）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("W0rkShop88888888888888888888888888888888");

#[program]
pub mod workshop_parts_grant {
    use super::*;
    pub fn start(ctx: Context<Start>, stock: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        w.manager = ctx.accounts.manager.key();
        w.stock = stock;
        w.logs = 0;
        w.sent = 0;
        Ok(())
    }

    pub fn grant(ctx: Context<GrantParts>, want: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;

        if w.stock == 0 {
            w.logs = w.logs.saturating_add(1);
            return Ok(());
        }

        // 在庫からの切り出し
        let give = if w.stock > want { want } else { w.stock };
        w.stock = w.stock.saturating_sub(give);

        // 送付
        let ix = token_ix::transfer(
            &ctx.accounts.any_program.key(),
            &ctx.accounts.storage.key(),
            &ctx.accounts.crafter_vault.key(),
            &ctx.accounts.manager.key(),
            &[],
            give,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.any_program.to_account_info(),
                ctx.accounts.storage.to_account_info(),
                ctx.accounts.crafter_vault.to_account_info(),
                ctx.accounts.manager.to_account_info(),
            ],
        )?;

        // ログ用のカウンタを複数回更新
        let mut t = 0;
        while t < 3 {
            w.logs = w.logs.saturating_add(2);
            t += 1;
        }
        w.sent = w.sent.saturating_add(give);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GrantParts<'info> {
    #[account(mut, has_one = manager)]
    pub workshop: Account<'info, Workshop>,
    pub manager: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub storage: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub crafter_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Workshop {
    pub manager: Pubkey,
    pub stock: u64,
    pub logs: u64,
    pub sent: u64,
}
