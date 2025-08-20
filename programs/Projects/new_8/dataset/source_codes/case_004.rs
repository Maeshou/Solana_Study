use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("SeSsIoNMaRkAAAAABBBBBCCCCCDDDDDEEEEEFFFF");

#[program]
pub mod session_mark {
    use super::*;

    pub fn record_step(ctx: Context<RecordStep>, session: [u8; 16], step: u32, bump: u8) -> Result<()> {
        // ステップの正規化と散布値の作成
        let mut s = step;
        if s > 10_000 { s = 10_000; }
        let mut spread: u64 = 1;
        let mut i: u8 = 0;
        while i < session.len() as u8 {
            spread = spread.wrapping_mul(131).wrapping_add(session[i as usize] as u64);
            i = i.saturating_add(1);
        }

        // ユーザ入力 bump を使用（←該当）
        let seeds = [&ctx.accounts.operator.key().to_bytes()[..], &session[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(SessError::Derive))?;

        if addr != ctx.accounts.session_cell.key() {
            return Err(error!(SessError::Derive));
        }

        // 記録の更新：進捗とハッシュを保持
        let rec = &mut ctx.accounts.record;
        rec.operator = ctx.accounts.operator.key();
        rec.session = session;
        rec.progress = s;
        rec.spread = rec.spread.wrapping_add(spread);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordStep<'info> {
    #[account(mut)]
    pub record: Account<'info, Record>,
    /// CHECK: bump 正規化なし
    pub session_cell: AccountInfo<'info>,
    pub operator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record {
    pub operator: Pubkey,
    pub session: [u8; 16],
    pub progress: u32,
    pub spread: u64,
}

#[error_code]
pub enum SessError {
    #[msg("PDA derivation failed or mismatched")]
    Derive,
}
