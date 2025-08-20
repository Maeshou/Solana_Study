use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("SaFeUt1lCPI1111111111111111111111111111");
const UTIL_ID: Pubkey = pubkey!("Ut1lProg00000000000000000000000000000000");

#[event]
pub struct UtilSent { pub chunks: u8, pub total: u32 }

#[program]
pub mod safe_util_call {
    use super::*;

    pub fn send_chunks(ctx: Context<SendChunks>, payload: Vec<u8>, chunk: u16) -> Result<()> {
        // 固定ID（差し替え不可）
        let pid = UTIL_ID;

        // チャンク長の下限/上限
        let mut size = chunk as usize;
        if size < 16 { size = 16; }
        if size > 256 { size = 256; }

        // 分割して複数回invoke
        let total_len = payload.len() as u32;
        let mut offset: usize = 0;
        let mut sent: u8 = 0;

        while offset < payload.len() {
            let end = if offset + size > payload.len() { payload.len() } else { offset + size };
            let piece = &payload[offset..end];

            // ヘッダ: [offset(4)][len(2)][crc(1)] + data
            let mut data = Vec::with_capacity(7 + piece.len());
            data.extend_from_slice(&(offset as u32).to_le_bytes());
            data.extend_from_slice(&(piece.len() as u16).to_le_bytes());
            let mut crc: u8 = 0;
            for b in piece { crc ^= *b; }
            data.push(crc);
            data.extend_from_slice(piece);

            let metas = vec![
                AccountMeta::new(ctx.accounts.target_cell.key(), false),
                AccountMeta::new_readonly(ctx.accounts.actor.key(), false),
            ];
            let ix = Instruction { program_id: pid, accounts: metas, data };

            invoke(
                &ix,
                &[
                    ctx.accounts.util_hint.to_account_info(),
                    ctx.accounts.target_cell.to_account_info(),
                    ctx.accounts.actor.to_account_info(),
                ],
            )?;

            sent = sent.saturating_add(1);
            offset = end;
        }

        emit!(UtilSent { chunks: sent, total: total_len });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendChunks<'info> {
    /// CHECK:
    pub util_hint: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub target_cell: AccountInfo<'info>,
    /// CHECK:
    pub actor: AccountInfo<'info>,
}
