use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("EnErGyBoX000000000000000000000000000004");

#[program]
pub mod energy_box {
    use super::*;

    pub fn topup(ctx: Context<TopUp>, label: Vec<u8>, amount: u64, bump: u8) -> Result<()> {
        // ラベル正規化と簡易評価
        let mut l = label.clone();
        if l.is_empty() { l.extend_from_slice(b"default"); }
        if l.len() > 32 { l.truncate(32); }
        let mut eval: u64 = 1;
        for b in l.iter() { eval = eval.wrapping_mul(257).wrapping_add(*b as u64); }

        // 任意 bump で PDA 検証（該当点）
        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &l[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(EnergyErr::Addr))?;
        if addr != ctx.accounts.box_cell.key() {
            return Err(error!(EnergyErr::Addr));
        }

        // 加算と上限
        let mut add = amount;
        if add > 50_000 { add = 50_000; }
        let b = &mut ctx.accounts.energy;
        b.owner = ctx.accounts.owner.key();
        b.label = l;
        b.value = b.value.saturating_add(add);
        b.metric = b.metric.wrapping_add(eval);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TopUp<'info> {
    #[account(mut)]
    pub energy: Account<'info, Energy>,
    /// CHECK:
    pub box_cell: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
}

#[account]
pub struct Energy {
    pub owner: Pubkey,
    pub label: Vec<u8>,
    pub value: u64,
    pub metric: u64,
}

#[error_code]
pub enum EnergyErr {
    #[msg("Energy cell PDA mismatch")]
    Addr,
}
