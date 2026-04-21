use anchor_lang::prelude::*;

// ID del programa
declare_id!("39eGAzUMjCvLnsQPkbNf9dEGaJ5uTS598pYtBNAL3V5i");

#[program]
pub mod subchain_solana {
    use super::*;

    pub fn inicializar_gestor(ctx: Context<CrearGestor>, nombre_usuario: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        gestor.owner = ctx.accounts.owner.key();
        gestor.nombre_usuario = nombre_usuario;
        gestor.suscripciones = Vec::new();
        
        msg!("Gestor de suscripciones para {} creado.", gestor.nombre_usuario);
        Ok(())
    }

    pub fn agregar_suscripcion(ctx: Context<GestionarSuscripcion>, nombre: String, costo: u32) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nueva_sub = Suscripcion {
            nombre,
            costo,
            activa: true,
        };

        gestor.suscripciones.push(nueva_sub);
        msg!("Suscripción agregada correctamente.");
        Ok(())
    }

    pub fn editar_suscripcion(ctx: Context<GestionarSuscripcion>, nombre: String, nuevo_costo: u32, esta_activa: bool) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut gestor.suscripciones;
        for i in 0..lista.len() {
            if lista[i].nombre == nombre {
                lista[i].costo = nuevo_costo;
                lista[i].activa = esta_activa;
                msg!("Suscripción '{}' actualizada.", nombre);
                return Ok(());
            }
        }
        Err(Errores::SuscripcionNoEncontrada.into())
    }

    pub fn eliminar_suscripcion(ctx: Context<GestionarSuscripcion>, nombre: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut gestor.suscripciones;
        let index = lista.iter().position(|s| s.nombre == nombre);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Suscripción '{}' eliminada.", nombre);
            Ok(())
        } else {
            Err(Errores::SuscripcionNoEncontrada.into())
        }
    }

    pub fn ver_suscripciones(ctx: Context<GestionarSuscripcion>) -> Result<()> {
        msg!("Usuario: {}", ctx.accounts.gestor.nombre_usuario);
        msg!("Lista de Suscripciones: {:#?}", ctx.accounts.gestor.suscripciones);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Suscripcion {
    #[max_len(30)]
    pub nombre: String,
    pub costo: u32,
    pub activa: bool,
}

#[account]
#[derive(InitSpace)]
pub struct GestorSuscripciones {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(10)] 
    pub suscripciones: Vec<Suscripcion>,
}

#[derive(Accounts)]
pub struct CrearGestor<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + GestorSuscripciones::INIT_SPACE,
        seeds = [b"gestor", owner.key().as_ref()],
        bump
    )]
    pub gestor: Account<'info, GestorSuscripciones>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarSuscripcion<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub gestor: Account<'info, GestorSuscripciones>,
}

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para esta acción.")]
    NoEresElOwner,
    #[msg("La suscripción no existe.")]
    SuscripcionNoEncontrada,
}
