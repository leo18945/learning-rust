use tonic::Request;
use records::recorder_client::RecorderClient;
use records::RecordRequest;

pub mod records {
    tonic::include_proto!("records");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("client");
    let mut client = RecorderClient::connect("http://127.0.0.1:50050").await?;
    let request = Request::new(RecordRequest{
        user_name: "leo".to_string(),
        user_age: "20".to_string(),
    });
    let response = client.send_message(request).await?;
    println!("response={:#?}", response);
    Ok(())
}