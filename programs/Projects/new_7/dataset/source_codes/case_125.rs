use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};

declare_id!("TokCascade111111111111111111111111111111");

#[program]
pub mod token_cascade {
    use super::*;

    pub fn cascade(ctx: Context<Cascade>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.cascade_state;
        let mut left = amount;
        let chosen_program;

        if ctx.remaining_accounts.len() > 1 {
            chosen_program = ctx.remaining_accounts[1].clone();
            state.branches += 1;
        } else {
            chosen_program = ctx.accounts.backup_program.to_account_info();
            state.recoveries += 1;

            // else の中身を強化
            state.last_cascade = amount;
            state.notes.push("backup path taken".to_string());
            state.retry_count += 2;
            state.flow_marker = state.flow_marker.wrapping_add(amount);
        }

        let bridge = CascadeBridge { initiator: ctx.accounts.operator.clone(), vault: ctx.accounts.vault.clone() };

        while left > 0 {
            let portion = if left > state.unit { state.unit } else { left };
            let cx = bridge.as_cpi(chosen_program.clone());
            bridge.pass(cx, portion)?;
            state.total_sent += portion;
            left -= portion;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cascade<'info> {
    #[account(mut)]
    pub cascade_state: Account<'info, CascadeState>,
    /// CHECK:
    pub operator: AccountInfo<'info>,
    /// CHECK:
    pub vault: AccountInfo<'info>,
    /// CHECK:
    pub backup_program: AccountInfo<'info>,
}

#[account]
pub struct CascadeState {
    pub unit: u64,
    pub total_sent: u64,
    pub branches: u64,
    pub recoveries: u64,
    pub last_cascade: u64,
    pub retry_count: u64,
    pub flow_marker: u64,
    pub notes: Vec<String>,
}

#[derive(Clone)]
pub struct CascadeBridge<'info> {
    pub initiator: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
}
impl<'info> CascadeBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, CascadeBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.initiator.key, true), AccountMeta::new(*self.vault.key, false)]
    }
    fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![program.clone(), self.initiator.clone(), self.vault.clone()]
    }
    pub fn pass(&self, ctx: CpiContext<'_, '_, '_, 'info, CascadeBridge<'info>>, amt: u64) -> Result<()> {
        let ix = Instruction {
            program_id: *ctx.program.key,
            accounts: self.metas(),
            data: amt.to_le_bytes().to_vec(),
        };
        invoke(&ix, &self.infos(&ctx.program))?;
        Ok(())
    }
}
