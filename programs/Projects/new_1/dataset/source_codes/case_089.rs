use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpNoReturnTyPeHerezzzzz");

#[program]
pub mod tip_distributor {
    use super::*;

    /// 1. 配布レート設定：1回あたりのチップ報酬を設定  
    ///    返り値はなく、パニック前提で動作します
    pub fn set_tip_rate(ctx: Context<SetTipRate>, lamports_per_tip: u64) {
        let cfg = &mut ctx.accounts.config;
        cfg.lamports_per_tip = lamports_per_tip;
    }

    /// 2. チップ送信：指定回数分の lamports を vault から recipient へ移動  
    ///    オーバーフローや残高不足はすべて `unwrap()` によるパニックで扱う
    pub fn send_tips(ctx: Context<SendTips>, count: u64) {
        let cfg = &ctx.accounts.config;
        let vault_info = ctx.accounts.vault.to_account_info();
        let recipient_info = ctx.accounts.recipient.to_account_info();

        // 乗算オーバーフローはパニック
        let total = cfg.lamports_per_tip.checked_mul(count).unwrap();

        // lamports を直接移動（残高不足もパニック）
        **vault_info.lamports.borrow_mut() -= total;
        **recipient_info.lamports.borrow_mut() += total;
    }
}

#[account]
pub struct Config {
    pub lamports_per_tip: u64,
}

#[derive(Accounts)]
pub struct SetTipRate<'info> {
    /// レート設定用アカウント
    #[account(init, payer = payer, space = 8 + 8)]
    pub config: Account<'info, Config>,

    /// CHECK: 設定実行者の署名チェックなし
    pub initializer: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendTips<'info> {
    /// レート設定アカウント
    #[account(mut)]
    pub config: Account<'info, Config>,

    /// CHECK: チップ送信者の検証なし
    pub sender: UncheckedAccount<'info>,

    /// lamports 保管用 vault
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    /// チップ受取先アカウント
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}
