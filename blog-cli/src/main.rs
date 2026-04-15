use blog_client::{BlogClient, BlogClientApi};
use clap::{Parser, Subcommand};
use tokio::fs;
use uuid::Uuid;

const DEFAULT_HTTP_ADDRESS: &str = "http://localhost:8080";
const DEFAULT_GRPC_ADDRESS: &str = "http://localhost:50051";
const TOKEN_FILE: &str = ".blog_token";

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
    let token = fs::read_to_string(TOKEN_FILE)
        .await
        .ok();

    if cli.grpc {
        let url = cli
            .server
            .unwrap_or_else(|| DEFAULT_GRPC_ADDRESS.to_string());

        let mut client = BlogClient::new_grpc(url).await?;
        run_command(&mut client, cli.command, token).await?;
    } else {
        let url = cli
            .server
            .unwrap_or_else(|| DEFAULT_HTTP_ADDRESS.to_string());

        let mut client = BlogClient::new_http(url)?;
        run_command(&mut client, cli.command, token).await?;
    }

    Ok(())
}

async fn run_command<ClientApi>(client: &mut ClientApi, command: Commands, token: Option<String>) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    if let Some(token) = token {
        client.set_token(token);
    }

    match command {
        Commands::Register { name, email, password} => register(client, name, email, password).await?,
        Commands::Login { name, password } => login(client, name, password).await?,
        Commands::Create { title, content } => create(client, title, content).await?,

        Commands::Get { id } => get_post(client, id).await?,
        Commands::Update { id, title, content } => update_post(client, id, title, content).await?,
        Commands::Delete { id } => delete_post(client, id).await?,
        Commands::List { limit, offset } => list(client, limit, offset).await?,
    }

    Ok(())
}

async fn register<ClientApi>(client: &mut ClientApi, name: String, email: String, password: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let user_with_token = client.register(name, email, password).await?;
    let user = user_with_token.user;
    println!("Пользователь зарегистрирован: {} ({})", user.name, user.id);

    let token = user_with_token.token.clone();
    client.set_token(user_with_token.token);
    tokio::fs::write(TOKEN_FILE, token).await?;

    Ok(())
}

async fn login<ClientApi>(client: &mut ClientApi, name: String, password: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let user_with_token = client.login(name, password).await?;
    let user = user_with_token.user;

    println!("Вход выполнен: {} ({})", user.name, user.id);

    let token = user_with_token.token.clone();
    client.set_token(user_with_token.token);
    tokio::fs::write(TOKEN_FILE, token).await?;

    Ok(())
}

async fn create<ClientApi>(client: &ClientApi, title: String, content: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let post = client.create_post(title, content).await?;
    println!("Пост создан: {} ({})", post.title, post.id);

    Ok(())
}

async fn get_post<ClientApi>(client: &ClientApi, id: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let post_id = Uuid::parse_str(&id)?;
    let post = client.get_post(post_id).await?;

    println!("Пост: {} ({})", post.title, post.id);

    Ok(())
}

async fn update_post<ClientApi>(client: &ClientApi, id: String, title: String, content: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let post_id = Uuid::parse_str(&id)?;
    let post = client.update_post(post_id, title, content).await?;

    println!("Пост обновлён: {} ({})", post.title, post.id);

    Ok(())
}

async fn delete_post<ClientApi>(client: &ClientApi, id: String) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let post_id = Uuid::parse_str(&id)?;
    client.delete_post(post_id).await?;

    println!("Пост удалён: {}", post_id);

    Ok(())
}

async fn list<ClientApi>(client: &ClientApi, limit: u32, offset: u32) -> anyhow::Result<()>
where
    ClientApi: BlogClientApi,
{
    let response = client.list_posts(limit, offset).await?;
    println!(
        "Всего постов: {}, limit: {}, offset: {}",
        response.total, response.limit, response.offset
    );

    for post in response.posts {
        println!("{}: {} ({})", post.id, post.title, post.content);
    }

    Ok(())
}