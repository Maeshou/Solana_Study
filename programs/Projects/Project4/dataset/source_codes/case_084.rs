use anchor_lang::prelude::*;

declare_id!("SafeEx30MemMonitor11111111111111111111111111");

#[program]
pub mod example30 {
    use super::*;

    pub fn init_memory(
        ctx: Context<InitMemory>,
        total: u32,
    ) -> Result<()> {
        let m = &mut ctx.accounts.memory;
        m.total_mem       = total;
        m.used_mem        = 0;
        m.mem_ok_flag     = false;

        // 初期使用率計算
        let rate = if total > 0 { m.used_mem * 100 / total } else { 0 };
        if rate < 80 {
            m.mem_ok_flag = true;
        }
        Ok(())
    }

    pub fn allocate_mem(
        ctx: Context<AllocateMem>,
        amount: u32,
    ) -> Result<()> {
        let m = &mut ctx.accounts.memory;
        m.used_mem = m.used_mem.saturating_add(amount.min(m.total_mem - m.used_mem));

        let rate = if m.total_mem > 0 { m.used_mem * 100 / m.total_mem } else { 0 };
        if rate >= 90 {
            m.mem_ok_flag = false;
        } else {
            m.mem_ok_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMemory<'info> {
    #[account(init, payer = user, space = 8 + 4*2 + 1)]
    pub memory: Account<'info, MemoryMonitorData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AllocateMem<'info> {
    #[account(mut)] pub memory: Account<'info, MemoryMonitorData>,
}

#[account]
pub struct MemoryMonitorData {
    pub total_mem:   u32,
    pub used_mem:    u32,
    pub mem_ok_flag: bool,
}
