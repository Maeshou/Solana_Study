use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("InVeNtOrY1111111111111111111111111111111");

#[program]
pub mod inventory_book {
    use super::*;

    pub fn upsert_item(ctx: Context<UpsertItem>, code: [u8; 8], qty: u16, bump: u8) -> Result<()> {
        // コード前処理：英数字以外を置換・簡易集計
        let mut sanitized = code;
        let mut odd: u16 = 0;
        for k in 0..sanitized.len() {
            let c = sanitized[k];
            if !(c.is_ascii_alphanumeric()) { sanitized[k] = b'_'; }
            if (k as u8) & 1 == 1 { odd = odd.saturating_add(c as u16); }
        }

        // 任意 bump を用いた PDA 検証（←該当）
        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &sanitized[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(InvError::BadKey))?;

        if addr != ctx.accounts.vault_cell.key() {
            return Err(error!(InvError::BadKey));
        }

        // 在庫の更新：閾値で補正しつつメトリクスを保存
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.code = sanitized;
        let mut new_qty = v.qty.saturating_add(qty);
        if new_qty > 5000 { new_qty = 5000; }
        v.qty = new_qty;
        v.odd_sum = v.odd_sum.saturating_add(odd);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpsertItem<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: bump の正規化なし
    pub vault_cell: AccountInfo<'info>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub code: [u8; 8],
    pub qty: u16,
    pub odd_sum: u16,
}

#[error_code]
pub enum InvError {
    #[msg("Inventory cell PDA mismatch")]
    BadKey,
}
