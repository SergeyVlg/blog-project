use blog_client::{BlogClient, BlogClientApi};
use clap::{Parser, Subcommand};
use uuid::Uuid;

const DEFAULT_HTTP_ADDRESS: &str = "http://localhost:8080";
const DEFAULT_GRPC_ADDRESS: &str = "http://localhost:50051";

#[derive(Parser)]
#[command(name = "blog", about = "CLI-утилита для работы с блогом", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    grpc: bool,
    #[arg(long)]
    server: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Register {
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    Login {
        #[arg(long)]
        name: String,
        #[arg(long)]
        password: String,
    },
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    Get {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    List {
        #[arg(long)]
        limit: u32,
        #[arg(long)]
        offset: u32,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;

    if cli.grpc {
        let url = cli
            .server
            .unwrap_or_else(|| DEFAULT_GRPC_ADDRESS.to_string());

        let mut client = BlogClient::new_grpc(url).await?;
        run_command(&mut client, cli.command).await?;
    } else {
        let url = cli
            .server
            .unwrap_or_else(|| DEFAULT_HTTP_ADDRESS.to_string());

        let mut client = BlogClient::new_http(url)?;
        run_command(&mut client, cli.command).await?;
    }

    Ok(())
}

async fn run_command<ClientApi>(client: &mut ClientApi, command: Commands) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    match command {
        Commands::Register { name, email, password } => {
            let user = client.register(name, email, password).await?;

            println!("Пользователь зарегистрирован: {} ({})", user.name, user.id);
        }
        Commands::Login { name, password } => {
            let user = client.login(name, password).await?;

            println!("Вход выполнен: {} ({})", user.name, user.id);
        }
        Commands::Create { title, content } => {
            let post = client.create_post(title, content).await?;

            println!("Пост создан: {} ({})", post.title, post.id);
        }
        Commands::Get { id } => {
            let post_id = Uuid::parse_str(&id)?;
            let post = client.get_post(post_id).await?;

            println!("Пост: {} ({})", post.title, post.id);
        }
        Commands::Update { id, title, content } => {
            let post_id = Uuid::parse_str(&id)?;
            let post = client.update_post(post_id, title, content).await?;

            println!("Пост обновлён: {} ({})", post.title, post.id);
        }
        Commands::Delete { id } => {
            let post_id = Uuid::parse_str(&id)?;
            client.delete_post(post_id).await?;

            println!("Пост удалён: {}", post_id);
        }
        Commands::List { limit, offset } => {
            let response = client.list_posts(limit, offset).await?;
            println!("Всего постов: {}, limit: {}, offset: {}", response.total, response.limit, response.offset);

            for post in response.posts {
                println!("{}: {} ({})", post.id, post.title, post.content);
            }
        }
    }

    Ok(())
}