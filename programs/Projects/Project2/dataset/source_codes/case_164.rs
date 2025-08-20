use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUJ");

#[program]
pub mod escrow_service {
    use super::*;

    /// エスクロー初期化：売り手・買い手・金額・期限を受け取り、状態・タイムスタンプをまとめて設定
    pub fn create_escrow(
        ctx: Context<CreateEscrow>,
        bump: u8,
        escrow_id: u64,
        buyer: Pubkey,
        amount: u64,
        deadline: i64,
    ) -> Result<()> {
        *ctx.accounts.escrow = Escrow {
            seller:        ctx.accounts.seller.key(),
            bump,
            escrow_id,
            buyer,
            amount,
            deposited:     false,
            state:         String::from("created"),
            deadline,
            updated_ts:    ctx.accounts.clock.unix_timestamp,
        };
        Ok(())
    }

    /// 入金処理：買い手の署名チェック後、入金フラグとタイムスタンプを更新
    pub fn deposit_funds(
        ctx: Context<DepositFunds>,
    ) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.deposited   = true;
        e.updated_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 売り手による支払い完了：状態とタイムスタンプを更新
    pub fn release_funds(
        ctx: Context<ModifyEscrow>,
    ) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.state       = String::from("released");
        e.updated_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 売り手による返金：状態とタイムスタンプを更新
    pub fn refund_funds(
        ctx: Context<ModifyEscrow>,
    ) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.state       = String::from("refunded");
        e.updated_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, escrow_id: u64, buyer: Pubkey, amount: u64, deadline: i64)]
pub struct CreateEscrow<'info> {
    /// PDA で生成する Escrow アカウント
    #[account(
        init,
        payer = seller,
        // 8 + 32 + 1 + 8 + 32 + 8 + 1 + (4+10) + 8 + 8 = 120 bytes
        space = 8 + 32 + 1 + 8 + 32 + 8 + 1 + 4 + 10 + 8 + 8,
        seeds = [b"escrow", seller.key().as_ref(), &escrow_id.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// 売り手（署名必須）
    #[account(mut)]
    pub seller: Signer<'info>,

    /// 現在のブロック時間を取得
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    /// 既存の Escrow（PDA 検証 + has_one = buyer）
    #[account(
        mut,
        seeds = [b"escrow", escrow.seller.as_ref(), &escrow.escrow_id.to_le_bytes()],
        bump = escrow.bump,
        has_one = buyer
    )]
    pub escrow: Account<'info, Escrow>,

    /// 買い手（署名必須）
    #[account(signer)]
    pub buyer: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ModifyEscrow<'info> {
    /// 既存の Escrow（PDA 検証 + has_one = seller）
    #[account(
        mut,
        seeds = [b"escrow", escrow.seller.as_ref(), &escrow.escrow_id.to_le_bytes()],
        bump = escrow.bump,
        has_one = seller
    )]
    pub escrow: Account<'info, Escrow>,

    /// 売り手（署名必須）
    #[account(signer)]
    pub seller: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Escrow {
    pub seller:        Pubkey,
    pub bump:          u8,
    pub escrow_id:     u64,
    pub buyer:         Pubkey,
    pub amount:        u64,
    pub deposited:     bool,
    pub state:         String,
    pub deadline:      i64,
    pub updated_ts:    i64,
}
