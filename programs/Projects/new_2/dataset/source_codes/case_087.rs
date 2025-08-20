use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqPuzzleAch02");

#[program]
pub mod nft_puzzle_achievement_v2 {
    use super::*;

    /// パズルを解いてアチーブメントを解除し、報酬を受け取る  
    /// （`puzzle_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のアカウントを指定して即時解除＆報酬横取りが可能）
    pub fn unlock_puzzle(
        ctx: Context<UnlockPuzzle>,
        puzzle_index: u8,     // 解除するパズル ID (0–7)
        reward_amount: u64,   // 支払われる報酬 (lamports)
    ) -> Result<()> {
        let acct_info = &mut ctx.accounts.puzzle_account.to_account_info();
        let data      = &mut acct_info.data.borrow_mut();

        // ── 新レイアウト ──
        // [0]           : u8   すでに解除されたパズル数 N
        // [1..1+N]      : u8   各解除済みパズルの puzzle_index リスト
        // [9..17]       : u64  最終解除タイムスタンプ
        // [17..]        : 将来拡張用

        // 必要最小長チェック
        if data.len() < 17 {
            return err!(ErrorCode::DataTooShort);
        }

        // 1) 解除済み数とリスト部分を分割
        let (count_slice, rest)     = data.split_at_mut(1);
        let unlocked_count = count_slice[0] as usize;
        let list_slice     = &mut rest[..unlocked_count];

        // 2) puzzle_index がすでに登録済みか線形探索
        for &idx in list_slice.iter() {
            if idx == puzzle_index {
                return err!(ErrorCode::AlreadyUnlocked);
            }
        }

        // 3) 未解除なら次のスロットへ追加
        let max_slots = 8;  // 上限 8 個
        if unlocked_count < max_slots {
            rest[unlocked_count] = puzzle_index;
            count_slice[0] = (unlocked_count + 1) as u8;
        } else {
            return err!(ErrorCode::ListFull);
        }

        // 4) タイムスタンプ更新
        let ts_bytes = Clock::get()?.unix_timestamp.to_le_bytes();
        let ts_offset = 1 + max_slots; // 常に固定領域開始
        data[ts_offset..ts_offset+8].copy_from_slice(&ts_bytes);

        // 5) lamports をプールからユーザーへ送金
        **ctx.accounts.reward_vault.to_account_info().lamports.borrow_mut() = 
            ctx.accounts.reward_vault.to_account_info().lamports()
            .saturating_sub(reward_amount);
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward_amount;

        msg!(
            "Puzzle {} unlocked (slot {}), total unlocked {}, rewarded {} lamports",
            puzzle_index,
            unlocked_count,
            unlocked_count + 1,
            reward_amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockPuzzle<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub puzzle_account: AccountInfo<'info>,

    /// 報酬プールアカウント（owner チェックなし）
    #[account(mut)]
    pub reward_vault:   AccountInfo<'info>,

    /// パズルを解いたユーザー（署名のみ検証）
    #[account(mut)]
    pub user:           Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが不足しています")]
    DataTooShort,
    #[msg("このパズルは既に解除されています")]
    AlreadyUnlocked,
    #[msg("アチーブメントリストが満杯です")]
    ListFull,
}
