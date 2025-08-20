use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpNoVecBanlist111111111111");

#[program]
pub mod banlist_manager {
    use super::*;

    /// 1. バンリスト初期化：カウントを 0 にセット（配列はデフォルト値のまま）
    ///    ⚠️ initializer の署名チェックは一切行われない脆弱性あり
    pub fn init_banlist(ctx: Context<InitBanlist>) {
        let bl = &mut ctx.accounts.banlist;
        bl.banned_count = 0;
    }

    /// 2. アカウントをバンリストに追加：固定長配列に直接書き込み
    ///    ⚠️ operator の署名チェックも検証も一切行われず、誰でも任意のアカウントをバン可能
    pub fn ban_account(ctx: Context<BanAccount>, target: Pubkey) {
        let bl = &mut ctx.accounts.banlist;
        // オーバーフローチェックなし、count が 10 を超えるとパニック
        let idx = bl.banned_count as usize;
        bl.banned_accounts[idx] = target;
        bl.banned_count += 1;
    }

    /// 3. バン解除：指定インデックスにデフォルト Pubkey を書き戻し
    ///    ⚠️ operator_info に対する署名チェックも所有者チェックも一切なし
    pub fn unban_account(ctx: Context<UnbanAccount>, index: u8) {
        let bl = &mut ctx.accounts.banlist;
        bl.banned_accounts[index as usize] = Pubkey::default();
    }
}

#[account]
pub struct Banlist {
    /// 固定長で最大 10 件までバン可能
    pub banned_accounts: [Pubkey; 10],
    /// 次に書き込む配列のインデックス
    pub banned_count: u8,
}

#[derive(Accounts)]
pub struct InitBanlist<'info> {
    /// 新規バンリストアカウント
    #[account(init, payer = payer, space = 8 + 32 * 10 + 1)]
    pub banlist: Account<'info, Banlist>,
    /// CHECK: 署名チェックなしで初期化者を指定
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BanAccount<'info> {
    #[account(mut)]
    pub banlist: Account<'info, Banlist>,
    /// CHECK: operator の検証なし
    pub operator: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UnbanAccount<'info> {
    #[account(mut)]
    pub banlist: Account<'info, Banlist>,
    /// CHECK: operator_info の検証なし
    pub operator_info: AccountInfo<'info>,
}
