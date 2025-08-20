use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

declare_id!("Vst111111111111111111111111111111111111");

#[program]
pub mod token_vesting_safe {
    use super::*;

    /// ベスティングアカウントの初期化
    /// └ トークンをベネフィシャリー→Vault PDA に必ず預託する
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        release_timestamp: i64,
        total_amount: u64,
    ) -> Result<()> {
        let vest = &mut ctx.accounts.vesting;
        vest.beneficiary   = ctx.accounts.beneficiary.key();     // 署名者チェック用
        vest.vault_mint    = ctx.accounts.vault_mint.key();      // PDA seeds にも使う
        vest.release_time  = release_timestamp;
        vest.remaining     = total_amount;
        vest.bump          = *ctx.bumps.get("vesting").unwrap();

        // ── トークン預託 CPI ──
        let cpi_accounts = Transfer {
            from:      ctx.accounts.beneficiary_token_account.to_account_info(),
            to:        ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.beneficiary.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        transfer(cpi_ctx, total_amount)?;

        Ok(())
    }

    /// トークン請求
    /// └ Release 時刻を超えていれば、PDA Vault→Dest に確実に移動
    pub fn claim(ctx: Context<ClaimVesting>) -> Result<()> {
        let vest = &mut ctx.accounts.vesting;
        let now  = Clock::get()?.unix_timestamp;
        require!(now >= vest.release_time, ErrorCode::TooEarly);

        let amount = vest.remaining;
        vest.remaining = 0;

        // ── PDA Vault から Dest への CPI ──
        let seeds = &[
            b"vesting".as_ref(),
            vest.beneficiary.as_ref(),
            vest.vault_mint.as_ref(),
            &[vest.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from:      ctx.accounts.vault_token_account.to_account_info(),
            to:        ctx.accounts.dest_token_account.to_account_info(),
            authority: ctx.accounts.vesting.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(release_timestamp: i64, total_amount: u64)]
pub struct InitializeVesting<'info> {
    /// ベスティング情報を保持する PDA
    #[account(
        init,
        payer  = beneficiary,
        seeds  = [b"vesting", beneficiary.key().as_ref(), vault_mint.key().as_ref()],
        bump,
        space  = 8 + 32 + 32 + 8 + 8 + 1
    )]
    pub vesting: Account<'info, VestingAccount>,

    /// ベネフィシャリーのトークン預託元
    #[account(
        mut,
        constraint = beneficiary_token_account.owner == *beneficiary.key,
        constraint = beneficiary_token_account.mint == vault_mint.key()
    )]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    /// Vault 用 TokenAccount（PDA、authority＝vesting）
    #[account(
        mut,
        seeds      = [b"vault", vesting.key().as_ref()],
        bump,
        constraint = vault_token_account.owner == vesting.key(),
        constraint = vault_token_account.mint  == vault_mint.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// 請求権者（署名者）
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    /// 扱うトークンのミント
    pub vault_mint: Account<'info, anchor_spl::token::Mint>,

    /// CPI 呼び出し先を固定
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
    pub clock:          Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ClaimVesting<'info> {
    /// 既存の vesting PDA（has_one で beneficiary==署名者 を担保）
    #[account(
        mut,
        has_one   = beneficiary,
        seeds     = [b"vesting", beneficiary.key().as_ref(), vault_mint.key().as_ref()],
        bump      = vesting.bump
    )]
    pub vesting: Account<'info, VestingAccount>,

    /// Vault PDA の TokenAccount（PDA／authority＝vesting／mintチェック）
    #[account(
        mut,
        seeds      = [b"vault", vesting.key().as_ref()],
        bump      = vesting.bump,
        constraint = vault_token_account.owner == vesting.key(),
        constraint = vault_token_account.mint  == vesting.vault_mint
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// 受取先 TokenAccount（署名者が所有、mintチェック）
    #[account(
        mut,
        constraint = dest_token_account.owner == *beneficiary.key,
        constraint = dest_token_account.mint  == vesting.vault_mint
    )]
    pub dest_token_account: Account<'info, TokenAccount>,

    /// 実際に請求するユーザー（署名者）
    pub beneficiary: Signer<'info>,

    pub vault_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VestingAccount {
    pub beneficiary:  Pubkey,  // has_one でチェック
    pub vault_mint:   Pubkey,  // seeds に使用
    pub release_time: i64,
    pub remaining:    u64,
    pub bump:         u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("リリース時間前です")]
    TooEarly,
    #[msg("不正な演算が発生しました")]
    Underflow,
}
