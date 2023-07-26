use std::collections::HashMap;
use std::env;

use tonic::{transport::Server, Request, Response, Status};

use envoy_types::ext_authz::v3::pb::{
    Address, Authorization, AuthorizationServer, CheckRequest, CheckResponse, DeniedHttpResponse,
    HeaderAppendAction, HeaderValue, HeaderValueOption, HttpResponse, HttpStatus, HttpStatusCode,
    OkHttpResponse, QueryParameter,
};
use envoy_types::pb::google::rpc;

#[derive(Default)]
struct MyServer;

fn get_external_request_data(
    request: Request<CheckRequest>,
) -> Option<(String, HashMap<String, String>)> {
    let attributes = request.into_inner().attributes?;

    let client_address = match attributes.source?.address?.address? {
        Address::SocketAddress(socket_address) => socket_address.address,
        _ => return None,
    };
    let client_headers = attributes.request?.http?.headers;

    Some((client_address, client_headers))
}

#[tonic::async_trait]
impl Authorization for MyServer {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        println!("{:?}", request);

        let denied_http_response = DeniedHttpResponse {
            status: Some(HttpStatus {
                code: HttpStatusCode::Forbidden.into(),
            }),
            headers: Vec::new(),
            body: "FORBIDDEN".to_string(),
        };

        let mut http_response = HttpResponse::DeniedResponse(denied_http_response);

        if let Some((client_address, client_headers)) = get_external_request_data(request) {
            println!("\n{:?}", client_address);
            println!("{:?}", client_headers);

            if let Some(authorization) = client_headers.get("authorization") {
                println!("{:?}", authorization);

                if authorization == "Bearer valid-token" {
                    #[allow(deprecated)]
                    let ok_http_response = OkHttpResponse {
                        headers: vec![HeaderValueOption {
                            header: Some(HeaderValue {
                                key: "extauthz-header".to_string(),
                                value: "extauthz-value".to_string(),
                            }),
                            append: None, // Deprecated field
                            append_action: HeaderAppendAction::AddIfAbsent.into(),
                            keep_empty_value: false,
                        }],
                        headers_to_remove: Vec::new(),
                        dynamic_metadata: None, // Deprecated field
                        response_headers_to_add: Vec::new(),
                        query_parameters_to_remove: Vec::new(),
                        query_parameters_to_set: vec![QueryParameter {
                            key: "extauthz-query-param".to_string(),
                            value: "extauthz-query-value".to_string(),
                        }],
                    };

                    http_response = HttpResponse::OkResponse(ok_http_response);
                }
            }
        }

        let response_status = match http_response {
            HttpResponse::OkResponse(_) => rpc::Status {
                code: rpc::Code::Ok.into(),
                message: "request ok".to_string(),
                details: Vec::new(),
            },
            HttpResponse::DeniedResponse(_) => rpc::Status {
                code: rpc::Code::Unauthenticated.into(),
                message: "request denied".to_string(),
                details: Vec::new(),
            },
        };

        let response = CheckResponse {
            status: Some(response_status),
            dynamic_metadata: None,
            http_response: Some(http_response),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_port = env::var("SERVER_PORT").unwrap_or("8080".into());

    let addr = format!("0.0.0.0:{server_port}").parse().unwrap();
    let server = MyServer::default();

    println!("AuthorizationServer listening on {}", addr);

    Server::builder()
        .add_service(AuthorizationServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
