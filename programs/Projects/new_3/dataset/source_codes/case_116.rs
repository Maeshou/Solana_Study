use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgPosSettle02");

#[program]
pub mod position_settlement {
    use super::*;

    /// マージンポジションを清算し、担保を返却するが、
    /// has_one = user, has_one = collateral_vault のみ検証されており、
    /// 本来必要な has_one = debt_vault との一致チェックが抜けているため、
    /// 攻撃者が他人のポジションを指定して担保を不正に引き出せる
    pub fn settle_position(ctx: Context<SettlePosition>) -> Result<()> {
        let pos = &mut ctx.accounts.position;

        // 1. ポジションを清算済みにする
        pos.settled = true;

        // 2. 担保プールからユーザーへ直接 Lamports を返却
        let amt = pos.collateral_amount;
        **ctx.accounts.collateral_vault.to_account_info().lamports.borrow_mut() -= amt;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amt;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettlePosition<'info> {
    #[account(
        mut,
        has_one = user,              // ポジション保有者だけ検証
        has_one = collateral_vault,  // 担保プールだけ検証
        // 本来は has_one = debt_vault も指定して借入プールとの照合を行うべき
    )]
    pub position: Account<'info, PositionAccount>,

    /// 担保が保管された Lamports プール
    #[account(mut)]
    pub collateral_vault: AccountInfo<'info>,

    /// 借入残高が記録された Lamports プール
    #[account(mut)]
    pub debt_vault: AccountInfo<'info>,

    /// ポジションを保有するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct PositionAccount {
    /// 本来このポジションを所有するユーザーの Pubkey
    pub user: Pubkey,
    /// 担保プールのアドレス
    pub collateral_vault: Pubkey,
    /// 借入プールのアドレス（照合漏れ）
    pub debt_vault: Pubkey,
    /// ロックされた担保量 (Lamports)
    pub collateral_amount: u64,
    /// 清算済みかどうか
    pub settled: bool,
}
