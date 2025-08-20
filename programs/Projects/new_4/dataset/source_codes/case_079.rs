use anchor_lang::prelude::*;

declare_id!("NextCaseEx40404040404040404040404040404040");

#[program]
pub mod example4 {
    use super::*;

    // 社員を登録（employee にだけ init）
    pub fn onboard_employee(ctx: Context<Onboard>, name: String) -> Result<()> {
        let emp = &mut ctx.accounts.employee;          // ← initあり
        emp.name = name;

        let payroll = &mut ctx.accounts.payroll;       // ← initなし（本来は初期化すべき）
        // 名前に応じて給与ランク設定
        if emp.name.len() > 5 {
            payroll.grade = 2;
        } else {
            payroll.grade = 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Onboard<'info> {
    #[account(init, payer = hr, space = 8 + 32)]
    pub employee: Account<'info, EmployeeData>,
    pub payroll: Account<'info, PayrollData>,
    #[account(mut)] pub hr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EmployeeData {
    pub name: String,
}

#[account]
pub struct PayrollData {
    pub grade: u8,
}
