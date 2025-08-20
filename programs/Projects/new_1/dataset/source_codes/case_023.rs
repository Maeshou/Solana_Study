use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxGENESIS_NO_SIGNER000000");

#[program]
pub mod genesis_factory {
    use super::*;

    /// オリジナルスキン情報をオンチェーンに登録
    /// ──────────────────────────────────────────────
    /// name       : 入力された名称を大文字化して保存  
    /// symbol     : 入力されたシンボルを逆順にして保存  
    /// uri        : タイムスタンプを付与して保存  
    /// description: クォートで囲んで保存  
    pub fn register_skin(
        ctx: Context<RegisterSkin>,
        name: String,
        symbol: String,
        uri: String,
        description: String,
    ) {
        let ts = ctx.accounts.clock.unix_timestamp;

        // 文字列操作
        let up_name     = name.to_uppercase();
        let rev_symbol  = symbol.chars().rev().collect::<String>();
        let uri_ts      = format!("{}?ts={}", uri, ts);
        let desc_quoted = format!("\"{}\"", description);

        // PDA に書き込み
        let skin = &mut ctx.accounts.skin_pda;
        skin.owner       = *ctx.accounts.user.key;
        skin.created_at  = ts;
        skin.name        = up_name;
        skin.symbol      = rev_symbol;
        skin.uri         = uri_ts;
        skin.description = desc_quoted;
    }

    /// 登録済みスキンをもとにジェネシス情報をPDAに保存
    /// ──────────────────────────────────────────────
    /// 別途オフチェーンでNFT発行する想定の簡易版
    pub fn mint_genesis(ctx: Context<MintGenesis>) {
        let ts    = ctx.accounts.clock.unix_timestamp;
        let skin  = ctx.accounts.skin_pda.key();
        let bytes = ctx.accounts.genesis_pda.key().to_bytes();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[..8]);
        let serial = u64::from_le_bytes(arr);

        let gen = &mut ctx.accounts.genesis_pda;
        gen.owner     = *ctx.accounts.user.key;
        gen.skin      = skin;
        gen.issued_at = ts;
        gen.serial    = serial;
    }
}

#[account]
pub struct SkinPda {
    pub owner:       Pubkey,
    pub created_at:  i64,
    pub name:        String,
    pub symbol:      String,
    pub uri:         String,
    pub description: String,
}

#[derive(Accounts)]
pub struct RegisterSkin<'info> {
    /// この tx の手数料を支払うアカウント（署名必須）
    #[account(mut)]
    pub fee_payer: ProgramAccount<'info, Signer>,

    /// 脆弱性として署名チェックを省略する対象アカウント
    /// (本来なら Signer<'info> とすべきところを AccountInfo<'info> のまま)
    pub user: AccountInfo<'info>,

    /// スキン情報を保持する PDA
    #[account(
        init_if_needed,
        payer = fee_payer,
        space  = 8 + 32 + 8 + 4*4 + 512,
        seeds  = [b"skin", user.key().as_ref()],
        bump
    )]
    pub skin_pda: Account<'info, SkinPda>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
    pub clock:          Sysvar<'info, Clock>,
}

#[account]
pub struct GenesisPda {
    pub owner:     Pubkey,
    pub skin:      Pubkey,
    pub issued_at: i64,
    pub serial:    u64,
}

#[derive(Accounts)]
pub struct MintGenesis<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer: ProgramAccount<'info, Signer>,

    /// 登録済み SkinPda (owner チェックのみ)
    #[account(
        seeds     = [b"skin", user.key().as_ref()],
        bump,
        has_one   = owner @ ErrorCode::Unauthorized
    )]
    pub skin_pda: Account<'info, SkinPda>,

    /// owner 検証用 (PDA.owner と照合するのみ)
    pub owner: AccountInfo<'info>,

    /// ジェネシス情報を保持する PDA
    #[account(
        init,
        payer   = fee_payer,
        space   = 8 + 32 + 32 + 8 + 8,
        seeds   = [b"genesis", user.key().as_ref(), skin_pda.key().as_ref()],
        bump
    )]
    pub genesis_pda: Account<'info, GenesisPda>,

    /// 署名チェック omitted intentionally
    pub user:    AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
    pub clock:          Sysvar<'info, Clock>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("SkinPda.owner mismatch")]
    Unauthorized,
}
