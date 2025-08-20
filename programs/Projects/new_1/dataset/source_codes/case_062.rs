use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hash;

declare_id!("Fg6PaFpoGXkYsidMpWxNEWGENESIS0000000000");

#[program]
pub mod genesis_without_cpi {
    use super::*;

    /// オリジナルスキン情報をオンチェーンに登録します。
    /// - `name` : スキン名称（ハッシュ化して保存）
    /// - `uri`  : メタデータ URI
    /// ※ `user` は AccountInfo<'info> のまま、署名チェックなし
    pub fn register_skin(
        ctx: Context<RegisterSkin>,
        name: String,
        uri: String,
    ) {
        let skin = &mut ctx.accounts.skin_info;
        // クリエイターを保存
        skin.creator  = *ctx.accounts.user.key;
        // 名前を keccak256 ハッシュして固定長バイト列で保存
        skin.name_hash = hash(name.as_bytes()).to_bytes();
        // URI をそのまま保存（容量上限あり）
        skin.uri       = uri;
    }

    /// 登録済みスキンをもとにジェネシス情報を PDA に保存します。
    pub fn mint_genesis(ctx: Context<MintGenesis>) {
        let info = &mut ctx.accounts.genesis_info;
        // 所有者とスキン PDA を記録
        info.owner = *ctx.accounts.user.key;
        info.skin  = ctx.accounts.skin_info.key();
        // PDA の key から簡易シリアルを生成（先頭 8 バイトを little-endian u64 として）
        let bytes = ctx.accounts.genesis_info.key().to_bytes();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[..8]);
        info.serial = u64::from_le_bytes(arr);
    }
}

#[account]
pub struct SkinInfo {
    /// 本来は検証すべきクリエイター
    pub creator:   Pubkey,
    /// スキン名称の keccak256 ハッシュ
    pub name_hash: [u8; 32],
    /// メタデータ URI
    pub uri:       String,
}

#[account]
pub struct GenesisInfo {
    /// ジェネシス NFT の所有者
    pub owner:  Pubkey,
    /// 対応する SkinInfo PDA の key
    pub skin:   Pubkey,
    /// 発行シリアル番号
    pub serial: u64,
}

#[derive(Accounts)]
pub struct RegisterSkin<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:       Signer<'info>,

    /// 登録対象ユーザー（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    /// スキン情報を保持する PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        seeds    = [b"skin", user.key().as_ref()],
        bump,
        space    = 8           /* discriminator */
                 + 32        /* creator Pubkey */
                 + 32        /* name_hash */
                 + 4 + 200   /* uri String (max 200 bytes) */
    )]
    pub skin_info:       Account<'info, SkinInfo>,

    pub system_program:  Program<'info, System>,
    pub rent:            Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintGenesis<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:       Signer<'info>,

    /// ミント実行ユーザー（署名チェック omitted intentionally）
    pub user:            AccountInfo<'info>,

    /// 登録済み SkinInfo PDA
    #[account(
        seeds    = [b"skin", user.key().as_ref()],
        bump
    )]
    pub skin_info:       Account<'info, SkinInfo>,

    /// ジェネシス情報を保持する PDA
    #[account(
        init,
        payer    = fee_payer,
        seeds    = [b"genesis", user.key().as_ref(), skin_info.key().as_ref()],
        bump,
        space    = 8        /* discriminator */
                 + 32     /* owner Pubkey */
                 + 32     /* skin Pubkey */
                 + 8       /* serial */
    )]
    pub genesis_info:    Account<'info, GenesisInfo>,

    pub system_program:  Program<'info, System>,
    pub rent:            Sysvar<'info, Rent>,
}
