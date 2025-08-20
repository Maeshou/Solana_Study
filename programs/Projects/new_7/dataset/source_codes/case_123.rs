use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{Instruction, AccountMeta},
    program::invoke,
};

declare_id!("ArbCpiDemo111111111111111111111111111111");

#[program]
pub mod arb_cpi_demo {
    use super::*;

    pub fn run(ctx: Context<Run>, value: u64) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.calls += 1;

        // program の選択（if let を使わず、長さで判定）
        let mut program = ctx.accounts.alt_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.branch_a += value;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.branch_b += value;
        }

        // Bridge 経由で CPI 実行
        let bridge = DemoBridge {
            user: ctx.accounts.user_account.to_account_info(),
            vault: ctx.accounts.vault_account.to_account_info(),
        };
        let cx = bridge.as_cpi(program.clone());
        bridge.invoke_call(cx, value)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 8 + 8)]
    pub state: Account<'info, DemoState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK:
    pub user_account: AccountInfo<'info>,
    /// CHECK:
    pub vault_account: AccountInfo<'info>,
    /// CHECK:
    pub alt_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DemoState {
    pub owner: Pubkey,
    pub calls: u64,
    pub branch_a: u64,
    pub branch_b: u64,
}

#[derive(Clone)]
pub struct DemoBridge<'info> {
    pub user: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
}

impl<'info> DemoBridge<'info> {
    pub fn as_cpi(
        &self,
        program: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, DemoBridge<'info>> {
        CpiContext::new(program, self.clone())
    }

    fn metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(*self.user.key, false),
            AccountMeta::new(*self.vault.key, false),
        ]
    }

    fn infos(&self, program: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> {
        vec![program.clone(), self.user.clone(), self.vault.clone()]
    }

    pub fn invoke_call(
        &self,
        ctx: CpiContext<'_, '_, '_, 'info, DemoBridge<'info>>,
        val: u64,
    ) -> Result<()> {
        let ix = Instruction {
            program_id: *ctx.program.key, // ← Arbitrary CPI 経路
            accounts: self.metas(),
            data: val.to_le_bytes().to_vec(),
        };
        invoke(&ix, &self.infos(&ctx.program))?;
        Ok(())
    }
}
