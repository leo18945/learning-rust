use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::bookstore::bookstore_server::{Bookstore, BookstoreServer};
use crate::bookstore::{GetBookRequest, GetBookResponse};
use crate::bookstore_impl::BookStoreImpl;

mod bookstore;
mod bookstore_impl;

// https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e
// Add this
pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("greeter_descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server starting...");
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {}", addr);

    let bookstore = BookStoreImpl::default();
    // add this line
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(BookstoreServer::new(bookstore))
        .add_service(reflection_service) // add this line
        .serve(addr)
        .await?;

    Ok(())
}