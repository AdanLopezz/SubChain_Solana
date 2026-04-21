💳 SubChain Solana - Gestor de Suscripciones

SubChain Solana es un contrato inteligente (Smart Contract) desarrollado con el framework Anchor sobre la red de Solana.

Este proyecto implementa un sistema CRUD para gestionar suscripciones de pago (SaaS, Streaming, Cloud), demostrando dominio técnico en:

🔑 Program Derived Addresses (PDAs)
⚡ Optimización de memoria On-Chain
🔒 Seguridad basada en firmas
🧠 1. Arquitectura y Estructuras de Datos (State)

En Solana, el almacenamiento requiere definir explícitamente el tamaño de los datos para calcular correctamente la rent.

📦 Contenedor Principal (PDA): GestorSuscripciones

Cuenta raíz derivada criptográficamente, inicializada una única vez por usuario.

#[account]
#[derive(InitSpace)]
pub struct GestorSuscripciones {
    pub owner: Pubkey,                // 32 bytes: Llave pública del creador.
    #[max_len(40)]                    
    pub nombre_usuario: String,       // 44 bytes (4 bytes prefijo + 40 datos).
    #[max_len(10)]                    
    pub suscripciones: Vec<Suscripcion>, 
}
🧩 Objeto Interno: Suscripcion

Define cada elemento dentro del vector:

nombre (String): Máximo 30 caracteres
costo (u32): Optimizado para memoria (sin decimales)
activa (bool): 1 byte para estado de pago
🔒 2. Seguridad y Contextos de Ejecución

Anchor utiliza contextos para separar validación y lógica de negocio.

🏗️ Contextos principales
🆕 CrearGestor
Inicializa la cuenta en memoria
Usa:
space = 8 + GestorSuscripciones::INIT_SPACE

Incluye 8 bytes del discriminador de Anchor

🔄 GestionarSuscripcion
Maneja operaciones Read / Update / Delete
Requiere:
Signer
Cuenta gestor como #[account(mut)]
🛡️ Medida de Seguridad Principal
require!(
    gestor.owner == ctx.accounts.owner.key(),
    Errores::NoEresElOwner
);

✔ Garantiza que solo el propietario puede modificar las suscripciones, aunque la cuenta sea pública en la blockchain.

⚙️ 3. Lógica de las Funciones (CRUD)
🟢 Create — Inicialización
inicializar_gestor
Deriva la PDA con:
[b"gestor", owner.key()]
Inicializa:
Owner
Nombre de usuario
Vector vacío: Vec::new()
➕ Create — Inserción
agregar_suscripcion
Recibe:
nombre
costo
Asigna automáticamente:
activa: true
Inserta con:
.push()
✏️ Update — Modificación
editar_suscripcion
Busca mediante for
Compara por nombre
Actualiza:
costo
activa

✔ Permite simular pausas en pagos

❌ Delete — Eliminación
eliminar_suscripcion
Usa:
.iter().position(|s| s.nombre == nombre)
Si existe:
.remove(index)
Si no:
❗ Error: SuscripcionNoEncontrada
📖 Read — Lectura
ver_suscripciones
Usa:
msg!("{:#?}", gestor.suscripciones);

✔ Genera logs On-Chain legibles
