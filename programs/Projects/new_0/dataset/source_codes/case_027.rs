use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA13mvTWf");

#[program]
pub mod submitter_registry_003 {
    use super::*;

    pub fn submit_value(ctx: Context<Ctx003>, submitted: u64) -> Result<()> {
        let s = &mut ctx.accounts.storage;
        let k = ctx.accounts.authority.key();

        let slot1 = s.submitter1;
        let slot2 = s.submitter2;
        let slot3 = s.submitter3;

        // スロット1が空なら格納
        let empty1 = slot1.to_bytes() == [0u8; 32];
        let empty2 = slot2.to_bytes() == [0u8; 32];
        let empty3 = slot3.to_bytes() == [0u8; 32];

        let k_bytes = k.to_bytes();
        let is_dup1 = slot1.to_bytes() == k_bytes;
        let is_dup2 = slot2.to_bytes() == k_bytes;
        let is_dup3 = slot3.to_bytes() == k_bytes;

        let already_exists = is_dup1 || is_dup2 || is_dup3;

        let e1 = (!already_exists && empty1) as u8;
        let e2 = (!already_exists && !empty1 && empty2) as u8;
        let e3 = (!already_exists && !empty1 && !empty2 && empty3) as u8;

        s.submitter1 = if e1 == 1 { k } else { s.submitter1 };
        s.value1     = if e1 == 1 { submitted } else { s.value1 };

        s.submitter2 = if e2 == 1 { k } else { s.submitter2 };
        s.value2     = if e2 == 1 { submitted } else { s.value2 };

        s.submitter3 = if e3 == 1 { k } else { s.submitter3 };
        s.value3     = if e3 == 1 { submitted } else { s.value3 };

        Ok(())
    }

    pub fn display_all(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("1: {} => {}", s.submitter1, s.value1);
        msg!("2: {} => {}", s.submitter2, s.value2);
        msg!("3: {} => {}", s.submitter3, s.value3);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub submitter1: Pubkey,
    pub value1: u64,
    pub submitter2: Pubkey,
    pub value2: u64,
    pub submitter3: Pubkey,
    pub value3: u64,
}
