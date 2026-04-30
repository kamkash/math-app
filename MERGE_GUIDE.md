# Math App - Unified Tauri + Axum Web Project

This is a merged workspace combining:
- **math-app**: Tauri desktop application
- **math-app-web**: Axum web server backend
- Shared mathematical computation crates

## Project Structure

```
math-app/
├── Cargo.toml                 # Workspace root with all crates
├── src-tauri/                 # Tauri desktop app (Rust)
├── src/                       # Tauri npm/vite frontend
├── axum-backend/              # NEW: Axum web server crate
├── web-frontend/              # NEW: Web frontend (Svelte + Vite)
├── [shared math crates]/
│   ├── math-core/
│   ├── math-parser/
│   ├── interpreter/
│   └── ...
```

## Building and Running

### 1. Tauri Desktop App

Build the Tauri desktop application:

```bash
cd /Users/kamran/mathappws/math-app
cargo tauri build
```

Or run in development mode:

```bash
cargo tauri dev
```

### 2. Axum Web Application

The Axum backend can be run as a standalone web server:

```bash
cd /Users/kamran/mathappws/math-app
cargo build --release -p axum-backend
./target/release/axum-backend
```

The server will start on `http://127.0.0.1:3000`

#### With Web Frontend

Build the web frontend first:

```bash
cd web-frontend
npm install
npm run build
```

Then run the Axum backend (it will serve the built frontend from `web-frontend/dist`):

```bash
cargo run -p axum-backend
```

Visit `http://127.0.0.1:3000/web` to access the web interface.

#### Development with Hot Reload

For frontend development with hot reload:

```bash
cd web-frontend
npm run dev
```

This will start a Vite dev server on `http://localhost:5173`

For backend development, run in a separate terminal:

```bash
cargo run -p axum-backend
```

### 3. Available API Endpoints

- `POST /api/hello` - Send a greeting request
- `GET /api/list` - Get a list of names

## Workspace Dependencies

The workspace uses shared dependencies configured in the root `Cargo.toml`:

- `serde` - Serialization framework
- `serde_json` - JSON support
- `log` - Logging
- All math crates are available to both `src-tauri` and `axum-backend`

## Shared Math Crates

The following crates are available to both the Tauri and web backends:

- `math-core` - Core mathematical functionality
- `math-parser` - Expression parsing
- `interpreter` - Expression interpretation
- `calculus` - Calculus operations
- `linear-algebra` - Linear algebra operations
- `statistics` - Statistical functions
- `algebra` - Algebraic operations
- `symbolic-engine` - Symbolic computation
- `giac-rs` - GiAC wrapper
- `symengine-rs` - SymEngine wrapper

## Integration Steps

### For math-app-web developers:

The Axum backend has been migrated as `axum-backend/`. Key changes:

1. **Cargo.toml**: Now references workspace dependencies and shared math crates
2. **main.rs**: Updated to serve frontend from `web-frontend/dist` instead of `dist/`
3. **Dependencies**: All workspace crates are now available (math-core, interpreter, etc.)

### For math-app developers:

The desktop Tauri app remains unchanged. You can now also use shared math crates in the web backend.

## Development Workflow

1. **Code changes to math crates**: Available immediately to both frontends
2. **Tauri frontend changes**: Rebuild with `cargo tauri build`
3. **Web frontend changes**: Rebuild with `npm run build` in web-frontend/
4. **Backend changes**: Recompile with `cargo build` or `cargo run`

## Cross-Platform Considerations

- **Tauri App**: Native desktop (Windows, macOS, Linux)
- **Web App**: Browser-based via Axum server
- Both share the same Rust math backend for consistency

## Next Steps

1. Integrate math-core APIs into Axum handlers in `axum-backend/src/main.rs`
2. Add more sophisticated API routes for mathematical operations
3. Connect web frontend UI to backend API endpoints
4. Configure CORS if needed for external access
5. Add authentication/authorization as needed

## Notes

- The web frontend and Tauri frontend are separate. You may want to share components or styling in the future.
- Static asset serving in Axum looks for `web-frontend/dist/` - ensure the frontend is built before running the server.
- Both applications can be developed independently while sharing the mathematical computation backend.
