use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfARTGAL");

#[program]
pub mod art_gallery {
    use super::*;

    /// 新しいアートギャラリーを作成（クリエイター署名必須）
    pub fn initialize_art(
        ctx: Context<InitializeArt>,
        art_id: String,
    ) -> Result<()> {
        let gallery = &mut ctx.accounts.gallery;
        gallery.creator = ctx.accounts.creator.key();
        gallery.art_id = art_id.clone();
        gallery.total_contributions = 0;
        gallery.contributor_count = 0;
        msg!("Gallery '{}' initialized by {}", art_id, gallery.creator);
        Ok(())
    }

    /// どなたでも署名付きで作品に寄付できる（amount > 0）
    pub fn contribute(
        ctx: Context<Contribute>,
        amount: u64,
    ) -> Result<()> {
        require!(ctx.accounts.contributor.is_signer, ErrorCode::Unauthorized);
        require!(amount > 0, ErrorCode::InvalidAmount);

        let gallery = &mut ctx.accounts.gallery;
        gallery.total_contributions = gallery
            .total_contributions
            .checked_add(amount)
            .unwrap();
        gallery.contributor_count = gallery
            .contributor_count
            .checked_add(1)
            .unwrap();

        msg!(
            "💖 {} contributed {} units to '{}'",
            ctx.accounts.contributor.key(),
            amount,
            gallery.art_id
        );
        Ok(())
    }

    /// クリエイターのみが呼べる：アートの新しいクリエイターに変更
    pub fn change_creator(
        ctx: Context<ChangeCreator>,
        new_creator: Pubkey,
    ) -> Result<()> {
        require!(ctx.accounts.creator.is_signer, ErrorCode::Unauthorized);
        let gallery = &mut ctx.accounts.gallery;
        gallery.creator = new_creator;
        msg!("Creator for '{}' changed to {}", gallery.art_id, new_creator);
        Ok(())
    }

    /// 誰でも現在の寄付状況を確認できる
    pub fn view_stats(
        ctx: Context<ViewStats>,
    ) -> Result<()> {
        let gallery = &ctx.accounts.gallery;
        msg!(
            "Gallery '{}': total={} from {} contributors",
            gallery.art_id,
            gallery.total_contributions,
            gallery.contributor_count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeArt<'info> {
    /// PDA を使ってギャラリーアカウントを初期化
    #[account(
        init,
        payer = creator,
        space  = 8 + 32 + 4 + 64 + 8 + 8,
        seeds  = [b"gallery", art_id.as_bytes()],
        bump
    )]
    pub gallery: Account<'info, GalleryAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    /// 既存のギャラリーアカウントを更新
    #[account(
        mut,
        seeds = [b"gallery", gallery.art_id.as_bytes()],
        bump
    )]
    pub gallery: Account<'info, GalleryAccount>,

    /// 寄付者署名必須
    pub contributor: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangeCreator<'info> {
    /// ギャラリー所有者署名チェック
    #[account(
        mut,
        seeds = [b"gallery", gallery.art_id.as_bytes()],
        bump,
        has_one = creator
    )]
    pub gallery: Account<'info, GalleryAccount>,

    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewStats<'info> {
    /// ギャラリーの統計を読むだけ
    #[account(
        seeds = [b"gallery", gallery.art_id.as_bytes()],
        bump
    )]
    pub gallery: Account<'info, GalleryAccount>,
}

#[account]
pub struct GalleryAccount {
    /// アート所有者
    pub creator: Pubkey,
    /// ギャラリー識別子
    pub art_id: String,
    /// 総寄付額
    pub total_contributions: u64,
    /// 寄付者数
    pub contributor_count: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Invalid amount: must be > 0")]
    InvalidAmount,
}
