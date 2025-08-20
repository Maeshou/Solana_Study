use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgReviveSvc01");

#[program]
pub mod revival_service {
    use super::*;

    /// キャラクターを復活させ、ヘルスを最大に戻すが、
    /// character_account.owner と ctx.accounts.player.key() の照合チェックがない
    pub fn revive_character(ctx: Context<ReviveCharacter>) -> Result<()> {
        let char_acc = &mut ctx.accounts.character_account;
        let cfg = &ctx.accounts.config;

        // 1. ヘルスを最大値に設定
        char_acc.health = cfg.max_health;

        // 2. 復活コストを徴収
        let cost = cfg.revive_cost;
        **ctx.accounts.player.to_account_info().lamports.borrow_mut() -= cost;
        **ctx.accounts.treasury.to_account_info().lamports.borrow_mut() += cost;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReviveCharacter<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを行うべき
    pub character_account: Account<'info, CharacterAccount>,

    /// 復活をリクエストするプレイヤー（署名者）
    #[account(mut)]
    pub player: Signer<'info>,

    /// 復活コスト受取口座
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// 復活条件（最大ヘルス・コスト）を保持する設定アカウント
    pub config: Account<'info, RevivalConfig>,
}

#[account]
pub struct CharacterAccount {
    /// 本来このキャラクターを所有するべきプレイヤーの Pubkey
    pub owner: Pubkey,
    /// 現在のヘルス値
    pub health: u64,
}

#[account]
pub struct RevivalConfig {
    /// 復活後の最大ヘルス
    pub max_health: u64,
    /// 復活に必要なコスト（Lamports）
    pub revive_cost: u64,
}
