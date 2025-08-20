use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NftCraftBench111111111111111111111111111");

#[program]
pub mod nft_crafting_bench {
    use super::*;
    pub fn craft(ctx: Context<Craft>, material_a: u64, material_b: u64) -> Result<()> {
        let st = &mut ctx.accounts.bench;
        st.attempts += 1;

        let mut program = ctx.accounts.recipe_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.route_custom += 1;
        } else {
            st.loss_rate = (st.loss_rate + (material_a & 3) + (material_b & 1)).min(100);
            st.recipe_fail += 1;
            st.backup_recipes.push(((material_a ^ material_b) & 0xffff) as u16);
        }

        let br = CraftBridge { mat_a: ctx.accounts.material_a.to_account_info(), mat_b: ctx.accounts.material_b.to_account_info() };
        let quality = ((material_a | material_b) & 31) + st.attempts as u64;
        let mut data = Vec::with_capacity(24);
        data.extend_from_slice(&st.attempts.to_le_bytes());
        data.extend_from_slice(&quality.to_le_bytes());
        data.extend_from_slice(&st.loss_rate.to_le_bytes());

        let cx = br.as_cpi(program.clone());
        br.invoke_recipe(cx, data)?;
        st.last_quality = quality;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(init, payer = smith, space = 8 + 8 + 8 + 8 + 1 + 4 + (4 + 64*2))]
    pub bench: Account<'info, BenchState>,
    #[account(mut)] pub smith: Signer<'info>,
    /// CHECK:
    pub material_a: AccountInfo<'info>,
    /// CHECK:
    pub material_b: AccountInfo<'info>,
    /// CHECK:
    pub recipe_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BenchState {
    pub attempts: u64,
    pub route_custom: u64,
    pub recipe_fail: u64,
    pub last_quality: u64,
    pub loss_rate: u64,
    pub pad: u32,
    pub backup_recipes: Vec<u16>,
}

#[derive(Clone)]
pub struct CraftBridge<'info> { pub mat_a: AccountInfo<'info>, pub mat_b: AccountInfo<'info> }
impl<'info> CraftBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, CraftBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.mat_a.key, false), AccountMeta::new_readonly(*self.mat_b.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.mat_a.clone(), self.mat_b.clone()] }
    pub fn invoke_recipe(&self, cx: CpiContext<'_, '_, '_, 'info, CraftBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
