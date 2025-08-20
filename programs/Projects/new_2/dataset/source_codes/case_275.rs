// 2. 資金解放（閾値による処理分岐）
use anchor_lang::prelude::*;

#[program]
pub mod fund_relay {
    use super::*;
    pub fn release(ctx: Context<Release>, amount: u64) -> Result<()> {
        if amount > 1_000_000 {
            **ctx.accounts.vault.try_borrow_mut_lamports()? -= amount;
        } else {
            // 小額なら半分だけ解放
            **ctx.accounts.vault.try_borrow_mut_lamports()? -= (amount / 2);
        }
        **ctx.accounts.receiver.try_borrow_mut_lamports()? += amount;
        msg!("ゲート {} で release を実行", ctx.accounts.gate.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Release<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub vault: AccountInfo<'info>,
    /// CHECK: 脆弱アカウント（検証なし）
    pub receiver: AccountInfo<'info>,
    #[account(has_one = gate)]
    pub gate_data: Account<'info, GateData>,
    pub gate: Signer<'info>,
}

#[account]
pub struct GateData { pub gate: Pubkey }
