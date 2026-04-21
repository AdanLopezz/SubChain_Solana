SubChain Solana - Gestor de Suscripciones (Desglose Técnico) 💳
SubChain Solana es un contrato inteligente (Smart Contract) desarrollado con el framework Anchor sobre la red de Solana. Este proyecto implementa un sistema CRUD para gestionar suscripciones de pago (SaaS, Streaming, Cloud), demostrando el dominio técnico sobre Program Derived Addresses (PDAs), optimización de memoria On-Chain y seguridad basada en firmas.

🧠 1. Arquitectura y Estructuras de Datos (State)
El almacenamiento en Solana requiere definir explícitamente el tamaño máximo de nuestros datos para pagar la "renta" (rent) adecuada.

El Contenedor Principal (PDA): GestorSuscripciones
Es la cuenta raíz derivada criptográficamente. Se inicializa una única vez por usuario.

Rust
#[account]
#[derive(InitSpace)]
pub struct GestorSuscripciones {
    pub owner: Pubkey,                // 32 bytes: Llave pública del creador.
    #[max_len(40)]                    // Límite estricto para precalcular espacio.
    pub nombre_usuario: String,       // 44 bytes (4 bytes de prefijo + 40 de datos).
    #[max_len(10)]                    // Capacidad máxima de 10 suscripciones.
    pub suscripciones: Vec<Suscripcion>, 
}
El Objeto Interno: Suscripcion
Define la estructura atómica de cada elemento dentro del vector.

nombre (String): Limitado a 30 caracteres.

costo (u32): Se utiliza un entero de 32 bits en lugar de 64 bits para optimizar la memoria, ideal para guardar valores en dólares o moneda local sin decimales.

activa (bool): Ocupa solo 1 byte en memoria para determinar si la suscripción debe pagarse este mes o está pausada.

🔒 2. Seguridad y Contextos de Ejecución
Anchor utiliza "Contextos" para aislar la lógica de validación de cuentas de la lógica de negocio. Este programa utiliza dos contextos principales:

CrearGestor: Se encarga del despliegue en memoria. Utiliza el parámetro space = 8 + GestorSuscripciones::INIT_SPACE para que Anchor calcule matemáticamente el tamaño en bytes que requiere el vector y los strings, sumando los 8 bytes obligatorios del discriminador de cuenta de Anchor.

GestionarSuscripcion: Se utiliza para las operaciones R-U-D (Read, Update, Delete). Solo requiere dos cuentas: el Signer (quien paga la transacción) y la cuenta gestor marcada como #[account(mut)] para permitir su modificación.

Medida de Seguridad Principal:
Todas las funciones que modifican el vector interno implementan la siguiente macro de validación antes de ejecutar la lógica:

Rust
require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);
Esto garantiza que, aunque la cuenta (PDA) sea pública en la blockchain, nadie más que el creador original puede añadir, editar o borrar suscripciones.

⚙️ 3. Lógica de las Funciones (El CRUD Explicado)
Create (Inicialización): inicializar_gestor
Se deriva la PDA usando la semilla [b"gestor", owner.key()]. Asigna el owner, el nombre del usuario e inicializa el vector de suscripciones completamente vacío (Vec::new()).

Create (Inserción): agregar_suscripcion
Recibe el nombre y el costo como parámetros. Automáticamente asigna el booleano activa: true. Instancia el struct Suscripcion y lo empuja al final del vector usando el método nativo de Rust .push().

Update (Modificación): editar_suscripcion
Realiza una búsqueda iterativa mediante un bucle for. Compara el nombre recibido con los nombres existentes en el vector. Al encontrar la coincidencia, sobrescribe directamente el costo y el estado booleano (activa), permitiendo al usuario simular "pausas" en sus pagos.

Delete (Eliminación): eliminar_suscripcion
Utiliza iteradores funcionales y cierres (closures) de Rust: .iter().position(|s| s.nombre == nombre). Si la búsqueda arroja un Some(index), utiliza el método .remove(index) para sacar el elemento del vector, lo que reorganiza la memoria interna automáticamente. Si no lo encuentra, devuelve un error personalizado SuscripcionNoEncontrada.

Read (Lectura): ver_suscripciones
Utiliza la macro msg! con el formateador de depuración {:#?} para emitir un log On-Chain legible con todo el contenido del vector.

🧪 4. Despliegue y Pruebas
Para compilar y testear este código en Solana Playground:

Copia el código fuente en lib.rs.

Ejecuta cargo clean en la terminal (opcional, para evitar solapamientos de IDL).

Haz clic en Build para generar el binario y el IDL.

Haz clic en Deploy para publicar el programa en la Devnet.

Utiliza la pestaña Test o escribe un script en TypeScript invocando pg.program.methods... para interactuar con el CRUD enviando los parámetros de nombre (String), costo (u32) y estado (bool).
