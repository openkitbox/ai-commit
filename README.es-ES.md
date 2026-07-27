# AI Commit Tool

Un generador inteligente de mensajes de commit de Git que utiliza IA para analizar tus cambios de código y crear mensajes de commit convencionales y significativos automáticamente.

## Descripción General

AI Commit Tool se integra en tu flujo de trabajo de Git para generar automáticamente mensajes de commit de alta calidad siguiendo la especificación de Conventional Commits. Analiza tus cambios preparados (staged) o no preparados (unstaged) y utiliza IA para redactar los mensajes de commit adecuados, ahorrando tiempo y asegurando la consistencia en todo tu proyecto.

## Características

- **Mensajes de commit generados por IA**: Analiza automáticamente los git diffs y genera mensajes de commit contextuales siguiendo el formato de commit convencional.
- **Generación guiada por palabras clave**: Proporciona palabras clave o contexto para guiar el enfoque de la IA en aspectos específicos de tus cambios.
- **Selección de formato inteligente**: Elige automáticamente entre mensajes concisos de una sola línea o un formato detallado de viñetas basado en la complejidad del cambio.
- **Modo Dry Run**: Previsualiza los mensajes generados sin realizar el commit mediante `--dry-run`.
- **Límite de contexto**: Controla cuánta cantidad de contenido del diff se envía a la IA mediante `--context-limit`.
- **Soporte para Amend**: Genera nuevos mensajes para modificar commits anteriores con cambios adicionales.
- **Filtrado de archivos de bloqueo**: Ignora automáticamente los archivos de bloqueo comunes (Cargo.lock, package-lock.json, yarn.lock, etc.) del análisis.
- **Patrones de ignorado personalizados**: Patrones basados en glob para filtrar archivos del análisis (ej. `**/generated/**`).
- **Soporte para firma GPG**: Funciona a la perfección con commits firmados con GPG.
- **Variables de entorno**: Las credenciales de la API se leen de las variables de entorno, nunca se almacenan en archivos de configuración.
- **Nombres de variables de entorno configurables**: Personaliza qué nombres de variables de entorno se utilizan a través de la sección `[env]`.
- **Totalmente configurable**: Comportamiento de commit, patrones de ignorado y prompts de IA personalizables.

## Instalación

### Prerrequisitos

- Rust (última versión estable)
- Repositorio Git
- Un endpoint de API compatible con OpenAI y una clave de API

### Construir desde el código fuente

```bash
git clone <repository-url>
cd ai-commit
cargo build --release
```

### Configuración

1. **Establecer variables de entorno**:

   ```bash
   export AI_COMMIT_API_KEY="tu-clave-de-api-aqui"
   export AI_COMMIT_MODEL="nombre-de-tu-modelo"
   # Opcional: anular el endpoint predeterminado
   export AI_COMMIT_ENDPOINT="https://tu-endpoint-de-api/chat/completions"
   ```

2. **Inicializar configuración** (opcional, crea una configuración predeterminada con plantillas de prompts):

   ```bash
   ai-commit config init
   ```

## Uso

### Comandos Básicos

Generar mensaje de commit para los cambios preparados (staged):

```bash
ai-commit
# o explícitamente
ai-commit commit
```

Generar mensaje de commit con palabras clave para guiar a la IA:

```bash
ai-commit -k "fix authentication bug"
ai-commit commit -k "add user profile feature"
```

Previsualizar el mensaje de commit sin realizar el commit (modo dry-run):

```bash
ai-commit --dry-run
ai-commit commit --dry-run
```

Limitar el contexto enviado a la IA:

```bash
ai-commit --context-limit 100000
```

Modificar el último commit con nuevos cambios:

```bash
ai-commit amend
ai-commit amend -k "improve error handling"
```

Combinar opciones:

```bash
ai-commit commit -k "improve performance" --dry-run --context-limit 50000
ai-commit amend -k "add validation" --dry-run
```

### Comandos de Configuración

```bash
# Inicializar configuración predeterminada
ai-commit config init

# Ver configuración actual (muestra nombres de variables de entorno, no valores)
ai-commit config show

# Obtener ayuda para editar prompts
ai-commit config edit-prompts
```

### Integración de Git Hooks

Instalar git hooks para asistencia automática en los mensajes de commit:

```bash
ai-commit install
```

Eliminar git hooks:

```bash
ai-commit uninstall
```

## Configuración

La herramienta almacena la configuración en `~/.config/ai-commit/config.toml`. Inicializa con los ajustes predeterminados:

```bash
ai-commit config init
```

### Estructura de Configuración

```toml
# Personaliza qué nombres de variables de entorno leer
# Se muestran los valores predeterminados — solo descomenta y cambia si es necesario
[env]
# endpoint_env = "AI_COMMIT_ENDPOINT"
# api_key_env = "AI_COMMIT_API_KEY"
# model_env = "AI_COMMIT_MODEL"

[commit]
auto_confirm = false
dry_run_by_default = false
ignore_lock_files = true
custom_ignore_patterns = []
context_limit = 200000

[hooks]
enabled = false
hook_types = []

[prompts]
system_prompt = """You are an expert software developer..."""
user_prompt_template = """Analyze the following git diff...
```diff
{diff}
```"""
```

### Opciones de Configuración

#### Variables de Entorno (`[env]`)

Personaliza los nombres de las variables de entorno utilizadas para leer las credenciales de la API. Esto permite integrarse con entornos CI/CD existentes sin configurar variables adicionales.

| Campo | Predeterminado | Descripción |
|-------|---------|-------------|
| `endpoint_env` | `AI_COMMIT_ENDPOINT` | Variable de entorno para la URL del endpoint de la API |
| `api_key_env` | `AI_COMMIT_API_KEY` | Variable de entorno para la clave de la API |
| `model_env` | `AI_COMMIT_MODEL` | Variable de entorno para el nombre del modelo |

