use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balance { user } => {
            println!("Баланс пользователя {user}: 1000₽");
        }
        Commands::Transfer { from, to, amount } => {
            println!("Переводим {amount}₽ от {from} к {to}");
        }
        Commands::History { user } => {
            match user {
                Some(u) => println!("История операций пользователя {u}: ..."),
                None => println!("История всех операций: ..."),
            }
        }
    }
}