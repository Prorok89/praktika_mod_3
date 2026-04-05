use blog_client::{BlogClient, Transport, AuthResponse, Post};
use clap::{Parser, Subcommand};
use chrono::DateTime;
use std::path::Path;

const TOKEN_FILE: &str = ".blog_token";

#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI клиент для взаимодействия с блогом")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Использовать gRPC вместо HTTP
    #[arg(long)]
    grpc: bool,

    /// Порт сервера (по умолчанию: 8080 для HTTP, 50051 для gRPC)
    #[arg(long)]
    port: Option<String>,

    /// Адрес сервера (по умолчанию: localhost:8080 для HTTP, localhost:50051 для gRPC)
    #[arg(long, short)]
    server: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Регистрация нового пользователя
    Register {
        /// Имя пользователя
        #[arg(long)]
        username: String,

        /// Email пользователя
        #[arg(long)]
        email: String,

        /// Пароль пользователя
        #[arg(long)]
        password: String,
    },
    /// Вход в систему
    Login {
        /// Имя пользователя
        #[arg(long)]
        username: String,

        /// Пароль пользователя
        #[arg(long)]
        password: String,
    },
    /// Создание нового поста
    Create {
        /// Заголовок поста
        #[arg(long)]
        title: String,

        /// Содержимое поста
        #[arg(long)]
        content: String,
    },
    /// Получение поста по ID
    Get {
        /// ID поста
        #[arg(long)]
        id: i64,
    },
    /// Обновление поста
    Update {
        /// ID поста
        #[arg(long)]
        id: i64,

        /// Новый заголовок поста
        #[arg(long)]
        title: String,

        /// Новое содержимое поста
        #[arg(long)]
        content: String,
    },
    /// Удаление поста
    Delete {
        /// ID поста
        #[arg(long)]
        id: i64,
    },
    /// Список постов
    List {
        /// Количество постов для возврата
        #[arg(long, default_value = "10")]
        limit: i64,

        /// Смещение для пагинации
        #[arg(long, default_value = "0")]
        offset: i64,
    },
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let port = cli.port.unwrap_or_else(|| if cli.grpc { "50051" } else { "8080" }.to_string());
    let server_addr = if let Some(addr) = cli.server {
        addr
    } else {
        format!("http://localhost:{}", port)
    };
    let transport = if cli.grpc {
        Transport::Grpc(server_addr)
    } else {
        Transport::Http(server_addr)
    };
    let mut client = BlogClient::new(transport);

    if let Some(token) = load_token() {
        client.set_token(token);
    }

    let result = match cli.command {
        Commands::Register { username, email, password } => {
            client.register(username, email, password).await.map(|_response| {
                println!("✓ Регистрация успешна!");
            })
        }
        Commands::Login { username, password } => {
            client.login(username, password).await.map(|response| {
                println!("✓ Вход выполнен успешно!");
                print_auth_info(&response);
                save_token(&response.token);
            })
        }
        Commands::Create { title, content } => {
            client.create_post(title, content).await.map(|post| {
                println!("✓ Пост создан успешно!");
                print_post(&post);
            })
        }
        Commands::Get { id } => {
            client.get_post(id).await.map(|post| {
                print_post(&post);
            })
        }
        Commands::Update { id, title, content } => {
            client.update_post(id, title, content).await.map(|post| {
                println!("✓ Пост обновлён успешно!");
                print_post(&post);
            })
        }
        Commands::Delete { id } => {
            client.delete_post(id).await.map(|_| {
                println!("✓ Пост {} удалён успешно!", id);
            })
        }
        Commands::List { limit, offset } => {
            client.list_posts(limit, offset).await.map(|posts| {
                println!("Найдено постов: {}", posts.len());
                for post in posts {
                    print_post(&post);
                    println!();
                }
            })
        }
    };

    if let Err(e) = result {
        print_error(&e);
        std::process::exit(1);
    }
}

fn load_token() -> Option<String> {
    if Path::new(TOKEN_FILE).exists() {
        std::fs::read_to_string(TOKEN_FILE).ok()
    } else {
        None
    }
}

fn save_token(token: &str) {
    if let Err(e) = std::fs::write(TOKEN_FILE, token) {
        eprintln!("⚠ Не удалось сохранить токен: {}", e);
    }
}

