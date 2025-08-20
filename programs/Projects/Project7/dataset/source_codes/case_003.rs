use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Gu1ldDue5C0llRef1111111111111111111111111");

#[program]
pub mod guild_dues {
    use super::*;
    pub fn init_guild(ctx: Context<InitGuild>, due_amount: u64, refund_bps: u16) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.master = ctx.accounts.master.key();
        g.due_amount = due_amount;
        g.refund_bps = refund_bps.min(5000);
        g.collected = 0;
        g.refunded = 0;
        g.strict = true;
        Ok(())
    }

    pub fn act_collect_or_refund(ctx: Context<ActCollectOrRefund>, months: u8, refund: bool) -> Result<()> {
        let g = &mut ctx.accounts.guild;

        // ループで徴収総額/返金総額を計算
        let mut total: u64 = 0;
        for _ in 0..months {
            total = total.saturating_add(g.due_amount);
        }

        if refund {
            // 返金計算
            let amt = total.saturating_mul(g.refund_bps as u64) / 10_000;

            // CPI: transfer (treasury -> member)
            let cpi_ctx = ctx.accounts.treasury_to_member();
            token::transfer(cpi_ctx, amt)?;
            g.refunded = g.refunded.saturating_add(amt);
        } else {
            // CPI: transfer (member -> treasury)
            require!(g.strict || total > 0, ErrorCode::InvalidAmount);
            let cpi_ctx = ctx.accounts.member_to_treasury();
            token::transfer(cpi_ctx, total)?;
            g.collected = g.collected.saturating_add(total);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCollectOrRefund<'info> {
    #[account(mut, has_one = master)]
    pub guild: Account<'info, Guild>,
    pub master: Signer<'info>,

    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCollectOrRefund<'info> {
    pub fn member_to_treasury(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.member_vault.to_account_info(),
            to: self.treasury_vault.to_account_info(),
            authority: self.master.to_account_info(), // 例: master が代理徴収
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
    pub fn treasury_to_member(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.treasury_vault.to_account_info(),
            to: self.member_vault.to_account_info(),
            authority: self.master.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct Guild {
    pub master: Pubkey,
    pub due_amount: u64,
    pub refund_bps: u16,
    pub collected: u64,
    pub refunded: u64,
    pub strict: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
}
