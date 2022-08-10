// use tonic::{Request, Response, Status};
// use tonic::transport::Server;
// use crate::bookstore::bookstore_server::{Bookstore, BookstoreServer};
// use crate::bookstore::{GetBookRequest, GetBookResponse};
//
// mod bookstore {
//     include!("bookstore.rs");
//
//     // Add this
//     pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
//         tonic::include_file_descriptor_set!("greeter_descriptor");
// }
//
// #[derive(Default)]
// struct BookStoreImpl {}
//
// #[tonic::async_trait]
// impl Bookstore for BookStoreImpl {
//     async fn get_book(&self, request: Request<GetBookRequest>) -> Result<Response<GetBookResponse>, Status> {
//         println!("remote addr={:?}", request.remote_addr().unwrap());
//         let response = GetBookResponse{
//             id: "ISBN-9-1478204".to_string(),
//             name: "Thinking in Java".to_string(),
//             author: "James Gosling".to_string(),
//             year: 2020,
//         };
//         Ok(Response::new(response))
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Server starting...");
//     let addr = "[::1]:50051".parse().unwrap();
//     println!("Server listening on {}", addr);
//
//     let bookstore = BookStoreImpl::default();
//     // add this line
//     let reflection_service = tonic_reflection::server::Builder::configure()
//         .register_encoded_file_descriptor_set(bookstore::FILE_DESCRIPTOR_SET)
//         .build()
//         .unwrap();
//
//     Server::builder()
//         .add_service(BookstoreServer::new(bookstore))
//         .add_service(reflection_service) // add this line
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }