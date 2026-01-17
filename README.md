# Laravel God Mode

A powerful cross-platform desktop application built with **Tauri v2**, **Vue.js 3**, and **Tailwind CSS** for managing Laravel and Node.js Docker development environments, with this tool you don't need to spent time setip uo your system you can just use the power of docker and this tool to help you create laravel applications in no time.

![Laravel God Mode](./screenshots/dashboard.png)

## Features

### 🚀 Project Management
- Create Laravel and Node.js projects with one click
- Configurable PHP/Node versions
- Custom port mapping
- Multiple package manager support (npm, bun, pnpm, yarn)

### 🐳 Docker Integration
- Start, stop, restart, and rebuild containers
- Real-time container status monitoring
- Service management (MySQL, Redis, Nginx, phpMyAdmin, Mailhog)
- Container log viewing

### 🎨 Laravel Controls
- **Artisan Commands**: Run any artisan command with quick actions
- **Queue Management**: Start/stop workers, retry failed jobs
- **Supervisor**: Monitor and manage supervisor programs
- **Cache Management**: Clear config, routes, views, and more
- **Database**: Run migrations, seeders, or fresh install

### ⚙️ Settings
- Customizable projects directory
- Preferred code editor selection
- Default runtime versions
- Auto-start projects option

## Tech Stack

- **Frontend**: Vue.js 3, Tailwind CSS, Pinia, Vue Router
- **Backend**: Tauri v2 (Rust)
- **Container Runtime**: Docker & Docker Compose

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (latest stable)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/laravel-godmode.git
cd laravel-godmode
```

### 2. Install dependencies

```bash
npm install
```

### 3. Development

```bash
npm run tauri dev
```

### 4. Build for production

```bash
npm run tauri build
```

This will create platform-specific installers in `src-tauri/target/release/bundle/`.

## Project Structure

```
laravel-godmode/
├── src/                    # Vue.js frontend
│   ├── components/         # Vue components
│   ├── views/              # Page views
│   ├── stores/             # Pinia stores
│   ├── lib/                # API & types
│   ├── router/             # Vue Router
│   └── styles/             # Tailwind CSS
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri entry point
│   │   ├── commands.rs     # IPC commands
│   │   ├── docker.rs       # Docker management
│   │   ├── project.rs      # Project management
│   │   ├── template.rs     # Template engine
│   │   └── state.rs        # App state types
│   └── tauri.conf.json     # Tauri configuration
├── templates/              # Project templates
│   ├── laravel/
│   │   ├── config.json
│   │   └── stubs/
│   └── nodejs/
│       ├── config.json
│       └── stubs/
└── package.json
```

## Templates

### Laravel Template
- PHP 8.4 (configurable: 8.2-8.5)
- Node.js 18 LTS (configurable)
- MySQL 8.0
- Redis 7
- Nginx
- Supervisor
- Optional: phpMyAdmin, Mailhog

### Node.js Template
- Node.js 18 LTS (configurable)
- Express.js scaffold
- MySQL 8.0
- Redis 7
- Optional: phpMyAdmin, Mailhog

## Adding Custom Templates

1. Create a new folder in `templates/`
2. Add a `config.json` with template metadata
3. Create a `stubs/` folder with template files
4. Use `{{PLACEHOLDER}}` syntax for dynamic values

## Contributing

Contributions are welcome! Please read our contributing guidelines first.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

Built with ❤️ by Mario Tarosso

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Laravel Herd](https://herd.laravel.com/) (inspiration)
