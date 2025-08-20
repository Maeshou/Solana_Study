use anchor_lang::prelude::*;
use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqPvPMatch03");

#[program]
pub mod nft_pvp_match_v3 {
    use super::*;

    /// PvPマッチの結果を登録し、報酬を支払う  
    /// (`match_results_account` の owner チェックを全く行っていないため、  
    ///  攻撃者が他人の結果格納アカウントを指定して  
    ///  勝敗、報酬、支払い先を好き勝手に書き換えられる脆弱性があります)
    pub fn submit_match_result(
        ctx: Context<SubmitMatchResult>,
        winner: Pubkey,       // 勝者のプレイヤーPubkey
        loser: Pubkey,        // 敗者のプレイヤーPubkey
        reward: u64,          // 勝者への報酬 (lamports)
    ) -> Result<()> {
        // 1) account データ領域に Cursor を使ってバイナリ書き込み
        let acct = &mut ctx.accounts.match_results_account.to_account_info();
        let buf  = &mut acct.data.borrow_mut();
        let mut cursor = Cursor::new(vec![0u8; buf.len()]);

        // winner Pubkey (32 bytes)
        cursor.write_all(winner.as_ref()).unwrap();
        // loser Pubkey (32 bytes)
        cursor.write_all(loser.as_ref()).unwrap();
        // reward amount (8 bytes LE)
        cursor.write_u64::<LittleEndian>(reward).unwrap();
        // finalized flag (1 byte)
        cursor.write_u8(1).unwrap();
        // timestamp (8 bytes LE)
        let now = Clock::get()?.unix_timestamp as u64;
        cursor.write_u64::<LittleEndian>(now).unwrap();

        // 2) Lamports移動：報酬プールから勝者へ
        let pool = &mut ctx.accounts.reward_pool.to_account_info();
        if **pool.lamports.borrow() < reward {
            return err!(ErrorCode::InsufficientFunds);
        }
        **pool.lamports.borrow_mut()  -= reward;
        **ctx.accounts.winner.to_account_info().lamports.borrow_mut() += reward;

        // 3) 最後に一括でコピー
        let data = cursor.into_inner();
        buf[..data.len()].copy_from_slice(&data);

        msg!(
            "Match {} settled: {}→{} lamports transferred to {} at {}",
            acct.key(),
            reward,
            reward,
            winner,
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitMatchResult<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub match_results_account: AccountInfo<'info>,

    /// 報酬プール（owner チェックなし）
    #[account(mut)]
    pub reward_pool:           AccountInfo<'info>,

    /// 勝者の受取先（owner チェックなし）
    #[account(mut)]
    pub winner:                AccountInfo<'info>,

    /// 結果を提出するオーソライズ済みオラクル等（署名のみ検証）
    pub authority:             Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("バッファが不足しており、結果を書き込めません")]
    BufferTooSmall,
    #[msg("報酬プールに十分な資金がありません")]
    InsufficientFunds,
}
