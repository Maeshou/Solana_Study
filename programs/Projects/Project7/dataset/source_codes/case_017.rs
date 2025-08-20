use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("GuildV3yN2w5q4yN2w5q4yN2w5q4yN2w5q4yN2w5q4");

#[program]
pub mod guild_dues_v3 {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, monthly: u64, refund_bps: u16, soft: bool) -> Result<()> {
        let g = &mut ctx.accounts.guild;
        g.leader = ctx.accounts.leader.key();
        g.monthly_due = monthly.max(3);
        g.refund_rate_bps = refund_bps.min(7000).max(500);
        g.period_index = 2;
        g.net_collected = monthly.saturating_mul(2); // 既存期間がある想定
        g.soft_mode = soft;
        Ok(())
    }

    pub fn act_cycle(ctx: Context<ActCycle>, months: u8, request_refund: bool) -> Result<()> {
        let g = &mut ctx.accounts.guild;

        // 月数に応じた集金額（3ヶ月目以降は1割割引）
        let mut total = g.monthly_due;
        let mut k = 1u8;
        while k < months {
            let add = if k >= 2 { g.monthly_due.saturating_mul(9) / 10 } else { g.monthly_due };
            total = total.saturating_add(add);
            k = k.saturating_add(1);
        }

        if request_refund {
            // 段階割戻し：申請回数に応じてレート微減
            let mut rate = g.refund_rate_bps as u64;
            if g.period_index % 2 == 0 { rate = rate.saturating_sub(250); }
            let give_back = total.saturating_mul(rate) / 10_000;

            token::transfer(ctx.accounts.treasury_to_member(), give_back)?;
            g.net_collected = g.net_collected.saturating_sub(give_back);
        } else {
            // ソフトモードなら分割徵収
            if g.soft_mode {
                token::transfer(ctx.accounts.member_to_treasury(), total.saturating_div(2))?;
                token::transfer(ctx.accounts.member_to_treasury(), total.saturating_div(2))?;
            } else {
                token::transfer(ctx.accounts.member_to_treasury(), total)?;
            }
            g.net_collected = g.net_collected.saturating_add(total);
        }

        g.period_index = g.period_index.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub guild: Account<'info, GuildState>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActCycle<'info> {
    #[account(mut, has_one = leader)]
    pub guild: Account<'info, GuildState>,
    pub leader: Signer<'info>,
    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub guild_treasury: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ActCycle<'info> {
    pub fn member_to_treasury(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.member_vault.to_account_info(),
            to: self.guild_treasury.to_account_info(),
            authority: self.leader.to_account_info(), // 代表者徵収
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
    pub fn treasury_to_member(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.guild_treasury.to_account_info(),
            to: self.member_vault.to_account_info(),
            authority: self.leader.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub monthly_due: u64,
    pub refund_rate_bps: u16,
    pub period_index: u64,
    pub net_collected: u64,
    pub soft_mode: bool,
}
