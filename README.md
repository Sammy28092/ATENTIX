ATENTIX
Plataforma Cognitiva en Solana

ATENTIX es una plataforma interactiva diseñada para mejorar la concentración y el rendimiento cognitivo en personas con TDAH. Combina actividades didácticas con tecnología blockchain para ofrecer un seguimiento del progreso de forma segura, transparente y motivadora.
---
Propuesta de Valor
Las personas con TDAH suelen tener dificultades para mantener la constancia en actividades cognitivas. ATENTIX aborda este problema mediante:
- Actividades interactivas que estimulan la atención, memoria y velocidad de respuesta.
- Retroalimentación inmediata a través de un sistema de puntaje.
- Registro del progreso del usuario en blockchain para mayor seguridad y persistencia.
---
Integración con Solana
El proyecto utiliza un smart contract desarrollado en Solana para:
- Crear perfiles de usuario.
- Registrar puntajes obtenidos en actividades.
- Mantener un historial de progreso seguro e inmutable.
---
Arquitectura del Proyecto (MVP)
El repositorio está organizado para demostrar una integración completa:
1. Smart Contract (On-Chain)
   - Ubicación: "/programs/src/lib.rs"
   - Desarrollado en Rust con Anchor.
   - Gestiona perfiles de usuario y almacenamiento de puntajes.

2. Cliente (Interacción con Blockchain)
   - Ubicación: "/tests/client.ts"
   - Script en TypeScript que simula la interacción con el contrato mediante funciones como creación de perfil y registro de puntaje.

3. Frontend (Prototipo Funcional)
   - Ubicación: "/frontend_prototype/index.html"
   - Aplicación web en HTML
   - Incluye juegos cognitivos como:
     - Reacción
     - Memoria
     - Velocidad
     - Secuencia
   - Simula conexión con wallet y envío de datos a la blockchain.

4. Mockups y Diseño
   - Ubicación: "/docs/"
   - Contiene los diseños de interfaz y flujo de usuario.
---
Diseño de Experiencia (UX)
ATENTIX está diseñado para reducir la sobrecarga cognitiva:
- Interfaz limpia y minimalista.
- Uso de colores suaves para evitar distracciones.
- Feedback inmediato para mantener la motivación del usuario.
---
Instalación y Pruebas
Para probar el proyecto:
1. Instalar dependencias de Solana y Anchor.
2. Ejecutar:
   anchor build
anchor test
3. Abrir el frontend:
   - Ejecutar "index.html" en el navegador.
---
Estado del Proyecto
Este proyecto es un prototipo funcional (MVP) desarrollado para un hackatón.
El frontend simula la interacción con la blockchain, mientras que el backend en Solana implementa la lógica real del sistema.
---
Proyecto desarrollado para el Solana Hackathon 2026
