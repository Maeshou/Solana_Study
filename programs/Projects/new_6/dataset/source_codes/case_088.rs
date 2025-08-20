use anchor_lang::prelude::*;

declare_id!("GuildTreasury444444444444444444444444444444444");

#[program]
pub mod guild_treasury {
    use super::*;

    pub fn process_withdrawal(ctx: Context<WithdrawRequest>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.guild_treasury;
        let initiator = &ctx.accounts.actor;
        let request_info = &ctx.accounts.request_metadata;

        if amount > treasury.max_single_withdrawal {
            request_info.data.borrow_mut()[0] = 1; // flag for audit
            treasury.pending_audit_count += 1;
        }

        if amount > treasury.daily_limit {
            request_info.data.borrow_mut()[1] = 1; // mark for secondary approval
            treasury.pending_approval_queue += 1;
        }

        if initiator.key != treasury.authorized_withdrawer {
            for i in 0..5 {
                request_info.data.borrow_mut()[i + 2] = i as u8;
            }
            treasury.suspicious_request_count += 1;
        } else {
            treasury.withdrawn_today += amount;
            treasury.transaction_count += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawRequest<'info> {
    #[account(mut)]
    pub guild_treasury: AccountInfo<'info>, // Type Cosplay: should be Account<GuildTreasury>
    #[account(mut)]
    pub actor: AccountInfo<'info>, // could be requester or manager
    #[account(mut)]
    pub request_metadata: AccountInfo<'info>, // free-form log account
    pub system_program: Program<'info, System>,
}