Ejemplo — usando variables compatibles con OpenAI:

```toml
[env]
api_key_env = "OPENAI_API_KEY"
model_env = "OPENAI_MODEL"
endpoint_env = "OPENAI_BASE_URL"
```

#### Ajustes de Commit (`[commit]`)

| Campo | Predeterminado | Descripción |
|-------|---------|-------------|
| `auto_confirm` | `false` | Omitir el mensaje de confirmación |
| `dry_run_by_default` | `false` | Ejecutar siempre en modo dry-run |
| `ignore_lock_files` | `true` | Filtrar archivos de bloqueo del análisis |
| `custom_ignore_patterns` | `[]` | Patrones glob para archivos a ignorar (ej. `["**/generated/**"]`) |
| `context_limit` | `200000` | Máximo de caracteres del diff enviados a la IA |

#### Ajustes de Prompt (`[prompts]`)

| Campo | Descripción |
|-------|-------------|
| `system_prompt` | Prompt del sistema que define el comportamiento de la IA y el formato del commit |
| `user_prompt_template` | Plantilla para analizar diffs — debe contener el marcador `{diff}` |

### Personalización de Prompts de IA

Edita `~/.config/ai-commit/config.toml` para personalizar cómo la IA genera los mensajes de commit:

```toml
[prompts]
system_prompt = """You are a senior developer focused on clear, concise commits.
Generate conventional commit messages prioritizing single-line format.
Use bullet points only for truly unrelated changes."""

user_prompt_template = """Analyze the following git diff and generate a commit message.
Prefer single-line format under 72 characters.

Git diff:
```diff
{diff}
```

Provide only the commit message."""
```

**Consejos:**
- Mantén el marcador `{diff}` en las plantillas.
- Prueba los cambios con `ai-commit --dry-run`.
- La configuración se recarga automáticamente en la siguiente ejecución.

## Formato de Mensajes de Commit

La herramienta genera mensajes siguiendo la especificación de Conventional Commits.

### Formato de una sola línea (preferido)

Utilizado para cambios enfocados con un único propósito:

```
feat: add user authentication system
fix: resolve database connection timeout
refactor: improve error handling in auth module
```

### Formato multilínea (para cambios complejos)

Utilizado cuando hay múltiples cambios funcionales verdaderamente no relacionados:

```
feat: add user management and notification system

- Implement user CRUD operations with validation
- Add email notification service for user events
- Create admin dashboard for user management
```

### Tipos Soportados

| Tipo | Descripción |
|------|-------------|
| `feat` | Una nueva característica |
| `fix` | Corrección de un error |
| `docs` | Cambios solo en la documentación |
| `style` | Cambios de estilo de código (formateo, etc.) |
| `refactor` | Cambios de código que no corrigen errores ni añaden características |
| `perf` | Mejoras de rendimiento |
| `test` | Adición o corrección de pruebas |
| `chore` | Cambios en el proceso de construcción o herramientas auxiliares |

## Ejemplos de Flujo de Trabajo

### Configuración Inicial

```bash
export AI_COMMIT_API_KEY="tu-clave-api"
export AI_COMMIT_MODEL="tu-modelo"
ai-commit config init
ai-commit config show
```

### Flujo Estándar

```bash
git add .
ai-commit
# Revisar el mensaje generado y confirmar
```

### Flujo de Amend

```bash
git add .
ai-commit amend
# Revisar el mensaje generado para los cambios combinados
```

### Previsualizar Cambios

```bash
git add .
ai-commit --dry-run
```

### Uso de un Proveedor de IA Diferente

```bash
export AI_COMMIT_API_KEY="sk-..."
export AI_COMMIT_MODEL="gpt-4o"
export AI_COMMIT_ENDPOINT="https://api.openai.com/v1/chat/completions"
ai-commit commit
```

O configura nombres de variables de entorno personalizados en `config.toml`:

```toml
[env]
api_key_env = "OPENAI_API_KEY"
model_env = "OPENAI_MODEL"
endpoint_env = "OPENAI_BASE_URL"
```

## Detalles Técnicos

### Análisis de Diff

- Analiza los git diffs para comprender los cambios de código.
- Filtra automáticamente los archivos de bloqueo y artefactos de construcción.
- Soporta patrones de ignorado personalizados basados en glob.
- Impone un límite de contexto configurable para evitar solicitudes de API excesivas.
- Soporta el análisis tanto de cambios preparados (staged) como no preparados (unstaged).

### Integración de IA

- Compatible con cualquier API de chat completions compatible con OpenAI.
- Credenciales de API gestionadas únicamente a través de variables de entorno (nunca almacenadas en archivos).
- Tiempo de espera de conexión HTTP (10s conexión, 60s total).
- Maneja los errores de la API elegantemente con códigos de salida adecuados.
- Soporta prompts totalmente personalizables para diferentes estilos de commit.

### Seguridad

- Las claves de API se leen de las variables de entorno, nunca se escriben en archivos de configuración.
- `config show` muestra solo los nombres de las variables de entorno, nunca los valores reales.
- Funciona con commits firmados con GPG.
- Respeta los ajustes de configuración de git.

## Contribuciones

¡Las contribuciones son bienvenidas! No dudes en:

- Enviar informes de errores y solicitudes de funciones a través de issues.
- Crear pull requests para mejoras.
- Compartir comentarios y sugerencias.

### Configuración de Desarrollo

```bash
git clone <repository-url>
cd ai-commit
cargo test
cargo clippy
cargo run -- --help
```

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Consulta el archivo LICENSE para más detalles.
