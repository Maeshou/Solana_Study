use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("GuildDueV200000000000000000000000000000000");

#[program]
pub mod guild_dues_cycle_v2 {
    use super::*;

    pub fn init_guild(
        ctx: Context<InitGuild>,
        monthly_due: u64,
        refund_ratio_bps: u16,
        strict_mode: bool,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.leader = ctx.accounts.leader.key();
        guild.due_units = monthly_due;
        guild.refund_bps = refund_ratio_bps.min(6000);
        guild.period_count = 1;
        guild.collected_units = monthly_due; // 初期徴収分として設定
        guild.strict = strict_mode;
        Ok(())
    }

    pub fn act_collect_or_refund(
        ctx: Context<ActCollectOrRefund>,
        months: u8,
        do_refund: bool,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;

        // ループ：月数分の金額合計
        let mut aggregate = guild.due_units;
        let mut idx = 1u8;
        while idx < months {
            aggregate = aggregate.saturating_add(guild.due_units);
            idx = idx.saturating_add(1);
        }

        if do_refund {
            let refund_amount = aggregate.saturating_mul(guild.refund_bps as u64) / 10_000;
            let cpi = ctx.accounts.pay_from_treasury_to_member();
            token::transfer(cpi, refund_amount)?;
            guild.collected_units = guild.collected_units.saturating_sub(refund_amount);
        } else {
            if guild.strict {
                // strict時のみ集金
                let cpi = ctx.accounts.pay_from_member_to_treasury();
                token::transfer(cpi, aggregate)?;
            } else {
                // 非strict: 半額のみ集金
                let cpi = ctx.accounts.pay_from_member_to_treasury();
                token::transfer(cpi, aggregate / 2)?;
            }
            guild.collected_units = guild.collected_units.saturating_add(aggregate);
        }

        guild.period_count = guild.period_count.saturating_add(1);
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
pub struct ActCollectOrRefund<'info> {
    #[account(mut, has_one = leader)]
    pub guild: Account<'info, GuildState>,
    pub leader: Signer<'info>,

    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActCollectOrRefund<'info> {
    pub fn pay_from_member_to_treasury(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.member_vault.to_account_info(),
            to: self.treasury_vault.to_account_info(),
            authority: self.leader.to_account_info(), // 代表者徵収
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }

    pub fn pay_from_treasury_to_member(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.treasury_vault.to_account_info(),
            to: self.member_vault.to_account_info(),
            authority: self.leader.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct GuildState {
    pub leader: Pubkey,
    pub due_units: u64,
    pub refund_bps: u16,
    pub period_count: u64,
    pub collected_units: u64,
    pub strict: bool,
}
