use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTournSrv001");

#[program]
pub mod tournament_service {
    use super::*;

    /// トーナメントへの参加登録を行うが、
    /// EntryAccount.player と ctx.accounts.player.key() の照合チェックがない
    pub fn enter_tournament(ctx: Context<EnterTournament>, tournament_id: u64) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        let tournament = &ctx.accounts.tournament;

        // 1. 参加料を主催者に転送（所有者チェックなし）
        let fee = tournament.fee;
        **ctx.accounts.organizer.to_account_info().lamports.borrow_mut() += fee;
        **ctx.accounts.player.to_account_info().lamports.borrow_mut() -= fee;

        // 2. EntryAccount を更新
        entry.tournament = tournament_id;
        entry.player = ctx.accounts.player.key();
        entry.registered = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnterTournament<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = player)] や constraint で照合すべき
    pub entry: Account<'info, EntryAccount>,

    /// トーナメント設定アカウント（fee フィールドのみ使用）
    pub tournament: Account<'info, Tournament>,

    /// 参加料受取先の主催者アカウント
    #[account(mut)]
    pub organizer: AccountInfo<'info>,

    /// 参加者ユーザー（署名者）。登録データとの照合検証がない
    #[account(mut)]
    pub player: Signer<'info>,
}

#[account]
pub struct EntryAccount {
    /// 登録したトーナメントの ID
    pub tournament: u64,
    /// 登録者の Pubkey
    pub player: Pubkey,
    /// 登録済みフラグ
    pub registered: bool,
}

#[account]
pub struct Tournament {
    /// 参加料（Lamports）
    pub fee: u64,
}
