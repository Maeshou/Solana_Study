use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqStakeComp01");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct StakeState {
    staker:    [u8;32],  // ステーカー Pubkey
    amount:    u64,      // 現在のステーク量
    reward_rate: u32,    // 秒あたり報酬レート (1/1_000_000 単位)
    last_ts:   u64,      // 最終更新時刻
}

#[program]
pub mod nft_stake_compound {
    use super::*;

    /// ステーク報酬を複利で再ステークする  
    /// (`stake_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のステーク状態を指定して  
    ///  好きなだけ報酬を上乗せできる脆弱性があります)
    pub fn compound_reward(
        ctx: Context<CompoundReward>,
        current_ts: u64,    // クライアント提供の現在時刻
    ) -> Result<()> {
        let buf = &mut ctx.accounts.stake_account.data.borrow_mut();
        if buf.len() < std::mem::size_of::<StakeState>() {
            return err!(ErrorCode::DataTooShort);
        }
        // バッファ先頭を安全に構造体にマッピング
        let state: &mut StakeState = bytemuck::from_bytes_mut(&mut buf[..std::mem::size_of::<StakeState>()]);

        // 経過時間をクライアント時間で算出（偽装可能）
        let elapsed = current_ts.wrapping_sub(state.last_ts);
        // 簡易計算：reward = amount * rate * elapsed / 1_000_000
        let reward = (state.amount as u128)
            .wrapping_mul(state.reward_rate as u128)
            .wrapping_mul(elapsed as u128)
            / 1_000_000u128;
        // 複利として元本に追加
        state.amount = (state.amount as u128 + reward) as u64;

        // 最終更新時刻をクライアント時間で上書き
        state.last_ts = current_ts;

        msg!(
            "Compound: new stake {} (＋{}), time={} by {}",
            state.amount,
            reward,
            current_ts,
            ctx.accounts.staker.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CompoundReward<'info> {
    /// CHECK: owner チェックをしていない AccountInfo
    #[account(mut)]
    pub stake_account: AccountInfo<'info>,
    /// ステーク所有者の署名のみ検証
    pub staker:        Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("ステークアカウントのデータが不足しています")]
    DataTooShort,
}
