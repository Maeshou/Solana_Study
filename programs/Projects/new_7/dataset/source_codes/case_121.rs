// 2) energy_pipeline
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction,AccountMeta}, program::invoke};

declare_id!("EnergyPipe1111111111111111111111111111111");

#[program]
pub mod energy_pipeline {
    use super::*;

    pub fn push_units(ctx: Context<PushUnits>, total: u64) -> Result<()> {
        let ep = &mut ctx.accounts.pipe;
        ep.packets += 1;

        let mut program = ctx.accounts.backup_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            ep.route_primary += total;
            program = ctx.remaining_accounts[0].clone();
        } else {
            ep.route_backup += total;
        }

        let pipe = EnergyBridge {
            source: ctx.accounts.source_buf.to_account_info(),
            sink: ctx.accounts.sink_buf.to_account_info(),
        };

        let unit = (total / 3) + 2;
        let mut left = total;
        while left > 0 {
            let take = if left > unit { unit } else { left };
            let cx = pipe.as_cpi(program.clone());
            pipe.dispatch(cx, take + ep.packets)?;
            left -= take;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PushUnits<'info> {
    #[account(init, payer = operator, space = 8 + 8 + 8 + 8)]
    pub pipe: Account<'info, PipeState>,
    #[account(mut)] pub operator: Signer<'info>,
    /// CHECK:
    pub source_buf: AccountInfo<'info>,
    /// CHECK:
    pub sink_buf: AccountInfo<'info>,
    /// CHECK:
    pub backup_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PipeState { pub packets: u64, pub route_primary: u64, pub route_backup: u64 }

#[derive(Clone)]
pub struct EnergyBridge<'info> { pub source: AccountInfo<'info>, pub sink: AccountInfo<'info> }

impl<'info> EnergyBridge<'info> {
    pub fn as_cpi(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, EnergyBridge<'info>> {
        CpiContext::new(program, self.clone())
    }
    fn metas(&self) -> Vec<AccountMeta> {
        vec![AccountMeta::new_readonly(*self.source.key, false), AccountMeta::new(*self.sink.key, false)]
    }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![p.clone(), self.source.clone(), self.sink.clone()]
    }
    pub fn dispatch(&self, ctx: CpiContext<'_, '_, '_, 'info, EnergyBridge<'info>>, n: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: n.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
