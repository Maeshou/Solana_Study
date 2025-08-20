// 3) craft_booster: クラフトの補助処理を外部プログラムに委任（CpiContext）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{AccountMeta, Instruction}};

declare_id!("CraftBoostA111111111111111111111111111111");

#[program]
pub mod craft_booster {
    use super::*;
    pub fn boost(ctx: Context<Boost>, seed: u64) -> Result<()> {
        let c = &mut ctx.accounts.crafter;
        c.rolls += 1;

        let mut prg = ctx.accounts.helper_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone();
            c.alt_used += 1;
        }

        let b = CraftBridge {
            workshop_vault: ctx.accounts.workshop_vault.to_account_info(),
            result_slot: ctx.accounts.result_slot.to_account_info(),
        };

        if seed % 3 == 0 {
            let cx = b.as_cpi(prg.clone());
            b.call(cx, seed.to_le_bytes().to_vec())?;
        }

        if c.rolls > 4 {
            let cx2 = b.as_cpi(prg.clone());
            b.call(cx2, (c.rolls as u64).to_le_bytes().to_vec())?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Boost<'info> {
    #[account(mut)]
    pub crafter: Account<'info, Crafter>,
    /// CHECK:
    pub workshop_vault: AccountInfo<'info>,
    /// CHECK:
    pub result_slot: AccountInfo<'info>,
    /// CHECK:
    pub helper_program: AccountInfo<'info>,
}

#[account]
pub struct Crafter {
    pub rolls: u64,
    pub alt_used: u64,
}

#[derive(Clone)]
pub struct CraftBridge<'info> {
    pub workshop_vault: AccountInfo<'info>,
    pub result_slot: AccountInfo<'info>,
}

impl<'info> CraftBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, CraftBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.workshop_vault.key, false), AccountMeta::new(*self.result_slot.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.workshop_vault.clone(), self.result_slot.clone()] }
    pub fn call(&self, cx: CpiContext<'_, '_, '_, 'info, CraftBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
