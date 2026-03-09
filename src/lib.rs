// 1️⃣ Importaciones necesarias de Anchor
use anchor_lang::prelude::*;

// 2️⃣ Declaración del ID de tu programa
// Cada programa en Solana tiene su propia dirección única.
declare_id!("7tfUcKdkY7BVe6wqV4rASb2GZ6YRehJ64xpZac7Sxrd9");

// 3️⃣ Definición del módulo principal del programa
#[program]
pub mod tienda_celulares {
    use super::*;

    // 🔹 Función para registrar un celular en la tienda
    pub fn registrar_celular(
        ctx: Context<RegistrarCelular>, // Contexto con todas las cuentas necesarias
        nombre: String,                 // Nombre del celular
        precio: u64,                    // Precio del celular
    ) -> Result<()> {

        // Validaciones:
        // Nombre no vacío y menor o igual a 32 caracteres
        require!(nombre.len() > 0 && nombre.len() <= 32, TiendaError::NombreInvalido);
        // Precio debe ser mayor a 0
        require!(precio > 0, TiendaError::PrecioInvalido);

        // Accedemos a la cuenta del celular y modificamos sus datos
        let celular = &mut ctx.accounts.celular;
        celular.dueno = *ctx.accounts.dueno.key; // La wallet dueña del celular
        celular.nombre = nombre;                 // Nombre del celular
        celular.precio = precio;                 // Precio del celular
        celular.disponible = true;               // Inicialmente disponible
        celular.bump = ctx.bumps.celular;        // Bump para derivadas de cuentas seguras

        msg!("Celular registrado correctamente"); // Mensaje en los logs
        Ok(())
    }

    // 🔹 Función para actualizar precio o disponibilidad
    pub fn actualizar_celular(
        ctx: Context<ActualizarCelular>,
        precio: u64,
        disponible: bool,
    ) -> Result<()> {
        // Validación del precio
        require!(precio > 0, TiendaError::PrecioInvalido);

        let celular = &mut ctx.accounts.celular;
        celular.precio = precio;         // Actualizamos el precio
        celular.disponible = disponible; // Actualizamos si está disponible o no

        msg!("Celular actualizado correctamente"); // Log de la acción
        Ok(())
    }

    // 🔹 Función para eliminar un celular de la tienda
    pub fn eliminar_celular(ctx: Context<EliminarCelular>) -> Result<()> {
        msg!("Celular eliminado de la tienda"); // Solo mensaje de registro
        Ok(())
    }
}

// 4️⃣ Definición de la estructura de la cuenta "Celular"
#[account]
pub struct Celular {
    pub dueno: Pubkey,     // Wallet del dueño
    pub nombre: String,    // Nombre del celular
    pub precio: u64,       // Precio del celular
    pub disponible: bool,  // Disponible o no
    pub bump: u8,          // Bump para derivadas de cuentas
}

// 5️⃣ Definimos cuánto espacio ocupa la cuenta en la blockchain
impl Celular {
    pub const LEN: usize = 8  // Discriminador de Anchor
        + 32                 // Pubkey del dueño
        + 36                 // String nombre (máx 32 chars + padding)
        + 8                  // Precio u64
        + 1                  // Disponible bool
        + 1;                 // Bump u8
}

// 6️⃣ Contextos de cuentas

// 📌 Contexto para registrar un celular
#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct RegistrarCelular<'info> {
    #[account(
        init,                   // Se crea la cuenta
        payer = dueno,          // El dueño paga la creación
        space = Celular::LEN,   // Espacio en bytes
        seeds = [               // Derivación de la cuenta segura (PDA)
            b"celular",
            dueno.key().as_ref(),
            nombre.as_bytes()
        ],
        bump
    )]
    pub celular: Account<'info, Celular>, // Cuenta del celular

    #[account(mut)]
    pub dueno: Signer<'info>,             // Wallet del dueño

    pub system_program: Program<'info, System>, // Program de sistema de Solana
}

// 📌 Contexto para actualizar un celular
#[derive(Accounts)]
pub struct ActualizarCelular<'info> {
    #[account(
        mut,                                   // Se puede modificar
        has_one = dueno @ TiendaError::NoAutorizado, // Solo el dueño puede
        seeds = [
            b"celular",
            dueno.key().as_ref(),
            celular.nombre.as_bytes()
        ],
        bump = celular.bump
    )]
    pub celular: Account<'info, Celular>,

    pub dueno: Signer<'info>,               // Wallet del dueño
}

// 📌 Contexto para eliminar un celular
#[derive(Accounts)]
pub struct EliminarCelular<'info> {
    #[account(
        mut,                                   // Se puede modificar
        close = dueno,                         // Al eliminar, devuelve SOL al dueño
        has_one = dueno @ TiendaError::NoAutorizado, // Solo el dueño
        seeds = [
            b"celular",
            dueno.key().as_ref(),
            celular.nombre.as_bytes()
        ],
        bump = celular.bump
    )]
    pub celular: Account<'info, Celular>,

    #[account(mut)]
    pub dueno: Signer<'info>,               // Wallet del dueño
}

// 7️⃣ Definición de errores personalizados
#[error_code]
pub enum TiendaError {
    #[msg("No tienes permisos")]
    NoAutorizado,

    #[msg("El nombre del celular es inválido")]
    NombreInvalido,

    #[msg("El precio debe ser mayor a 0")]
    PrecioInvalido,
}
