use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("EscwSafe11111111111111111111111111111111");

#[program]
pub mod escrow_program_safe {
    use super::*;

    /// 初期化：Maker→Vault にエスクロートークンを転送
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.maker       = ctx.accounts.maker.key();
        escrow.token_mint  = ctx.accounts.mint.key();
        escrow.amount      = amount;
        escrow.bump        = *ctx.bumps.get("escrow_account").unwrap();

        let cpi_accounts = Transfer {
            from:      ctx.accounts.maker_token_account.to_account_info(),
            to:        ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            amount,
        )?;
        Ok(())
    }

    /// キャンセル：Vault→Maker に払い戻し後、自動クローズ
    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow_account;
        let seeds  = &[
            b"escrow".as_ref(),
            escrow.maker.as_ref(),
            escrow.token_mint.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from:      ctx.accounts.vault_token_account.to_account_info(),
            to:        ctx.accounts.maker_token_account.to_account_info(),
            authority: ctx.accounts.escrow_account.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer,
            ),
            escrow.amount,
        )?;
        Ok(())
    }

    /// 受け入れ：Taker→Maker ①、Vault→Taker ②、両方完了後に自動クローズ
    pub fn accept_escrow(ctx: Context<AcceptEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow_account;

        // ① Taker → MakerReceive
        let cpi1 = Transfer {
            from:      ctx.accounts.taker_token_account.to_account_info(),
            to:        ctx.accounts.maker_receive_account.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        token::transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi1),
            escrow.amount,
        )?;

        // ② Vault → TakerReceive
        let seeds  = &[
            b"escrow".as_ref(),
            escrow.maker.as_ref(),
            escrow.token_mint.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi2 = Transfer {
            from:      ctx.accounts.vault_token_account.to_account_info(),
            to:        ctx.accounts.taker_receive_account.to_account_info(),
            authority: ctx.accounts.escrow_account.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi2,
                signer,
            ),
            escrow.amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct InitializeEscrow<'info> {
    /// PDA：["escrow", maker, mint], 再初期化不可
    #[account(
        init,
        payer  = maker,
        seeds  = [b"escrow", maker.key().as_ref(), mint.key().as_ref()],
        bump,
        space  = 8 + 32 + 32 + 8 + 1
    )]
    pub escrow_account:     Account<'info, EscrowAccount>,

    /// Maker（署名者）
    #[account(mut)]
    pub maker:              Signer<'info>,

    /// Maker のトークン保有口座
    #[account(
        mut,
        constraint = maker_token_account.owner == *maker.key,
        constraint = maker_token_account.mint  == mint.key(),
    )]
    pub maker_token_account: Account<'info, TokenAccount>,

    /// エスクロー用 Vault（PDA）のトークン口座
    #[account(
        mut,
        seeds      = [b"vault", escrow_account.key().as_ref()],
        bump,
        constraint = vault_token_account.mint == mint.key(),
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// トークンプログラムを固定
    #[account(address = token::ID)]
    pub token_program:      Program<'info, Token>,

    /// ミント情報
    pub mint:               Account<'info, anchor_spl::token::Mint>,

    pub system_program:     Program<'info, System>,
    pub rent:               Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    /// キャンセルは Maker のみ・自動クローズ
    #[account(
        mut,
        has_one   = maker,
        seeds     = [b"escrow", maker.key().as_ref(), mint.key().as_ref()],
        bump      = escrow_account.bump,
        close     = maker,
    )]
    pub escrow_account:     Account<'info, EscrowAccount>,

    #[account(mut)]
    pub maker:              Signer<'info>,

    /// Maker に戻す口座
    #[account(
        mut,
        constraint = maker_token_account.owner == *maker.key,
        constraint = maker_token_account.mint  == escrow_account.token_mint,
    )]
    pub maker_token_account: Account<'info, TokenAccount>,

    /// Vault トークン口座もクローズ
    #[account(
        mut,
        seeds      = [b"vault", escrow_account.key().as_ref()],
        bump      = escrow_account.bump,
        constraint = vault_token_account.mint == escrow_account.token_mint,
        close     = maker,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(address = token::ID)]
    pub token_program:      Program<'info, Token>,

    pub mint:               Account<'info, anchor_spl::token::Mint>,
}

#[derive(Accounts)]
pub struct AcceptEscrow<'info> {
    /// 受け入れ完了後に Maker に返却
    #[account(
        mut,
        has_one   = maker,
        seeds     = [b"escrow", maker.key().as_ref(), mint.key().as_ref()],
        bump      = escrow_account.bump,
        close     = maker,
    )]
    pub escrow_account:     Account<'info, EscrowAccount>,

    #[account(mut)]
    pub maker:              Signer<'info>,

    /// 支払い元：Taker の口座
    #[account(
        mut,
        constraint = taker_token_account.owner == *taker.key,
        constraint = taker_token_account.mint  == escrow_account.token_mint,
    )]
    pub taker_token_account: Account<'info, TokenAccount>,

    /// Maker が受け取る口座
    #[account(
        mut,
        constraint = maker_receive_account.owner == maker.key(),
        constraint = maker_receive_account.mint  == escrow_account.token_mint,
    )]
    pub maker_receive_account: Account<'info, TokenAccount>,

    /// Taker が受け取る口座
    #[account(
        mut,
        constraint = taker_receive_account.owner == *taker.key,
        constraint = taker_receive_account.mint  == escrow_account.token_mint,
    )]
    pub taker_receive_account: Account<'info, TokenAccount>,

    /// Vault トークン口座もクローズ
    #[account(
        mut,
        seeds      = [b"vault", escrow_account.key().as_ref()],
        bump       = escrow_account.bump,
        constraint = vault_token_account.mint == escrow_account.token_mint,
        close      = maker,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub taker:              Signer<'info>,

    #[account(address = token::ID)]
    pub token_program:      Program<'info, Token>,

    pub mint:               Account<'info, anchor_spl::token::Mint>,
}

#[account]
pub struct EscrowAccount {
    /// Maker の Pubkey（has_one でチェック）
    pub maker:      Pubkey,
    /// 扱うトークンのミント
    pub token_mint: Pubkey,
    /// エスクロー量
    pub amount:     u64,
    /// PDA bump
    pub bump:       u8,
}
