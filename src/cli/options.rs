use clap::Args;

#[derive(Args)]
pub struct NewOptions {
    #[arg(long, help = "Include SQLAlchemy ORM support")]
    pub sql: bool,
    #[arg(long, help = "Include JWT authentication support")]
    pub auth: bool,
    #[arg(long, help = "Include CORS middleware")]
    pub cors: bool,
    #[arg(long, help = "Include Redis and FastAPI-Cache support for caching")]
    pub cache: bool,
    #[arg(long, help = "Include Celery task queue and Flower monitoring support")]
    pub tasks: bool,
    #[arg(long, help = "Include Supabase integration")]
    pub supabase: bool,
}
