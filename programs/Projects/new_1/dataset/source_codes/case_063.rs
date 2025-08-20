use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hash;

declare_id!("Fg6PaFpoGXkYsidMpWxPATTERNX0000000000000");

#[program]
pub mod skin_registry {
    use super::*;

    /// 新しいスキンテンプレートを登録します。
    /// - `name`         : テンプレート名称  
    /// - `metadata_url` : JSON メタデータのURL  
    /// - `image_hash`   : 画像データのkeccak256ハッシュ（固定32バイト）
    /// すべてのアカウントは AccountInfo のまま、署名チェックなし。
    pub fn register_template(
        ctx: Context<RegisterTemplate>,
        name: String,
        metadata_url: String,
        image_hash: [u8; 32],
    ) {
        let tmpl = &mut ctx.accounts.template;
        tmpl.author       = *ctx.accounts.user.key;
        // テンプレートIDは name|author のハッシュ上位8バイト
        let mut hasher = hash(&[name.as_bytes(), &ctx.accounts.user.key.to_bytes()].concat()).to_bytes();
        let mut id_arr = [0u8; 8];
        id_arr.copy_from_slice(&hasher[..8]);
        tmpl.template_id = u64::from_le_bytes(id_arr);
        tmpl.name        = name;
        tmpl.metadata    = metadata_url;
        tmpl.image_hash  = image_hash;
    }

    /// 登録済テンプレートからジェネシスを作成します。
    /// - `template_id` : 対象テンプレートのID
    pub fn mint_from_template(ctx: Context<MintFromTemplate>, template_id: u64) {
        let inst = &mut ctx.accounts.instance;
        inst.owner       = *ctx.accounts.user.key;
        inst.template_id = template_id;
        // インスタンスシリアルは PDAキーの先頭8バイト
        let bytes = ctx.accounts.instance.key().to_bytes();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[..8]);
        inst.serial = u64::from_le_bytes(arr);
    }
}

#[account]
pub struct Template {
    pub author:      Pubkey,
    pub template_id: u64,
    pub name:        String,
    pub metadata:    String,
    pub image_hash:  [u8; 32],
}

#[account]
pub struct Instance {
    pub owner:       Pubkey,
    pub template_id: u64,
    pub serial:      u64,
}

#[derive(Accounts)]
pub struct RegisterTemplate<'info> {
    /// テンプレート登録者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// テンプレート情報を保持する PDA
    #[account(
        init,
        payer    = payer,
        space    = 8 + 32 + 8 + (4+100) + (4+200) + 32,
        seeds    = [b"template", &hash(user.key.as_ref()).to_bytes()[..8]],
        bump
    )]
    pub template:     Account<'info, Template>,

    #[account(mut)]
    pub payer:        Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent:         Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintFromTemplate<'info> {
    /// ミント実行者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 参照するテンプレートPDA
    #[account(
        seeds = [b"template", &hash(user.key.as_ref()).to_bytes()[..8]],
        bump
    )]
    pub template:     Account<'info, Template>,

    /// インスタンス情報を保持する PDA
    #[account(
        init,
        payer    = payer,
        space    = 8 + 32 + 8 + 8,
        seeds    = [b"instance", user.key().as_ref(), &template.template_id.to_le_bytes()],
        bump
    )]
    pub instance:     Account<'info, Instance>,

    #[account(mut)]
    pub payer:        Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent:         Sysvar<'info, Rent>,
}
