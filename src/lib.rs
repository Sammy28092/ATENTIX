use anchor_lang::prelude::*;

// Nota: Cuando compiles, Anchor cambiará este ID automáticamente.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); 

#[program]
pub mod adhd_learning_platform {
    use super::*;

    // Función 1: Crea el perfil del jugador en la blockchain
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.total_score = 0;
        user_profile.games_played = 0;
        Ok(())
    }

    // Función 2: Suma puntos cuando el usuario termina una actividad de atención
    pub fn add_score(ctx: Context<AddScore>, points: u32) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.total_score += points;
        user_profile.games_played += 1;
        Ok(())
    }
}

// Estructuras que definen qué datos se guardan y quién paga por el espacio
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + 4)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddScore<'info> {
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

// La "plantilla" de los datos del usuario
#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub total_score: u32,
    pub games_played: u32,
}
