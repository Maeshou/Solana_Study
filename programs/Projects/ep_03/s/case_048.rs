use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgScrtchCd01");

#[program]
pub mod scratch_card_service {
    use super::*;

    /// スクラッチカードを削り、当選金を受け取るが、
    /// scratch_card.owner と ctx.accounts.user.key() の一致検証がない
    pub fn scratch(
        ctx: Context<Scratch>,
        card_id: u64,
    ) -> Result<()> {
        let card = &mut ctx.accounts.scratch_card;
        let reward = ctx.accounts.config.win_amount;

        // 1. カード ID を記録
        card.card_id = card_id;

        // 2. 削り済みフラグを立てる
        card.scratched = true;

        // 3. 当選回数をインクリメント
        card.times_scratched = card.times_scratched.checked_add(1).unwrap();

        // 4. 当選金をプールからユーザーへ直接送金
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward;
        **ctx.accounts.pool.to_account_info().lamports.borrow_mut() -= reward;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Scratch<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて
    /// scratch_card.owner と user.key() を照合検証すべき
    pub scratch_card: Account<'info, ScratchCard>,

    /// 削り操作を行うユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 当選金プール（Lamports 保管先）
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    /// 当選金設定を保持するアカウント
    pub config: Account<'info, ScratchConfig>,
}

#[account]
pub struct ScratchCard {
    /// 本来このスクラッチカードを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// この操作で削ったカードの識別子
    pub card_id: u64,
    /// 削り済みフラグ
    pub scratched: bool,
    /// 削り操作回数（通常は 1 回）
    pub times_scratched: u64,
}

#[account]
pub struct ScratchConfig {
    /// スクラッチ成功時の当選金（Lamports）
    pub win_amount: u64,
}
