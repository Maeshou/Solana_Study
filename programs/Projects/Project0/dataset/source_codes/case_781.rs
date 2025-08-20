use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf781mvTWf");

#[program]
pub mod sequence_flow_781 {
    use super::*;

    pub fn sequence_flow(ctx: Context<SequenceFlow781>, lamports: u64) -> Result<()> {
        let flow_bump = *ctx.bumps.get("flow").unwrap();
        let audit_bump = *ctx.bumps.get("audit").unwrap();
        // CPI: system transfer
        let ix = system_program::Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
        };
        system_program::transfer(
            CpiContext::new(ctx.accounts.sys_prog.to_account_info(), ix),
            lamports,
        )?;
        // Clock and Rent
        let clk = Clock::get()?;
        let rent_bal = ctx.accounts.rent.minimum_balance(0);
        // Update state
        let flow = &mut ctx.accounts.flow;
        flow.bump = flow_bump;
        flow.total = flow.total.checked_add(lamports).unwrap();
        flow.last_slot = clk.slot;
        let audit = &mut ctx.accounts.audit;
        audit.bump = audit_bump;
        audit.collected_rent = rent_bal;
        msg!(
            "Case 781: flow_bump={} total={} slot={} rent={}",
            flow_bump,
            flow.total,
            flow.last_slot,
            audit.collected_rent
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SequenceFlow781<'info> {
    #[account(address = system_program::ID)]
    pub sys_prog: Program<'info, System>,
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: SystemAccount<'info>,
    #[account(init, seeds = [b"flow"], bump, payer = payer, space = 8 + 1 + 8 + 8)]
    pub flow: Account<'info, Flow781>,
    #[account(init, seeds = [b"audit"], bump, payer = payer, space = 8 + 1 + 8)]
    pub audit: Account<'info, Audit781>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Flow781 {
    pub bump: u8,
    pub total: u64,
    pub last_slot: u64,
}

#[account]
pub struct Audit781 {
    pub bump: u8,
    pub collected_rent: u64,
}
