use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("NftStakeGuild111111111111111111111111111");

#[program]
pub mod nft_stake_guild {
    use super::*;
    pub fn harvest(ctx: Context<Harvest>, stake_units: u64) -> Result<()> {
        let st = &mut ctx.accounts.pool;
        st.harvests += 1;

        let mut program = ctx.accounts.redeem_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            program = ctx.remaining_accounts[0].clone();
            st.route_fast += 1;
        } else {
            st.slash_points = st.slash_points.saturating_add(stake_units / 10);
            st.late_fee = st.late_fee.saturating_add((Clock::get()?.slot & 31) as u64);
            st.next_bonus = (st.next_bonus + 1).min(5);
            st.route_slow += 1;
        }

        let br = StakeBridge { vault: ctx.accounts.stake_vault.to_account_info(), mint: ctx.accounts.stake_mint.to_account_info() };
        let window = (Clock::get()?.slot & 3) as u64 + 2;
        let base = stake_units / window + 1;
        let mut i = 0u64;
        while i < window {
            let amt = base + i * (1 + st.next_bonus as u64);
            let mut data = Vec::with_capacity(16);
            data.extend_from_slice(&st.harvests.to_le_bytes());
            data.extend_from_slice(&amt.to_le_bytes());
            let cx = br.as_cpi(program.clone());
            br.redeem(cx, data)?;
            st.total_redeemed += amt;
            i += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(init, payer = staker, space = 8 + 8 + 8 + 8 + 8 + 8 + 8)]
    pub pool: Account<'info, StakeState>,
    #[account(mut)] pub staker: Signer<'info>,
    /// CHECK:
    pub stake_vault: AccountInfo<'info>,
    /// CHECK:
    pub stake_mint: AccountInfo<'info>,
    /// CHECK:
    pub redeem_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeState {
    pub harvests: u64,
    pub route_fast: u64,
    pub route_slow: u64,
    pub total_redeemed: u64,
    pub slash_points: u64,
    pub late_fee: u64,
    pub next_bonus: u64,
}

#[derive(Clone)]
pub struct StakeBridge<'info> { pub vault: AccountInfo<'info>, pub mint: AccountInfo<'info> }
impl<'info> StakeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, StakeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new_readonly(*self.vault.key, false), AccountMeta::new(*self.mint.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.vault.clone(), self.mint.clone()] }
    pub fn redeem(&self, cx: CpiContext<'_, '_, '_, 'info, StakeBridge<'info>>, bytes: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: bytes };
        invoke(&ix, &self.infos(&cx.program))?; Ok(())
    }
}
