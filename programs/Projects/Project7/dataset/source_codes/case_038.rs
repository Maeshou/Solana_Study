use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("GldC03Hk5Rz7Pw1Lm9Tq2Ns8Vy4Fd6Jb3Xe0C003");

#[program]
pub mod guild_dues_ladder_v1 {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, monthly_due: u64, base_refund_bps: u16) -> Result<()> {
        let ledger_state = &mut ctx.accounts.ledger_state;
        ledger_state.leader = ctx.accounts.leader.key();
        ledger_state.monthly_due = if monthly_due < 3 { 3 } else { monthly_due };
        ledger_state.base_refund_bps = if base_refund_bps > 7000 { 7000 } else { base_refund_bps };
        ledger_state.period_counter = 2;
        ledger_state.net_collected = monthly_due + 5;
        ledger_state.rollover_units = 1;
        Ok(())
    }

    pub fn act_cycle(ctx: Context<ActCycle>, months: u8, request_refund: bool) -> Result<()> {
        let ledger_state = &mut ctx.accounts.ledger_state;

        // ラダー割引：3,6,9ヶ月地点で都度5%引き
        let mut aggregate_due: u64 = ledger_state.monthly_due;
        let mut month_cursor: u8 = 1;
        while month_cursor < months {
            let mut period_due: u64 = ledger_state.monthly_due;
            if month_cursor >= 3 { period_due = period_due - (period_due / 20); }
            if month_cursor >= 6 { period_due = period_due - (period_due / 20); }
            if month_cursor >= 9 { period_due = period_due - (period_due / 20); }
            aggregate_due = aggregate_due + period_due;
            month_cursor = month_cursor + 1;
        }

        if request_refund {
            // 返金の一部は次期へロール
            let mut effective_bps: u64 = ledger_state.base_refund_bps as u64;
            if ledger_state.period_counter % 2 == 0 {
                effective_bps = effective_bps + 150;
            }
            let refund_total: u64 = (aggregate_due as u128 * effective_bps as u128 / 10_000u128) as u64;
            let rollover_part: u64 = refund_total / 10;
            let immediate_refund: u64 = refund_total - rollover_part;

            token::transfer(ctx.accounts.treasury_to_member_ctx(), immediate_refund)?;
            ledger_state.rollover_units = ledger_state.rollover_units + rollover_part;
            ledger_state.net_collected = ledger_state.net_collected - immediate_refund;
        } else {
            // 既存ロール分を相殺してから徴収
            let mut charge_amount: u64 = aggregate_due;
            if ledger_state.rollover_units > 0 {
                let apply_units = core::cmp::min(ledger_state.rollover_units, charge_amount);
                charge_amount = charge_amount - apply_units;
                ledger_state.rollover_units = ledger_state.rollover_units - apply_units;
            }
            token::transfer(ctx.accounts.member_to_treasury_ctx(), charge_amount)?;
            ledger_state.net_collected = ledger_state.net_collected + charge_amount;
        }

        ledger_state.period_counter = ledger_state.period_counter + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 2 + 8 + 8 + 8)]
    pub ledger_state: Account<'info, GuildLedger>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActCycle<'info> {
    #[account(mut, has_one = leader)]
    pub ledger_state: Account<'info, GuildLedger>,
    pub leader: Signer<'info>,

    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub guild_treasury: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActCycle<'info> {
    pub fn member_to_treasury_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.member_vault.to_account_info(),
            to: self.guild_treasury.to_account_info(),
            authority: self.leader.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn treasury_to_member_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.guild_treasury.to_account_info(),
            to: self.member_vault.to_account_info(),
            authority: self.leader.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}
#[account]
pub struct GuildLedger {
    pub leader: Pubkey,
    pub monthly_due: u64,
    pub base_refund_bps: u16,
    pub period_counter: u64,
    pub net_collected: u64,
    pub rollover_units: u64,
}
