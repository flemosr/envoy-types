use tonic::{transport::Server, Request, Response, Status};

use envoy_types::ext_authz::v3::pb::{
    Authorization, AuthorizationServer, CheckRequest, CheckResponse,
};
use envoy_types::ext_authz::v3::{CheckRequestExt, CheckResponseExt};

#[derive(Default)]
struct MyServer;

#[tonic::async_trait]
impl Authorization for MyServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        let request = request.into_inner();

        let client_headers = request
            .get_client_headers()
            .ok_or_else(|| Status::invalid_argument("client headers not populated by envoy"))?;

        let mut request_status = Status::unauthenticated("not authorized");

        if let Some(authorization) = client_headers.get("authorization") {
            if authorization == "Bearer valid-token" {
                request_status = Status::ok("request is valid");
            }
        }

        Ok(Response::new(CheckResponse::with_status(request_status)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:50051").parse().unwrap();
    let server = MyServer::default();

    println!("AuthorizationServer listening on {addr}");

    let reflection_server = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(envoy_types::pb::FILE_DESCRIPTOR_SET)
    .build()?;

    Server::builder()
        .add_service(AuthorizationServer::new(server))
        .add_service(reflection_server)
        .serve(addr)
        .await?;

    Ok(())
}
