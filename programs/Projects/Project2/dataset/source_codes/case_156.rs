use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUD");

#[program]
pub mod nft_adventure {
    use super::*;

    /// キャラクター作成：NFT mint アドレスを受け取り、レベル 1 で初期化
    pub fn create_character(
        ctx: Context<CreateCharacter>,
        bump: u8,
        character_id: u64,
        nft_mint: Pubkey,
    ) -> Result<()> {
        let ch = &mut ctx.accounts.character;
        ch.owner        = ctx.accounts.player.key();
        ch.bump         = bump;
        ch.character_id = character_id;
        ch.nft_mint     = nft_mint;
        ch.level        = 1;
        ch.in_quest     = false;
        Ok(())
    }

    /// レベルアップ：オーナーチェック／署名チェック後、レベルを 1 増加
    pub fn level_up(
        ctx: Context<ModifyCharacter>,
    ) -> Result<()> {
        let ch = &mut ctx.accounts.character;
        ch.level = ch.level.wrapping_add(1);
        Ok(())
    }

    /// クエスト開始／終了トグル：in_quest フラグを切り替え
    pub fn toggle_quest(
        ctx: Context<ModifyCharacter>,
    ) -> Result<()> {
        let ch = &mut ctx.accounts.character;
        ch.in_quest = !ch.in_quest;
        Ok(())
    }

    /// キャラクター削除：close 属性でアカウント解放＆残高返却
    pub fn retire_character(
        ctx: Context<RetireCharacter>,
    ) -> Result<()> {
        // 属性だけで閉鎖を実行
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, character_id: u64)]
pub struct CreateCharacter<'info> {
    /// PDA で生成する Character アカウント
    #[account(
        init,
        payer = player,
        // discriminator(8) + owner(32) + bump(1) + character_id(8)
        // + nft_mint(32) + level(1) + in_quest(1)
        space = 8 + 32 + 1 + 8 + 32 + 1 + 1,
        seeds = [b"nftgame", player.key().as_ref(), &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,

    /// トランザクション送信者（プレイヤー）
    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// レベルアップ／クエスト操作共通：PDA／bump／オーナーチェック
#[derive(Accounts)]
pub struct ModifyCharacter<'info> {
    #[account(
        mut,
        seeds = [b"nftgame", owner.key().as_ref(), &character.character_id.to_le_bytes()],
        bump = character.bump,
        has_one = owner
    )]
    pub character: Account<'info, Character>,

    /// キャラクター所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RetireCharacter<'info> {
    /// キャラクター削除時：close 属性でアカウント解放＆残高返却
    #[account(
        mut,
        seeds = [b"nftgame", owner.key().as_ref(), &character.character_id.to_le_bytes()],
        bump = character.bump,
        has_one = owner,
        close = owner
    )]
    pub character: Account<'info, Character>,

    /// キャラクター所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Character データ構造：オーナー、bump、ID、NFT mint、レベル、クエスト状態
#[account]
pub struct Character {
    pub owner: Pubkey,
    pub bump: u8,
    pub character_id: u64,
    pub nft_mint: Pubkey,
    pub level: u8,
    pub in_quest: bool,
}