fn print_error(e: &blog_client::BlogClientError) {
    eprintln!("\n⚠ Ошибка:");
    match e {
        blog_client::BlogClientError::HttpError(err) => {
            if let Some(status) = err.status() {
                match status {
                    reqwest::StatusCode::BAD_REQUEST => {
                        eprintln!("✗ Неверный запрос. Проверьте введенные данные.");
                    }
                    reqwest::StatusCode::UNAUTHORIZED => {
                        eprintln!("✗ Неавторизован. Пожалуйста, войдите в систему.");
                        eprintln!("  Попробуйте: blog-cli login --username <имя> --password <пароль>");
                    }
                    reqwest::StatusCode::FORBIDDEN => {
                        eprintln!("✗ Доступ запрещён.");
                    }
                    reqwest::StatusCode::NOT_FOUND => {
                        eprintln!("✗ Ресурс не найден.");
                    }
                    reqwest::StatusCode::CONFLICT => {
                        eprintln!("✗ Конфликт: пользователь с таким именем или email уже существует.");
                        eprintln!("  Попробуйте войти: blog-cli login --username <имя> --password <пароль>");
                    }
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                        eprintln!("✗ Ошибка сервера. Попробуйте позже.");
                    }
                    _ => {
                        eprintln!("✗ HTTP ошибка {}: {}", status, status.canonical_reason().unwrap_or("Неизвестная ошибка"));
                    }
                }
            } else {
                eprintln!("✗ HTTP ошибка: {}", err);
            }
        }
        blog_client::BlogClientError::GrpcError(err) => {
            eprintln!("✗ gRPC ошибка: {}", err.message());
        }
        blog_client::BlogClientError::TransportError(err) => {
            eprintln!("✗ Ошибка подключения к серверу: {}", err);
            eprintln!("  Проверьте, запущен ли сервер.");
        }
        blog_client::BlogClientError::NotFound(msg) => {
            eprintln!("✗ Не найдено: {}", msg);
        }
        blog_client::BlogClientError::Unauthorized => {
            eprintln!("✗ Неавторизован. Пожалуйста, войдите в систему.");
            eprintln!("  Попробуйте: blog-cli login --username <имя> --password <пароль>");
        }
        blog_client::BlogClientError::InvalidRequest(msg) => {
            eprintln!("✗ Неверный запрос: {}", msg);
        }
        blog_client::BlogClientError::SerializationError(err) => {
            eprintln!("✗ Ошибка обработки данных: {}", err);
        }
        blog_client::BlogClientError::InternalError(msg) => {
            eprintln!("✗ Внутренняя ошибка: {}", msg);
        }
    }
}

fn print_auth_info(response: &AuthResponse) {
    println!("\nТокен (сохранён в {}):", TOKEN_FILE);
    println!("{}\n", response.token);
    println!("Пользователь:");
    println!("  ID: {}", response.user.id.map(|id| id.to_string()).unwrap_or("не указан".to_string()));
    println!("  Имя: {}", response.user.username);
    println!("  Email: {}", response.user.email);
}

fn print_post(post: &Post) {
    let created: DateTime<chrono::Utc> = DateTime::from_timestamp(post.created_at, 0)
        .map(|dt| dt.to_utc())
        .unwrap_or(DateTime::<chrono::Utc>::MIN_UTC);
    let updated: DateTime<chrono::Utc> = DateTime::from_timestamp(post.updated_at, 0)
        .map(|dt| dt.to_utc())
        .unwrap_or(DateTime::<chrono::Utc>::MIN_UTC);

    println!("id: {}", post.id.map(|id| id.to_string()).unwrap_or("нет данных".to_string()));
    println!("заголовок: {}", post.title);
    println!("содержимое: {}", post.content);
    println!("author_id: {}", post.author_id);
    println!("created_at: {}", format_timestamp(&created));
    println!("updated_at: {}", format_timestamp(&updated));
}

fn format_timestamp(dt: &DateTime<chrono::Utc>) -> String {
    if *dt == DateTime::<chrono::Utc>::MIN_UTC {
        "неизвестно".to_string()
    } else {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}
