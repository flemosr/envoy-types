use std::env;
use tonic::{transport::Server, Request, Response, Status};

use envoy_types::ext_authz::v3::pb::{
    Authorization, AuthorizationServer, CheckRequest, CheckResponse, HttpStatusCode,
};
use envoy_types::ext_authz::v3::{
    CheckRequestExt, CheckResponseExt, DeniedHttpResponseBuilder, OkHttpResponseBuilder,
};

#[derive(Default)]
struct MyServer;

fn denied_response(status: Status) -> Result<Response<CheckResponse>, Status> {
    let mut denied_http_response = DeniedHttpResponseBuilder::new();
    denied_http_response
        .set_http_status(HttpStatusCode::Forbidden)
        .add_header("downstream-header", "downstream-header-value", None, false)
        .set_body("FORBIDDEN");

    let mut denied_response = CheckResponse::with_status(status);
    denied_response.set_http_response(denied_http_response);

    Ok(Response::new(denied_response))
}

#[allow(unused)]
#[tonic::async_trait]
impl Authorization for MyServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        println!("{:?}", request);

        let request = request.into_inner();

        let client_address = request
            .get_client_address()
            .ok_or_else(|| Status::invalid_argument("client address not provided by envoy"))?;

        // Validate `client_address`
        // ...
        // Possible to implement some basic rate-limiting

        let client_headers = request
            .get_client_headers()
            .ok_or_else(|| Status::invalid_argument("client headers not populated by envoy"))?;

        let client_id = if let Some(authorization) = client_headers.get("authorization") {
            // Validate `authorization`
            // ...
            if authorization != "Bearer valid-token" {
                return denied_response(Status::unauthenticated(
                    "authorization token is not valid",
                ));
            }

            "client_id_value"
        } else {
            return denied_response(Status::unauthenticated(
                "authorization header not available",
            ));
        };

        let path = client_headers
            .get(":path")
            .ok_or_else(|| Status::invalid_argument(":path header not populated by envoy"))?;

        // Validate `path`
        // ...
        // Good place to check for cases of `Status::permission_denied`
        // since `client_id` could be known

        // Requests that reached this point are valid

        let mut ok_http_response = OkHttpResponseBuilder::new();
        ok_http_response
            .add_header("extauthz-header", "extauthz-value", None, false)
            .set_query_parameter("extauthz-query-param", "extauthz-query-value");

        let mut ok_response = CheckResponse::with_status(Status::ok("request is valid"));
        ok_response.set_http_response(ok_http_response);

        Ok(Response::new(ok_response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_port = env::var("SERVER_PORT").unwrap_or("50051".into());
    let addr = format!("0.0.0.0:{server_port}").parse().unwrap();
    let server = MyServer::default();

    println!("AuthorizationServer listening on {}", addr);

    Server::builder()
        .add_service(AuthorizationServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
