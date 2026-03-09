use anchor_lang::prelude::*;

declare_id!("8Csv9Td8dR8xGYfFWGWeMewsGsMVokgiuLrisEU57LRB");

#[program]
pub mod task_rewards {
    use super::*;

    //Inicializar el perfil del usuario
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;

        user_stats.user = ctx.accounts.authority.key();
        user_stats.tasks_completed = 0;
        user_stats.points = 0;

        msg!("Perfil de usuario creado para: {}", user_stats.user);
        Ok(())
    }

    //Crear una tarea
    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: u64,
        reward_amount: u64,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        task.id = task_id;
        task.reward = reward_amount;
        task.is_completed = false;

        msg!(
            "Tarea {} creada con recompensa de {} lamports",
            task_id,
            reward_amount
        );

        Ok(())
    }

    // Completar tarea y recibir recompensa
    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let user_stats = &mut ctx.accounts.user_stats;

        require!(!task.is_completed, TaskError::AlreadyCompleted);

        let amount = task.reward;

        // Transferir lamports desde la cuenta de la tarea al usuario
        **task.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts
            .authority
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;

        // Actualizar estado
        task.is_completed = true;
        user_stats.tasks_completed += 1;
        user_stats.points += 100;

        msg!(
            "¡Tarea completada! Usuario recibió {} lamports y 100 puntos",
            amount
        );

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 8 + 1,
        seeds = [b"task", authority.key().as_ref(), task_id.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8,
        seeds = [b"user-stats", authority.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,

    #[account(
        mut,
        seeds = [b"user-stats", authority.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Task {
    pub id: u64,
    pub reward: u64,
    pub is_completed: bool,
}

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub tasks_completed: u64,
    pub points: u64,
}

#[error_code]
pub enum TaskError {
    #[msg("Esta tarea ya ha sido completada.")]
    AlreadyCompleted,
}
