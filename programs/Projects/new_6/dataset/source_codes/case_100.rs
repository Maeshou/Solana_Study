use anchor_lang::prelude::*;

declare_id!("GTreasury999999999999999999999999999999999");

#[program]
pub mod guild_treasury {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>, max_withdrawals: u8) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.guild_id = ctx.accounts.admin.key();
        treasury.max_withdrawals = max_withdrawals;
        treasury.remaining_withdrawals = max_withdrawals;
        treasury.locked = false;
        treasury.audit_log = Vec::new();
        Ok(())
    }

    pub fn propose_withdrawal(ctx: Context<ProposeWithdrawal>, amount: u64, reason: String) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let actor = &ctx.accounts.actor;
        let timestamp = Clock::get()?.unix_timestamp;

        if treasury.locked {
            treasury.audit_log.push(format!("{}: Rejected proposal by {} due to locked treasury", timestamp, actor.key()));
            return Ok(());
        }

        treasury.pending_amount = Some(amount);
        treasury.pending_reason = Some(reason.clone());
        treasury.pending_actor = Some(actor.key());
        treasury.pending_at = Some(timestamp);

        treasury.audit_log.push(format!("{}: Proposal submitted by {} - {} lamports for {}", timestamp, actor.key(), amount, reason));
        Ok(())
    }

    pub fn approve_withdrawal(ctx: Context<ApproveWithdrawal>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let board = &ctx.accounts.board;
        let now = Clock::get()?.unix_timestamp;

        if treasury.locked || treasury.remaining_withdrawals == 0 {
            treasury.audit_log.push(format!("{}: Withdrawal approval failed - treasury locked or exhausted", now));
            return Ok(());
        }

        let approved_amount = treasury.pending_amount.unwrap_or(0);
        if approved_amount == 0 {
            treasury.audit_log.push(format!("{}: Approval failed - no pending amount", now));
            return Ok(());
        }

        // Simulate transfer
        let dest = &ctx.accounts.fund_sink;
        **dest.to_account_info().try_borrow_mut_lamports()? += approved_amount;
        treasury.remaining_withdrawals -= 1;

        treasury.audit_log.push(format!(
            "{}: Withdrawal approved by {} - {} lamports to {}",
            now, board.key(), approved_amount, dest.key()
        ));

        treasury.pending_amount = None;
        treasury.pending_actor = None;
        treasury.pending_reason = None;
        treasury.pending_at = None;
        Ok(())
    }

    pub fn emergency_lock(ctx: Context<LockTreasury>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.locked = true;
        treasury.audit_log.push(format!(
            "{}: Emergency lock initiated by {}",
            Clock::get()?.unix_timestamp,
            ctx.accounts.council.key()
        ));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer = admin, space = 8 + 512)]
    pub treasury: Account<'info, TreasuryData>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeWithdrawal<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TreasuryData>,
    /// CHECK: 脆弱性ポイント - actor には何の検証もなし
    pub actor: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveWithdrawal<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TreasuryData>,
    /// CHECK: 脆弱性ポイント - board メンバーであることの検証なし
    pub board: AccountInfo<'info>,
    /// CHECK: 資金の送金先、任意のアカウントが通る可能性あり
    #[account(mut)]
    pub fund_sink: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LockTreasury<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TreasuryData>,
    /// CHECK: council 役職の証明なし
    pub council: AccountInfo<'info>,
}

#[account]
pub struct TreasuryData {
    pub guild_id: Pubkey,
    pub max_withdrawals: u8,
    pub remaining_withdrawals: u8,
    pub locked: bool,
    pub pending_amount: Option<u64>,
    pub pending_reason: Option<String>,
    pub pending_actor: Option<Pubkey>,
    pub pending_at: Option<i64>,
    pub audit_log: Vec<String>,
}
