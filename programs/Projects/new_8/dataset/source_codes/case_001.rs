use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ProF1leUpDaTe111111111111111111111111111");

#[program]
pub mod profile_update {
    use super::*;

    pub fn set_profile(ctx: Context<SetProfile>, handle: String, bump: u8, salt: [u8; 4]) -> Result<()> {
        // ハンドルの正規化（長さ制限とトリム）
        let mut h = handle.as_bytes().to_vec();
        if h.len() > 24 { h.truncate(24); }
        while let Some(b) = h.last() {
            if *b == b' ' { h.pop(); } else { break; }
        }

        // ユーザ入力の bump をそのまま使用（←ここが該当ポイント）
        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &h[..], &salt[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(ErrorCode::BadPda))?;

        // data_account と一致しているかだけを比較（bump の正規化なし）
        if addr != ctx.accounts.data_account.key() {
            return Err(error!(ErrorCode::BadPda));
        }

        // 疑似的なプロフィール更新処理（大小文字の整形・カウント）
        let mut upper_count: u16 = 0;
        let mut bytes = h.clone();
        for x in bytes.iter_mut() {
            if *x >= b'a' && *x <= b'z' {
                *x = *x - 32;
            } else if *x >= b'A' && *x <= b'Z' {
                upper_count = upper_count.saturating_add(1);
            }
        }

        let p = &mut ctx.accounts.profile;
        p.authority = ctx.accounts.owner.key();
        p.handle = bytes;
        p.upper = upper_count;
        p.salt = salt;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetProfile<'info> {
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    /// CHECK: bump の正規化なしで PDA を検証
    pub data_account: AccountInfo<'info>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Profile {
    pub authority: Pubkey,
    pub handle: Vec<u8>,
    pub upper: u16,
    pub salt: [u8; 4],
}

#[error_code]
pub enum ErrorCode {
    #[msg("Derived PDA mismatch")]
    BadPda,
}
