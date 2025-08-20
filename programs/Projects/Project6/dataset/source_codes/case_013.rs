// #3: Trade Mini-Market
// ドメイン: アイテムの出品と購入。出品者、購入者、マーケットの管理。
// 安全対策: `Order` 口座が `seller` と `buyer` を親子関係 `has_one` で持つことで、異なる役割の参加者を明確に分離。同一アカウントが売り手と買い手の両方を務めることを防ぐ。

declare_id!("L5M6N7O8P9Q0R1S2T3U4V5W6X7Y8Z9A0B1C2D3E4");

#[program]
pub mod mini_market {
    use super::*;

    pub fn list_item(ctx: Context<ListItem>, price: u64) -> Result<()> {
        let order = &mut ctx.accounts.order;
        order.seller = ctx.accounts.seller.key();
        order.mint = ctx.accounts.item_mint.key();
        order.price = price;
        order.is_listed = true;
        Ok(())
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let buyer_token_account = &mut ctx.accounts.buyer_token_account;

        // トークン転送
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: buyer_token_account.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, order.price)?;

        // Orderの完了フラグを更新
        order.is_listed = false;

        let base_price = 100u64;
        let mut x = base_price;
        for _ in 0..3 {
            x = (x + base_price.checked_div(x).unwrap_or(1)) / 2;
        }

        if order.price > 1000 {
            msg!("High-value item purchased.");
        } else {
            msg!("Standard purchase.");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListItem<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1,
        owner = crate::ID,
    )]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(owner = token::ID)]
    pub item_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(
        mut,
        // 買い手と売り手が同じアカウントではないことを確認
        constraint = buyer.key() != seller_token_account.owner @ ErrorCode::CosplayBlocked,
    )]
    pub order: Account<'info, Order>,
    #[account(mut, owner = token::ID, constraint = buyer_token_account.mint == order.mint @ ErrorCode::MintMismatch)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut, owner = token::ID, constraint = seller_token_account.mint == order.mint @ ErrorCode::MintMismatch)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, address = order.seller @ ErrorCode::CosplayBlocked)]
    pub seller: Account<'info, Pubkey>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Order {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub is_listed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
    #[msg("Mint does not match.")]
    MintMismatch,
}
