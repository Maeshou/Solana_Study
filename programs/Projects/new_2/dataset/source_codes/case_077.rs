use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqGacha01");

#[program]
pub mod nft_gacha {
    use super::*;

    /// ガチャプールからランダムに NFT を引く  
    /// （`gacha_pool_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人のプールアカウントを指定してレア NFT を独占入手できます）
    pub fn draw_nft(ctx: Context<DrawNft>, user_seed: u64) -> Result<Pubkey> {
        let pool_info = &ctx.accounts.gacha_pool_account.to_account_info();
        let pool_data = &pool_info.data.borrow();

        // ── レイアウト想定 ──
        // [0..8]              : u64 プールサイズ N
        // [8..8+32*N]         : Pubkey × N NFT リスト
        // [8+32*N..]          : 各 NFT のレアリティ値 (u8 × N)

        // プールサイズ読み取り
        if pool_data.len() < 8 {
            return err!(ErrorCode::DataTooShort);
        }
        let n = u64::from_le_bytes(pool_data[0..8].try_into().unwrap()) as usize;

        // NFT Pubkey リストとレアリティ配列を切り出し
        let keys_start = 8;
        let keys_end   = 8 + 32 * n;
        let rar_start  = keys_end;
        let rar_end    = rar_start + n;
        if pool_data.len() < rar_end {
            return err!(ErrorCode::DataTooShort);
        }
        let key_bytes = &pool_data[keys_start..keys_end];
        let rarity    = &pool_data[rar_start..rar_end];

        // 簡易 RNG: ユーザーシード + 現在スロット混ぜ込み
        let slot = Clock::get()?.slot as u64;
        let idx = ((user_seed ^ slot) as usize) % n;

        // レア度判定（例：rarity[idx] ≥ threshold ならレア扱い）
        let chosen_key = Pubkey::new(&key_bytes[idx*32..idx*32+32]);
        let chosen_rarity = rarity[idx];

        msg!(
            "Gacha draw: user_seed={} slot={} idx={} rarity={}",
            user_seed,
            slot,
            idx,
            chosen_rarity
        );
        Ok(chosen_key)
    }
}

#[derive(Accounts)]
pub struct DrawNft<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub gacha_pool_account: AccountInfo<'info>,

    /// ガチャを引くユーザー（署名のみ検証）
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("プールアカウントのデータ長が不足しています")]
    DataTooShort,
}
