use std::{io, sync::Arc};

mod adapter;
mod database;
mod core;
mod event_generator;
mod port;

use crate::{adapter::{
    inbound::server::new_server, 
    outbound::repository::new_repository,
}, core::service::service::new_service, database::db::create_database_connection, event_generator::{run_courier_movement, run_event_generator}};

#[tokio::main]
async fn main() -> io::Result<()> {

    // Create Database instance
    let db = create_database_connection().await.expect("Failed to create database connection");

    // Create Repository instance
    let repository = new_repository(db.connection);

    // Create Service instance, shared by the server and the event generator
    let service = Arc::new(new_service(repository));

    // Run the event generator as a background task next to the server
    let generator = tokio::spawn(run_event_generator(Arc::clone(&service)));

    // Run courier movement as its own background task (1 step per second)
    let movement = tokio::spawn(run_courier_movement(Arc::clone(&service)));

    // Create Server instance
    let server = new_server(service);
    let result = server.serve(shutdown_signal()).await;

    generator.abort();
    movement.abort();
    result
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("Failed to listen for Ctrl+C: {error}");
    }

    println!("Stopping web server and event generator...");
}
