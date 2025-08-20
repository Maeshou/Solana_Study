use anchor_lang::prelude::*;

#[program]
pub mod fund_gate {
    use super::*;
    pub fn release_funds(
        ctx: Context<ReleaseFunds>,
        amount: u64,
        note: String,
    ) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let receiver = &ctx.accounts.receiver;

        // ランプ数の移動のみ
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.to_account_info().try_borrow_mut_lamports()? += amount;

        // 出金ログを積む
        let log = &mut ctx.accounts.release_log;
        log.entries.push(Entry { amount, note });

        msg!("Funds released by {}", ctx.accounts.authority.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub release_log: Account<'info, ReleaseLog>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReleaseLog {
    pub entries: Vec<Entry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Entry {
    pub amount: u64,
    pub note: String,
}
