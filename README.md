# 💳 SubChain Solana


Sistema CRUD de suscripciones desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto simula la gestión de pagos recurrentes (*SaaS, Streaming, Cloud*) directamente en blockchain, aplicando buenas prácticas de:

- 🔑 Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad basada en firmas  

---

## 📚 Descripción

**SubChain Solana** implementa un gestor de suscripciones descentralizado donde cada usuario puede:

- Crear su propio gestor de suscripciones  
- Agregar servicios (Netflix, AWS, Spotify, etc.)  
- Editar costos o estados (activo/pausado)  
- Eliminar suscripciones  
- Consultar su información en la blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir explícitamente el tamaño de los datos para calcular correctamente la renta (*rent*).

### 📦 PDA Principal: `GestorSuscripciones`

Cuenta raíz derivada criptográficamente, única por usuario.

```rust
#[account]
#[derive(InitSpace)]
pub struct GestorSuscripciones {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_usuario: String,
    #[max_len(10)]
    pub suscripciones: Vec<Suscripcion>,
}
```

---

### 🧩 Estructura Interna: `Suscripcion`

Cada elemento dentro del vector contiene:

- `nombre (String)` → máximo 30 caracteres  
- `costo (u32)` → optimizado en memoria (sin decimales)  
- `activa (bool)` → estado de la suscripción  

---

## 🔒 Seguridad

El contrato implementa validación estricta de propiedad:

```rust
require!(
    gestor.owner == ctx.accounts.owner.key(),
    Errores::NoEresElOwner
);
```

✔ Solo el creador puede modificar sus datos  
✔ Previene accesos no autorizados  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Gestor

Crea la cuenta principal usando:

```rust
[b"gestor", owner.key()]
```

Inicializa:
- Owner  
- Nombre de usuario  
- Vector vacío (`Vec::new()`)  

---

### ➕ Agregar Suscripción

- Recibe nombre y costo  
- Asigna automáticamente `activa = true`  
- Inserta con `.push()`  

---

### ✏️ Editar Suscripción

- Busca por nombre  
- Actualiza:
  - costo  
  - estado (`activa`)  

---

### ❌ Eliminar Suscripción

```rust
.iter().position(|s| s.nombre == nombre)
```

- Si existe → `.remove(index)`  
- Si no → error `SuscripcionNoEncontrada`  

---

### 📖 Ver Suscripciones

```rust
msg!("{:#?}", gestor.suscripciones);
```

Muestra los datos en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `nombre: String`  
- `costo: u32`  
- `activa: bool`  

---

## 📌 Conclusión

Este proyecto demuestra:

- Uso eficiente de memoria en Solana  
- Seguridad basada en firmas  
- Manejo estructurado de datos On-Chain  
- Implementación de CRUD en blockchain  

---

## 🚀 Próximos pasos

- Integrar frontend (React / Next.js)  
- Automatizar pagos con lógica off-chain  
- Añadir tokens SPL para pagos reales  
- Implementar métricas de gasto  

---
