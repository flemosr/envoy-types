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

fn get_client_address(request: &CheckRequest) -> Option<&String> {
    let peer = request.attributes.as_ref()?.source.as_ref()?;
    let address = peer.address.as_ref()?.address.as_ref()?;
    if let Address::SocketAddress(socket_address) = address {
        return Some(&socket_address.address);
    }
    None
}

fn get_client_headers(request: &CheckRequest) -> Option<&HashMap<String, String>> {
    let request = request.attributes.as_ref()?.request.as_ref()?;
    Some(&request.http.as_ref()?.headers)
}

fn denied_response(status: Status) -> Result<Response<CheckResponse>, Status> {
    #[allow(deprecated)]
    let denied_http_response = DeniedHttpResponse {
        status: Some(HttpStatus {
            code: HttpStatusCode::Forbidden.into(),
        }),
        headers: vec![HeaderValueOption {
            header: Some(HeaderValue {
                key: "downstream-header".into(),
                value: "downstream-header-value".into(),
            }),
            append: None, // Deprecated field
            append_action: HeaderAppendAction::AppendIfExistsOrAdd.into(),
            keep_empty_value: false,
        }],
        body: "FORBIDDEN".into(),
    };

    let denied_response = CheckResponse {
        status: Some(rpc::Status {
            code: status.code().into(),
            message: status.message().into(),
            details: Vec::new(),
        }),
        dynamic_metadata: None,
        http_response: Some(HttpResponse::DeniedResponse(denied_http_response)),
    };

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

        let client_address = get_client_address(&request)
            .ok_or_else(|| Status::invalid_argument("client address not provided by envoy"))?;

        // Validate `client_address`
        // ...
        // Possible to implement some basic rate-limiting

        let client_headers = get_client_headers(&request)
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

        #[allow(deprecated)]
        let ok_http_response = OkHttpResponse {
            headers: vec![HeaderValueOption {
                header: Some(HeaderValue {
                    key: "extauthz-header".into(),
                    value: "extauthz-value".into(),
                }),
                append: None, // Deprecated field
                append_action: HeaderAppendAction::AppendIfExistsOrAdd.into(),
                keep_empty_value: false,
            }],
            headers_to_remove: Vec::new(),
            dynamic_metadata: None, // Deprecated field
            response_headers_to_add: Vec::new(),
            query_parameters_to_remove: Vec::new(),
            query_parameters_to_set: vec![QueryParameter {
                key: "extauthz-query-param".into(),
                value: "extauthz-query-value".into(),
            }],
        };

        let ok_response = CheckResponse {
            status: Some(rpc::Status {
                code: rpc::Code::Ok.into(),
                message: "request is valid".into(),
                details: Vec::new(),
            }),
            dynamic_metadata: None,
            http_response: Some(HttpResponse::OkResponse(ok_http_response)),
        };

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
