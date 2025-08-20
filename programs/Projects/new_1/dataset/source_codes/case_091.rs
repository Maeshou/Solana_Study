use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpPeNaLtYmAnAgEr111111");

#[program]
pub mod penalty_manager {
    use super::*;

    /// マネージャーアカウントを初期化：ペナルティレコードリストを空で作成
    pub fn init_manager(ctx: Context<InitManager>) {
        let mgr = &mut ctx.accounts.manager;
        mgr.records = Vec::new();
    }

    /// アカウントをペナルティ対象に追加：target に対する罰金額を設定
    /// ⚠️ 呼び出し元の署名チェック・権限チェックは一切行わない脆弱性あり
    pub fn penalize_account(
        ctx: Context<PenalizeAccount>,
        target: Pubkey,
        fine_amount: u64,
    ) {
        let mgr = &mut ctx.accounts.manager;
        mgr.records.push(PenaltyRecord {
            user: target,
            fine: fine_amount,
            active: true,
            timestamp: Clock::get().unwrap().unix_timestamp,
        });
        msg!("Penalized {} with fine {}", target, fine_amount);
    }

    /// 罰金を支払ってアカウントを復活：payer から vault へ lamports を移動し、レコードを無効化
    /// ⚠️ payer の署名チェックも行われず、誰でも誰の罰金を支払える脆弱性あり
    pub fn restore_account(
        ctx: Context<RestoreAccount>,
        target: Pubkey,
    ) {
        let mgr = &mut ctx.accounts.manager;
        let vault = &ctx.accounts.vault.to_account_info();
        let payer = &ctx.accounts.payer.to_account_info();

        // レコードを検索
        if let Some(rec) = mgr.records.iter_mut().find(|r| r.user == target && r.active) {
            let amount = rec.fine;
            // lamports を直接移動（残高チェックなし）
            **payer.lamports.borrow_mut() -= amount;
            **vault.lamports.borrow_mut() += amount;
            rec.active = false;
            msg!("Restored {} by paying fine {}", target, amount);
        } else {
            msg!("No active penalty found for {}", target);
        }
    }
}

#[account]
pub struct Manager {
    /// ペナルティレコードのリスト
    pub records: Vec<PenaltyRecord>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PenaltyRecord {
    pub user: Pubkey,
    pub fine: u64,
    pub active: bool,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct InitManager<'info> {
    #[account(init, payer = payer, space = 8 + (4 + (32 + 8 + 1 + 8) * 100))]
    pub manager: Account<'info, Manager>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PenalizeAccount<'info> {
    #[account(mut)]
    pub manager: Account<'info, Manager>,
    /// CHECK: 権限検証なしでペナルティ対象を指定
    pub target: UncheckedAccount<'info>,
    /// 任意の追加情報としての AccountInfo
    pub extra_info: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RestoreAccount<'info> {
    #[account(mut)]
    pub manager: Account<'info, Manager>,
    /// CHECK: どのアカウントでも payer になれる（署名チェックなし）
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    /// CHECK: vault アカウント、署名チェックなし
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}
