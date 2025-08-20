use anchor_lang::prelude::*;

declare_id!("Brd06Game0000000000000000000000000000006");

#[program]
pub mod breeding_station {
    use super::*;

    pub fn request_breed(
        ctx: Context<RequestBreed>,
        parent1: u64,
        parent2: u64,
        request_id: u64
    ) -> Result<()> {
        let r = &mut ctx.accounts.request;
        r.id = request_id;
        r.p1 = parent1;
        r.p2 = parent2;
        r.child = None;
        Ok(())
    }

    pub fn finalize_breed(ctx: Context<ModifyBreed>, child_id: u64) -> Result<()> {
        let r = &mut ctx.accounts.request;
        if r.child.is_none() {
            r.child = Some(child_id);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequestBreed<'info> {
    #[account(
        init,
        seeds = [b"breed", &request_id.to_le_bytes()],
        bump,
        payer = user,
        space = 8 + 8 + 8 + 8 + 1
    )]
    pub request: Account<'info, BreedData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyBreed<'info> {
    #[account(mut, seeds = [b"breed", &request.id.to_le_bytes()], bump)]
    pub request: Account<'info, BreedData>,
    pub user: Signer<'info>,
}

#[account]
pub struct BreedData {
    pub id: u64,
    pub p1: u64,
    pub p2: u64,
    pub child: Option<u64>,
}
