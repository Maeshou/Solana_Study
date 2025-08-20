use anchor_lang::prelude::*;

declare_id!("MystBx3030303030303030303030303030303030");

#[program]
pub mod mystery_box {
    use super::*;

    pub fn init_box(ctx: Context<InitBox>, prizes: [u64; 5]) -> Result<()> {
        let b = &mut ctx.accounts.mbox;
        b.prizes = prizes;
        b.next_index = 0;
        b.opens = 0;
        Ok(())
    }

    pub fn open_box(ctx: Context<OpenBox>) -> Result<u64> {
        let b = &mut ctx.accounts.mbox;
        if (b.next_index as usize) < b.prizes.len() {
            let prize = b.prizes[b.next_index as usize];
            b.next_index += 1;
            b.opens = b.opens.saturating_add(1);
            Ok(prize)
        } else {
            // 賞品尽きた場合はデフォルト返却
            b.empty_opens = b.empty_opens.saturating_add(1);
            Ok(0)
        }
    }
}

#[derive(Accounts)]
pub struct InitBox<'info> {
    #[account(init, payer = user, space = 8 + 8*5 + 1 + 8 + 8)]
    pub mbox: Account<'info, MysteryBoxData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenBox<'info> {
    #[account(mut)]
    pub mbox: Account<'info, MysteryBoxData>,
}

#[account]
pub struct MysteryBoxData {
    pub prizes: [u64; 5],
    pub next_index: u8,
    pub opens: u64,
    pub empty_opens: u64,
}
