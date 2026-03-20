### 🧠 Descripción del Proyecto
ATENTIX es una plataforma interactiva diseñada para mejorar la concentración y el rendimiento cognitivo en personas con TDAH. Combinamos actividades didácticas con la tecnología blockchain para ofrecer un seguimiento de progreso inmutable y motivador.

### 🚀 Implementación en Solana
El proyecto utiliza un Smart Contract (Programa) en Solana para gestionar los perfiles de los usuarios y registrar sus logros de forma segura:
- **lib.rs**: Maneja la creación de perfiles y el almacenamiento de puntajes (XP).
- **client.ts**: Actúa como el puente para que los juegos envíen datos a la blockchain.

### 🎨 Diseño y Experiencia de Usuario (UX)
Hemos diseñado una interfaz con baja carga cognitiva y colores relajantes para facilitar la atención. Puedes ver el diseño propuesto en la carpeta `/mockups`.

### 🛠️ Cómo probar el proyecto
1. Clonar el repositorio.
2. Ejecutar `anchor build` para compilar el programa.
3. Ejecutar `anchor test` para verificar la lógica con el cliente TypeScript.
