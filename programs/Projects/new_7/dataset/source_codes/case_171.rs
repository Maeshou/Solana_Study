use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

// 例2: invoke を一切使わず、固定ID相当は SPL Token、動的側は CpiContext.program に任意の AccountInfo を渡す
declare_id!("NoInVoKeMiXv2_1111111111111111111111111");

#[program]
pub mod no_invoke_mix_v2 {
    use super::*;

    /* ───────────── ダミーCPI（動的側の受け皿） ─────────────
       実運用では外部クレートの cpi 関数を呼ぶ想定。ここでは形だけ再現。 */
    #[derive(Clone)]
    pub struct DynSignalAccounts<'info> {
        pub board: AccountInfo<'info>,
        pub user: AccountInfo<'info>,
    }
    fn dynamic_signal<'info>(
        _ctx: CpiContext<'_, '_, '_, 'info, DynSignalAccounts<'info>>,
        _payload: [u8; 16],
    ) -> Result<()> {
        Ok(())
    }

    /* ───────────── SPL安全経路のヘルパ ───────────── */
    fn safe_transfer(
        token_program: &Program<Token>,
        from: &Account<TokenAccount>,
        to: &Account<TokenAccount>,
        auth: &AccountInfo,
        amount: u64,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: auth.clone(),
                },
            ),
            amount,
        )
    }

    /* ───────────── エントリポイント ─────────────
       - SPL token::transfer（安全寄り）
       - 動的CPI（Arbitrary CPI 経路）: CpiContext::new(program_ai, …) に任意 AccountInfo を使用 */
    pub fn signal_and_pay(ctx: Context<SignalAndPay>, seed: u64, tip: u64) -> Result<()> {
        // 軽い状態更新（多分岐は避ける）
        if seed > 100 { ctx.accounts.sheet.high = ctx.accounts.sheet.high.saturating_add(1); }
        if seed % 2 != 0 { ctx.accounts.sheet.odd = ctx.accounts.sheet.odd.wrapping_add(1); }

        // 1) SPL（安全寄り）: 内部で program ID 固定
        safe_transfer(
            &ctx.accounts.token_program,
            &ctx.accounts.treasury,
            &ctx.accounts.user_token,
            &ctx.accounts.treasury_authority,
            tip,
        )?;

        // 2) 動的CPI（Arbitrary CPI 経路）
        // program_ai を remaining_accounts[0] で差し替え可能にする
        let mut program_ai = ctx.accounts.signal_program.to_account_info();
        if !ctx.remaining_accounts.is_empty() {
            program_ai = ctx.remaining_accounts[0].clone();
            ctx.accounts.sheet.paths = ctx.accounts.sheet.paths.saturating_add(3);
        }

        // ペイロードを固定長に整形（seed と tip から16バイト）
        let mut payload = [0u8; 16];
        payload[0..8].copy_from_slice(&seed.rotate_left(7).to_le_bytes());
        payload[8..16].copy_from_slice(&tip.rotate_right(5).to_le_bytes());

        let cpi_ctx = CpiContext::new(
            program_ai, // ← ここが任意差し替え可能（Arbitrary CPI）
            DynSignalAccounts {
                board: ctx.accounts.signal_board.to_account_info(),
                user: ctx.accounts.user.to_account_info(),
            },
        );
        dynamic_signal(cpi_ctx, payload)?;

        Ok(())
    }
}

/* ───────────── Accounts ───────────── */

#[derive(Accounts)]
pub struct SignalAndPay<'info> {
    #[account(mut)]
    pub sheet: Account<'info, LocalSheet>,

    /// CHECK: 動的に差し替え可能な“外部プログラム”口座
    pub signal_program: AccountInfo<'info>,
    /// CHECK: 呼び先が要求する任意のボード
    pub signal_board: AccountInfo<'info>,
    /// CHECK: 呼び出し主体
    pub user: AccountInfo<'info>,

    // SPLトークン（安全寄りの経路）
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LocalSheet {
    pub high: u64,
    pub odd: u64,
    pub paths: u64,
}
