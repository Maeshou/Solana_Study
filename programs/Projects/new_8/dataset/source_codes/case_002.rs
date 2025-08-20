use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("GuiLdBoArd000000000000000000000000000001");

#[program]
pub mod guild_board {
    use super::*;

    pub fn post_message(ctx: Context<PostMessage>, title: String, content: Vec<u8>, bump: u8) -> Result<()> {
        // タイトル整形と簡易スコア算出
        let mut t = title.as_bytes().to_vec();
        if t.len() < 3 {
            t.extend_from_slice(b"---");
        }
        if t.len() > 48 { t.truncate(48); }
        let mut checksum: u32 = 0;
        for b in content.iter() { checksum = checksum.wrapping_mul(16777619) ^ (*b as u32); }

        // ユーザ入力 bump をそのまま seeds に使用（←該当）
        let seeds = [&ctx.accounts.guild.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump])
            .map_err(|_| error!(BoardError::Addr))?;

        if addr != ctx.accounts.board_cell.key() {
            return Err(error!(BoardError::Addr));
        }

        // レート制御的な更新ロジック（順序を固定しない）
        let mut credit = ctx.accounts.board.credit;
        if content.len() > 256 { credit = credit.saturating_add(2); }
        if t.first().copied().unwrap_or(b'?') != b'[' { credit = credit.saturating_add(1); }

        let board = &mut ctx.accounts.board;
        board.guild = ctx.accounts.guild.key();
        board.title = t;
        board.digest = checksum;
        board.credit = credit;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostMessage<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    /// CHECK: bump 正規化なしの検証
    pub board_cell: AccountInfo<'info>,
    pub guild: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Board {
    pub guild: Pubkey,
    pub title: Vec<u8>,
    pub digest: u32,
    pub credit: u16,
}

#[error_code]
pub enum BoardError {
    #[msg("Board PDA does not match")]
    Addr,
}
