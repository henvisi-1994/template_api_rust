# Guía para contribuir a Template API Rust

¡Gracias por tu interés en colaborar! 🦀 Este proyecto es una plantilla para construir APIs REST en Rust (Axum + Tokio + SQLx + PostgreSQL, con autenticación JWT) y toda ayuda es bienvenida, sin importar tu nivel de experiencia con Rust.

## Formas de contribuir

- 🐛 Reportar errores
- 💡 Proponer mejoras o nuevas funcionalidades
- 🔧 Enviar código (Pull Requests)
- 📖 Mejorar la documentación o corregir erratas
- 🧪 Agregar pruebas
- ⭐ Dar una estrella al repositorio y compartirlo

Si es tu primera vez, busca los issues con la etiqueta [`good first issue`](https://github.com/henvisi-1994/template_api_rust/labels/good%20first%20issue).

## Antes de empezar

1. Revisa los [issues abiertos](https://github.com/henvisi-1994/template_api_rust/issues) para evitar trabajo duplicado.
2. Si vas a hacer un cambio grande, abre primero un issue para conversarlo.
3. Comenta en el issue que te gustaría trabajar para que otros lo sepan.

## Preparar tu entorno

Necesitas:

- [Rust](https://www.rust-lang.org/tools/install) (se recomienda la versión estable más reciente)
- [Docker](https://docs.docker.com/get-docker/) y Docker Compose (opcional, pero recomendado)
- PostgreSQL (si no usas Docker)

```bash
# 1. Haz fork del repositorio desde GitHub y clónalo
git clone https://github.com/<tu-usuario>/template_api_rust.git
cd template_api_rust

# 2. Configura las variables de entorno
cp .env.example .env
# Edita .env con tu DATABASE_URL y define un JWT_SECRET

# 3. Ejecuta el proyecto
cargo run
# o con Docker
docker compose up --build
```

Al arrancar, las migraciones de SQLx se ejecutan automáticamente (crean la tabla `users`).

## Flujo de trabajo

1. Crea una rama desde `main` con un nombre descriptivo:

   ```bash
   git checkout -b feature/nombre-de-la-mejora
   # o: fix/descripcion-del-error, docs/descripcion
   ```

2. Haz tus cambios, con commits pequeños y claros.
3. Antes de enviar, verifica que todo pase:

   ```bash
   cargo fmt --all           # formato del código
   cargo clippy -- -D warnings   # lints
   cargo test                # pruebas
   ```

4. Sube tu rama y abre un Pull Request hacia `main`:

   ```bash
   git push origin feature/nombre-de-la-mejora
   ```

## Convención de commits

Usamos mensajes cortos en imperativo, con un prefijo opcional:

- `feat:` nueva funcionalidad
- `fix:` corrección de un error
- `docs:` cambios en documentación
- `test:` pruebas
- `refactor:` cambios internos sin alterar el comportamiento
- `chore:` tareas de mantenimiento

Ejemplo: `feat: agregar endpoint de health check`

## Qué incluir en tu Pull Request

- Una descripción clara de **qué** cambia y **por qué**.
- El número del issue relacionado (por ejemplo, `Closes #12`).
- Pruebas para el nuevo comportamiento, cuando aplique.
- Actualización del README o de la documentación si cambia la forma de usar el proyecto.
- Cambios enfocados: un PR por tema es más fácil de revisar.

## Reportar errores

Al abrir un issue de error, incluye:

- Qué esperabas que pasara y qué pasó realmente
- Pasos para reproducirlo
- Tu sistema operativo y versión de Rust (`rustc --version`)
- Logs o mensajes de error relevantes (sin datos sensibles como contraseñas o `DATABASE_URL` reales)

## Proponer mejoras

Abre un issue explicando el problema que resolvería tu idea, cómo imaginas la solución y alternativas que consideraste.

## Estilo de código

- Usa `cargo fmt` para el formato.
- Resuelve los avisos de `cargo clippy`.
- Prefiere nombres claros y comentarios que expliquen el **porqué**, no el qué.
- Evita agregar dependencias nuevas sin justificarlas en el PR.

## Código de conducta

Queremos una comunidad respetuosa y amable. Trata a todas las personas con cortesía, acepta las críticas constructivas con apertura y ayuda a quienes están aprendiendo. No se tolera el acoso, la discriminación ni los comentarios ofensivos.

## Licencia

Al contribuir, aceptas que tus aportes se publiquen bajo la licencia MIT del proyecto.

---

¿Dudas? Abre un issue o escribe en la sección de Discussions. ¡Gracias por ayudar a mejorar este proyecto! 🚀
