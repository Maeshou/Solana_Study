use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, mint_to};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::instruction::{
    create_metadata_accounts_v3, create_master_edition_v3,
};
use mpl_token_metadata::state::{DataV2};

declare_id!("Fg6PaFpoGXkYsidMpWxSKINANDGENESIS9X3FT7Y8Z0HA1BC");

#[program]
pub mod genesis_nft_factory {
    use super::*;

    /// 1) オリジナルスキンのメタデータを登録
    pub fn register_skin(
        ctx: Context<RegisterSkin>,
        name: String,
        symbol: String,
        uri: String,
        description: String,
    ) -> Result<()> {
        let skin = &mut ctx.accounts.skin_account;
        skin.owner = *ctx.accounts.user.key;
        skin.name = name;
        skin.symbol = symbol;
        skin.uri = uri;
        skin.description = description;
        Ok(())
    }

    /// 2) 登録済みスキンを元にジェネシスNFTを新規ミント
    pub fn mint_genesis(ctx: Context<MintGenesis>) -> Result<()> {
        let skin = &ctx.accounts.skin_account;

        // ① アートワークを参照するMetadataData
        let metadata_data = DataV2 {
            name: skin.name.clone(),
            symbol: skin.symbol.clone(),
            uri: skin.uri.clone(),
            seller_fee_basis_points: 500,        // ロイヤリティ 5%
            creators: Some(vec![
                mpl_token_metadata::state::Creator {
                    address: skin.owner,
                    verified: false,
                    share: 100,
                }
            ]),
            collection: None,
            uses: None,
        };

        // ② Metadata アカウント作成
        let metadata_ix = create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.genesis_mint.key(),
            ctx.accounts.user.key(),
            ctx.accounts.user.key(),
            ctx.accounts.user.key(),
            metadata_data,
            true,  // is_mutable
            true,  // update_authority_is_signer
            None,
            None,
            None,
        );
        invoke(
            &metadata_ix,
            &[
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.genesis_mint.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        // ③ マスターエディション（ジェネシス）アカウント作成
        let edition_ix = create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition_account.key(),
            ctx.accounts.genesis_mint.key(),
            ctx.accounts.user.key(),
            ctx.accounts.user.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.user.key(),
            Some(0),
        );
        invoke(
            &edition_ix,
            &[
                ctx.accounts.master_edition_account.to_account_info(),
                ctx.accounts.genesis_mint.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        // ④ 実際のトークンをユーザーのATAにミント
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            mint_to::Accounts {
                mint: ctx.accounts.genesis_mint.to_account_info(),
                to:   ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        mint_to(cpi_ctx, 1)?;

        Ok(())
    }
}

// ————— Accounts —————

/// スキン情報を保存するPDA
#[account(
    init,
    payer = user,
    space = 8 + 32 + 4*4 + 200,  // owner + 各String容量
    seeds = [b"skin", user.key().as_ref()],
    bump,
)]
pub struct SkinAccount {
    pub owner: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub description: String,
}

#[derive(Accounts)]
pub struct RegisterSkin<'info> {
    #[account(mut)]
    pub user:           Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 4*4 + 200,
        seeds = [b"skin", user.key().as_ref()],
        bump,
    )]
    pub skin_account:   Account<'info, SkinAccount>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintGenesis<'info> {
    #[account(mut)]
    pub user:                   Signer<'info>,

    /// 登録済みスキン情報
    #[account(
        seeds = [b"skin", user.key().as_ref()],
        bump,
        has_one = owner @ ErrorCode::Unauthorized,
    )]
    pub skin_account:           Account<'info, SkinAccount>,

    /// 新規Mintアカウント (ジェネシス用)
    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = user,
        mint::freeze_authority = user,
    )]
    pub genesis_mint:           Account<'info, Mint>,

    /// ユーザーの Associated Token Account
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = genesis_mint,
        associated_token::authority = user,
    )]
    pub user_ata:               Account<'info, TokenAccount>,

    /// Metaplex Token Metadata
    /// CHECK: CPI先アドレス
    pub token_metadata_program: UncheckedAccount<'info>,
    /// Metadata PDA
    #[account(mut)]
    pub metadata_account:       UncheckedAccount<'info>,
    /// Master Edition PDA
    #[account(mut)]
    pub master_edition_account: UncheckedAccount<'info>,

    pub token_program:          Program<'info, Token>,
    pub system_program:         Program<'info, System>,
    pub rent:                   Sysvar<'info, Rent>,
}

/// 独自エラー
#[error_code]
pub enum ErrorCode {
    #[msg("SkinAccount の所有者ではありません")]
    Unauthorized,
}
