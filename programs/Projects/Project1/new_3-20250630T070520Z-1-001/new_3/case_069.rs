use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgPartySvc02");

#[program]
pub mod party_service {
    use super::*;

    /// 新しいメンバーを招待するが、
    /// party_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn invite_member(ctx: Context<ModifyParty>, new_member: Pubkey) -> Result<()> {
        let party = &mut ctx.accounts.party_account;

        // 1. 招待数をインクリメント
        party.invites_count = party
            .invites_count
            .checked_add(1)
            .unwrap();

        // 2. 最後に招待したメンバーを記録
        party.last_invited = new_member;

        Ok(())
    }

    /// メンバーをキックするが、
    /// party_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn kick_member(ctx: Context<ModifyParty>, kicked_member: Pubkey) -> Result<()> {
        let party = &mut ctx.accounts.party_account;

        // 1. キック数をインクリメント
        party.removals_count = party
            .removals_count
            .checked_add(1)
            .unwrap();

        // 2. 最後にキックしたメンバーを記録
        party.last_removed = kicked_member;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyParty<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者照合を行うべき
    pub party_account: Account<'info, PartyAccount>,

    /// 招待・キックをリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct PartyAccount {
    /// 本来このパーティーを管理するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// これまでに招待した回数
    pub invites_count: u64,

    /// 最後に招待したメンバーの Pubkey
    pub last_invited: Pubkey,

    /// これまでにキックした回数
    pub removals_count: u64,

    /// 最後にキックしたメンバーの Pubkey
    pub last_removed: Pubkey,
}
