use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfTIPJAR");

#[program]
pub mod tip_jar {
    use super::*;

    /// クリエイターだけが呼べるジャー口座を初期化
    pub fn initialize_jar(ctx: Context<InitializeJar>) -> Result<()> {
        let jar = &mut ctx.accounts.jar;
        jar.creator = ctx.accounts.creator.key();
        jar.total_tips = 0;
        Ok(())
    }

    /// どなたでも署名を伴ってチップを送れる（amountは1以上）
    pub fn tip(ctx: Context<Tip>, amount: u64) -> Result<()> {
        // 送金者の署名チェック
        require!(ctx.accounts.tip_payer.is_signer, ErrorCode::Unauthorized);
        // 正の金額のみ許可
        require!(amount > 0, ErrorCode::InvalidAmount);
        let jar = &mut ctx.accounts.jar;
        jar.total_tips = jar.total_tips.checked_add(amount).unwrap();
        msg!(
            "❤ {} tipped {} units. New total: {}",
            ctx.accounts.tip_payer.key(),
            amount,
            jar.total_tips
        );
        Ok(())
    }

    /// クリエイターだけが全部引き出せる
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        // クリエイター署名チェック
        require!(ctx.accounts.creator.is_signer, ErrorCode::Unauthorized);
        let jar = &mut ctx.accounts.jar;
        // 引き出すチップがあるか
        require!(jar.total_tips > 0, ErrorCode::NoTipsLeft);
        let to_send = jar.total_tips;
        jar.total_tips = 0;
        msg!(
            "💰 {} withdrew {} units",
            ctx.accounts.creator.key(),
            to_send
        );
        Ok(())
    }

    /// 現在のジャー残高を見るだけ
    pub fn view_balance(ctx: Context<ViewBalance>) -> Result<()> {
        let jar = &ctx.accounts.jar;
        msg!(
            "TipJar for {} has {} units",
            jar.creator,
            jar.total_tips
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeJar<'info> {
    /// 初回だけ PDA を init
    #[account(
        init,
        payer = creator,
        space  = 8 + 32 + 8,
        seeds  = [b"jar", creator.key().as_ref()],
        bump
    )]
    pub jar: Account<'info, TipJarAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    /// PDA とクリエイターは関係ないが jar 自体を更新
    #[account(
        mut,
        seeds = [b"jar", jar.creator.as_ref()],
        bump
    )]
    pub jar: Account<'info, TipJarAccount>,

    /// チップを払う人
    pub tip_payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// PDA のクリエイター署名を確認
    #[account(
        mut,
        seeds = [b"jar", creator.key().as_ref()],
        bump,
        has_one = creator
    )]
    pub jar: Account<'info, TipJarAccount>,

    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewBalance<'info> {
    /// どなたでも残高を見られる
    #[account(
        seeds = [b"jar", jar.creator.as_ref()],
        bump
    )]
    pub jar: Account<'info, TipJarAccount>,
}

#[account]
pub struct TipJarAccount {
    /// チップ受取人
    pub creator: Pubkey,
    /// 累計チップ量
    pub total_tips: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: signer required")]
    Unauthorized,
    #[msg("Invalid amount: must be > 0")]
    InvalidAmount,
    #[msg("No tips left to withdraw")]
    NoTipsLeft,
}
