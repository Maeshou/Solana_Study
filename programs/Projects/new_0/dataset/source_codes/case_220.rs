use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpZ9y8x7w6v5u4t3s2r1q0p9o8n7");

#[program]
pub mod counter {
    use super::*;

    /// カウンターアカウントの初期化
    pub fn initialize_counter(
        ctx: Context<InitializeCounter>,
        bump: u8,
    ) -> ProgramResult {
        let ctr = &mut ctx.accounts.counter;
        ctr.owner = *ctx.accounts.user.key;
        ctr.bump = bump;
        ctr.value = 0;
        Ok(())
    }

    /// カウンターをインクリメント
    pub fn increment(
        ctx: Context<ModifyCounter>,
    ) -> ProgramResult {
        let ctr = &mut ctx.accounts.counter;
        ctr.value = ctr.value.checked_add(1).unwrap();
        Ok(())
    }

    /// カウンターをリセット
    pub fn reset(
        ctx: Context<ModifyCounter>,
        new_value: u64,
    ) -> ProgramResult {
        let ctr = &mut ctx.accounts.counter;
        ctr.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeCounter<'info> {
    #[account(
        init,
        seeds = [b"counter", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 8,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", counter.owner.as_ref()],
        bump = counter.bump,
        has_one = owner,
    )]
    pub counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Counter {
    pub owner: Pubkey,
    pub bump: u8,
    pub value: u64,
}
