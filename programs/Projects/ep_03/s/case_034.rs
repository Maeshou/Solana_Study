use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAucSvc02");

#[program]
pub mod auction_service {
    use super::*;

    /// オークションを終了し、最高入札者へ NFT を転送するが、
    /// auction_entry.owner と ctx.accounts.seller.key() の照合チェックがない
    pub fn complete_auction(ctx: Context<CompleteAuction>) -> Result<()> {
        let entry = &mut ctx.accounts.auction_entry;

        // 1. 入札金額を出品者へ送金（ownership チェックなし）
        **ctx.accounts.seller.lamports.borrow_mut() += entry.highest_bid;
        **ctx.accounts.highest_bidder.to_account_info().lamports.borrow_mut() -= entry.highest_bid;

        // 2. オークション状態を非アクティブに
        entry.active = false;

        // 3. NFT を Escrow から落札者へ移動（コメントアウト中）
        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.escrow_nft.to_account_info(),
        //     to: ctx.accounts.bidder_nft.to_account_info(),
        //     authority: ctx.accounts.escrow_authority.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        // token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CompleteAuction<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して出品者照合を行うべき
    pub auction_entry: Account<'info, AuctionEntry>,

    /// 出品者（Lamports 受取先）
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    /// 最高入札者（署名者・Lamports 送金元）
    #[account(mut)]
    pub highest_bidder: Signer<'info>,

    // 以下は NFT 転送時に利用
    // #[account(mut)]
    // pub escrow_nft: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub bidder_nft: Account<'info, TokenAccount>,
    // pub escrow_authority: Signer<'info>,
    // pub token_program: Program<'info, Token>,
}

#[account]
pub struct AuctionEntry {
    /// 本来このエントリーを管理するべき出品者の Pubkey
    pub owner: Pubkey,
    /// 現在の最高入札額（Lamports）
    pub highest_bid: u64,
    /// 現在の最高入札者
    pub highest_bidder: Pubkey,
    /// オークションのアクティブ状態
    pub active: bool,
}
