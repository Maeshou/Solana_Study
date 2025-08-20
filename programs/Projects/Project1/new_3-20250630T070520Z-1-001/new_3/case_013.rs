use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDeckMgmt001");

#[program]
pub mod deck_management {
    use super::*;

    /// デッキ情報を更新するが、所有者アカウントとの照合がない
    pub fn configure_deck(
        ctx: Context<ConfigureDeck>,
        new_name: String,
        main_card: Pubkey,
        side_card: Pubkey,
    ) -> Result<()> {
        let deck = &mut ctx.accounts.deck_account;

        // ↓ 本来は deck.owner と ctx.accounts.user.key() の一致を検証すべき
        deck.name = new_name;
        deck.main_card = main_card;
        deck.side_card = side_card;

        // レベルボーナスを固定値分だけ加算
        deck.power_level = deck.power_level.checked_add(5).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureDeck<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを入れるべき
    pub deck_account: Account<'info, Deck>,
    /// デッキ所有者としての署名者
    pub user: Signer<'info>,
}

#[account]
pub struct Deck {
    /// このデッキを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// デッキ名
    pub name: String,
    /// メインカードのミントアドレス
    pub main_card: Pubkey,
    /// サイドカードのミントアドレス
    pub side_card: Pubkey,
    /// デッキの総合パワーレベル
    pub power_level: u64,
}
