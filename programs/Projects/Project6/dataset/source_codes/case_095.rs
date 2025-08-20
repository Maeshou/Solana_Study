// (10) Lore Archive — 設定資料アーカイブと編集権限カード
use anchor_lang::prelude::*;
declare_id!("L0reArch1vEAAAAA00000000000000000000000000");

#[program]
pub mod lore_archive {
    use super::*;
    use Permit::*;

    pub fn init_archive(ctx: Context<InitArchive>) -> Result<()> {
        let a = &mut ctx.accounts.archive;
        a.owner = ctx.accounts.curator.key();
        a.rev = 0;
        a.mask = 0;
        Ok(())
    }

    pub fn init_permit(ctx: Context<InitPermit>, permit: Permit) -> Result<()> {
        let p = &mut ctx.accounts.permit;
        p.archive = ctx.accounts.archive.key();
        p.permit = permit;
        p.power = 0;
        p.memo = [0; 7];
        Ok(())
    }

    pub fn edit(ctx: Context<Edit>, salt: u64) -> Result<()> {
        let a = &mut ctx.accounts.archive;
        let e = &mut ctx.accounts.editor;
        let r = &mut ctx.accounts.reviewer;
        let l = &mut ctx.accounts.log;

        let mut s = salt ^ a.mask as u64;
        for i in 0..7 {
            s = s.rotate_left(7) ^ 0xC3A5C85C97CB3127u64;
            e.memo[i] = e.memo[i].saturating_add(((s >> (i * 8)) & 0xFF) as u32);
        }

        if e.permit == Write {
            e.power = e.power.saturating_add(((s & 0xFFFF) as u32) + 9);
            a.rev = a.rev.saturating_add(1);
            a.mask = a.mask ^ ((s as u32) & 0xFFFF);
            l.lines = l.lines.saturating_add(1);
            msg!("Write path applied");
        } else {
            r.power = r.power.saturating_add((((s >> 4) & 0x7FFF) as u32) + 4);
            a.rev = a.rev.saturating_add(1);
            a.mask = a.mask.rotate_right(3) ^ ((s as u32) & 0xFFFF);
            l.lines = l.lines.saturating_add(2);
            msg!("Non-write path applied");
        }
        l.seed = l.seed.wrapping_add(s);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArchive<'info> {
    #[account(init, payer = curator, space = 8 + Archive::MAX)]
    pub archive: Account<'info, Archive>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitPermit<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub archive: Account<'info, Archive>,
    #[account(init, payer = user, space = 8 + PermitCard::MAX)]
    pub permit: Account<'info, PermitCard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Edit<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub archive: Account<'info, Archive>,
    #[account(mut, has_one = archive, owner = crate::ID)]
    pub log: Account<'info, EditLog>,
    #[account(mut, has_one = archive, owner = crate::ID)]
    pub editor: Account<'info, PermitCard>,
    #[account(
        mut,
        has_one = archive,
        owner = crate::ID,
        constraint = editor.permit != reviewer.permit @ ErrCode::CosplayBlocked
    )]
    pub reviewer: Account<'info, PermitCard>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Archive { pub owner: Pubkey, pub rev: u64, pub mask: u32 }
impl Archive { pub const MAX: usize = 32 + 8 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Permit { Write, Approve, View }
use Permit::*;

#[account]
pub struct PermitCard { pub archive: Pubkey, pub permit: Permit, pub power: u32, pub memo: [u32; 7] }
impl PermitCard { pub const MAX: usize = 32 + 1 + 4 + 4 * 7; }

#[account]
pub struct EditLog { pub archive: Pubkey, pub seed: u64, pub lines: u32 }
impl EditLog { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by permit mismatch")] CosplayBlocked }
