# Template API Rust

¡Bienvenido a **Template API Rust**!  
Una plantilla minimalista y moderna para construir APIs RESTful en Rust, con enfoque en rendimiento, asincronía y buenas prácticas.

## 📖 Descripción

Este proyecto es una base sólida para desarrollar una API backend en Rust. Incluye:

- Servidor web con **Axum** (rápido, modular y basado en Hyper)
- Runtime asíncrono con **Tokio** (full features)
- Conexión a base de datos PostgreSQL mediante **SQLx** (type-safe, async, con migraciones)
- Logging y tracing con **tracing** + **tracing-subscriber**
- Carga de variables de entorno con **dotenvy**
- Soporte completo para Docker y Docker Compose (imagen optimizada multi-stage)

Ideal para prototipos rápidos, microservicios o proyectos *production-ready*.

## 🛠️ Tecnologías utilizadas

- **Rust** 1.83
- **Axum** 0.7 (con macros)
- **Tokio** 1.37 (full)
- **SQLx** 0.6  
  - runtime-tokio-native-tls  
  - postgres  
  - chrono  
  - uuid
- **Serde** + **Serde JSON**
- **Tracing** + **Tracing Subscriber**
- **UUID**, **Chrono**, **Anyhow**, **Async-Trait**
- **Dotenvy**

## 🚀 Inicio rápido

### 1. Ejecución local (sin Docker)

```bash
# Clona el repositorio
git clone https://github.com/tu-usuario/template_api_rust.git
cd template_api_rust

# Copia el ejemplo de variables de entorno
cp .env.example .env

# Edita .env con tu DATABASE_URL
nano .env

# Ejecuta la aplicación
cargo run --release
```

### 2. Ejecución con Docker Compose (recomendado)

```bash
# Copia el ejemplo de variables de entorno
cp .env.example .env

# Edita .env con tu DATABASE_URL
nano .env

# Levanta la API
docker compose up --build
```

### 3. Ejecución solo con Docker (sin Compose)

```bash
docker build -t template-api-rust .
docker run --rm -p 3000:3000 --env-file .env template-api-rust
```

## ⚙️ Variables de entorno

Crea un archivo `.env` en la raíz del proyecto con al menos:

```bash
DATABASE_URL=postgresql://usuario:contraseña@host:5432/base_de_datos?sslmode=require

# Opcionales
RUST_LOG=debug
PORT=3000
JWT_SECRET=super-secret
```

## 📂 Estructura del proyecto

```
.
├── src/                # Código fuente principal
├── Cargo.toml          # Dependencias y configuración de Rust
├── Dockerfile          # Imagen Docker optimizada (multi-stage)
├── docker-compose.yml  # Docker Compose (opcional DB local)
├── .env.example        # Ejemplo de variables de entorno
└── README.md           # Documentación del proyecto
```

## 🧪 Pruebas

```bash
cargo test
```

## 🤝 Contribuir

Las contribuciones son bienvenidas:

1. Haz fork del repositorio

2. Crea una rama:
```bash
git checkout -b feature/nueva-funcionalidad
```

3. Commit tus cambios:
```bash
git commit -m "Add nueva funcionalidad"
```

4. Push a la rama:
```bash
git push origin feature/nueva-funcionalidad
```

5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Eres libre de usarlo, modificarlo y distribuirlo.

## 👨‍💻 Autor

**Henry Vinicio Simbaña Cruz**  
📍 Machala, Ecuador  
🐙 GitHub: [@henvisi-1994](https://github.com/henvisi-1994)

Si tienes dudas o sugerencias, abre un issue o contáctame.

---

¡Éxitos con tu API! 🚀
