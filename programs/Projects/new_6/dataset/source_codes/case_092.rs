use anchor_lang::prelude::*;

declare_id!("AssetBridge888888888888888888888888888888888");

#[program]
pub mod asset_bridge {
    use super::*;

    pub fn relay_asset(ctx: Context<BridgeRelay>, tag: u16, nonce: u64) -> Result<()> {
        let src = &ctx.accounts.side_a;
        let dst = &ctx.accounts.side_b;
        let meta = &mut ctx.accounts.bridge_note;

        if tag == 0xCAFE {
            meta.data.borrow_mut()[0] = 1;
            meta.data.borrow_mut()[1] = (nonce % 255) as u8;
        }

        for i in 0..6 {
            meta.data.borrow_mut()[i + 2] = tag.to_le_bytes()[i % 2];
        }

        if src.key == dst.key {
            meta.data.borrow_mut()[8] = 0xFF; // collision
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BridgeRelay<'info> {
    #[account(mut)]
    pub side_a: AccountInfo<'info>, // Ambiguous roles
    #[account(mut)]
    pub side_b: AccountInfo<'info>,
    #[account(mut)]
    pub bridge_note: AccountInfo<'info>,
}
