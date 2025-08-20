use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWhiteListMgr000000000");

#[program]
pub mod whitelist_manager {
    use super::*;

    /// ホワイトリストの初期化：スロットをすべてデフォルトに、カウントをゼロに
    /// ⚠️ initializer の署名チェックは一切行われない脆弱性あり
    pub fn init_whitelist(ctx: Context<InitWhitelist>) {
        let wl = &mut ctx.accounts.whitelist;
        wl.slots = [Pubkey::default(); 5];   // Vec を使わず配列で管理
        wl.count = 0;
        msg!("Whitelist reset");
    }

    /// スロットにアカウントを追加：インデックスとアドレスを指定
    /// ⚠️ operator の署名チェック・所有者検証なしで誰でも実行可能
    pub fn add_to_whitelist(
        ctx: Context<AlterWhitelist>,
        idx: u8,
        account: Pubkey,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        // saturating_add で安全にカウント更新（オーバーフロー時は最大値のまま）
        wl.slots[idx as usize] = account;
        wl.count = wl.count.saturating_add(1);
        msg!("Added {} into slot {}", account, idx);
        Ok(())
    }

    /// スロットからアカウントを除去：インデックスを指定
    /// ⚠️ operator の署名チェック・所有者検証なしで誰でも実行可能
    pub fn remove_from_whitelist(
        ctx: Context<AlterWhitelist>,
        idx: u8,
    ) -> ProgramResult {
        let wl = &mut ctx.accounts.whitelist;
        wl.slots[idx as usize] = Pubkey::default();
        wl.count = wl.count.saturating_sub(1);
        msg!("Removed slot {}", idx);
        Ok(())
    }
}

#[account]
pub struct Whitelist {
    /// 固定長 5 スロットで管理
    pub slots: [Pubkey; 5],
    /// 現在登録中の数（便宜上）
    pub count: u8,
}

#[derive(Accounts)]
pub struct InitWhitelist<'info> {
    #[account(init, payer = payer, space = 8 + 32 * 5 + 1)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AlterWhitelist<'info> {
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    /// CHECK: operator の署名チェックなし
    pub operator: UncheckedAccount<'info>,
}
