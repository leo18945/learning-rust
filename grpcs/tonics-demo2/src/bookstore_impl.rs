use tonic::{Request, Response, Status};
use crate::bookstore::bookstore_server::Bookstore;
use crate::{GetBookRequest, GetBookResponse};

#[derive(Default)]
pub struct BookStoreImpl {}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    async fn get_book(&self, request: Request<GetBookRequest>) -> Result<Response<GetBookResponse>, Status> {
        println!("remote addr={:?}", request.remote_addr().unwrap());
        let response = GetBookResponse{
            id: "ISBN-9-1478204".to_string(),
            name: "Thinking in Java".to_string(),
            author: "James Gosling".to_string(),
            year: 2020,
        };
        Ok(Response::new(response))
    }
}
