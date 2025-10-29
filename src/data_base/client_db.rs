use dotenv::dotenv;
use std::env;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use anyhow::Result;

pub async fn client_t() -> Result<Client<Compat<TcpStream>>> {
    dotenv().ok();
    let conn = env::var("CONNECTION_STRING")?;

    let config = Config::from_ado_string(conn.as_str())?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    let tcp = tcp.compat(); 
    let client = Client::connect(config, tcp).await?;

    Ok(client)
}

pub async fn client_6pr()-> Result<Client<Compat<TcpStream>>> {
    dotenv().ok();
    let conn = env::var("CONNECTION_STRING_6PR")?;

    let config = Config::from_ado_string(conn.as_str())?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    let tcp = tcp.compat(); 
    let client = Client::connect(config, tcp).await?;

    Ok(client)
}