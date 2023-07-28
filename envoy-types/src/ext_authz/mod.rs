pub mod v3 {
    /// Re-exporting selected pbs for more convenient imports when implementing
    /// an ExtAuthz server.
    pub mod pb {
        pub use crate::pb::envoy::config::core::v3::{
            address::Address, header_value_option::HeaderAppendAction, HeaderValue,
            HeaderValueOption, QueryParameter,
        };
        pub use crate::pb::envoy::r#type::v3::{HttpStatus, StatusCode as HttpStatusCode};
        pub use crate::pb::envoy::service::auth::v3::{
            authorization_server::{Authorization, AuthorizationServer},
            check_response::HttpResponse,
            CheckRequest, CheckResponse, DeniedHttpResponse, OkHttpResponse,
        };
    }

    use pb::{
        Address, CheckRequest, CheckResponse, DeniedHttpResponse, HeaderAppendAction, HeaderValue,
        HeaderValueOption, HttpResponse, HttpStatus, HttpStatusCode, OkHttpResponse,
        QueryParameter,
    };
    use std::collections::HashMap;

    use crate::pb::google::{protobuf::Struct, rpc};

    impl crate::sealed::Sealed for CheckRequest {}
    impl crate::sealed::Sealed for CheckResponse {}
    impl crate::sealed::Sealed for OkHttpResponse {}
    impl crate::sealed::Sealed for DeniedHttpResponse {}
    impl crate::sealed::Sealed for HttpResponse {}

    pub trait CheckRequestExt: crate::sealed::Sealed {
        fn get_client_address(&self) -> Option<&String>;

        fn get_client_headers(&self) -> Option<&HashMap<String, String>>;
    }

    impl CheckRequestExt for CheckRequest {
        fn get_client_address(&self) -> Option<&String> {
            let peer = self.attributes.as_ref()?.source.as_ref()?;
            let address = peer.address.as_ref()?.address.as_ref()?;
            if let Address::SocketAddress(socket_address) = address {
                return Some(&socket_address.address);
            }
            None
        }

        fn get_client_headers(&self) -> Option<&HashMap<String, String>> {
            let request = self.attributes.as_ref()?.request.as_ref()?;
            Some(&request.http.as_ref()?.headers)
        }
    }

    pub trait ToHttpResponse: crate::sealed::Sealed {
        fn to_http_response(self) -> HttpResponse;
    }

    impl ToHttpResponse for OkHttpResponse {
        fn to_http_response(self) -> HttpResponse {
            HttpResponse::OkResponse(self)
        }
    }

    impl ToHttpResponse for DeniedHttpResponse {
        fn to_http_response(self) -> HttpResponse {
            HttpResponse::DeniedResponse(self)
        }
    }

    impl ToHttpResponse for HttpResponse {
        fn to_http_response(self) -> HttpResponse {
            self
        }
    }

    pub trait CheckResponseExt: crate::sealed::Sealed {
        fn new() -> Self;

        fn with_status(status: tonic::Status) -> Self;

        fn set_status(&mut self, status: tonic::Status) -> &mut Self;

        fn set_dynamic_metadata(&mut self, dynamic_metadata: Option<Struct>) -> &mut Self;

        fn set_http_response(&mut self, http_response: impl ToHttpResponse) -> &mut Self;
    }

    impl CheckResponseExt for CheckResponse {
        fn new() -> Self {
            CheckResponse::default()
        }

        fn with_status(status: tonic::Status) -> Self {
            let mut check_response = CheckResponse::default();
            check_response.status = Some(rpc::Status {
                code: status.code().into(),
                message: status.message().into(),
                // TODO
                details: Vec::new(),
            });
            check_response
        }

        fn set_status(&mut self, status: tonic::Status) -> &mut Self {
            self.status = Some(rpc::Status {
                code: status.code().into(),
                message: status.message().into(),
                // TODO
                details: Vec::new(),
            });
            self
        }

        fn set_dynamic_metadata(&mut self, dynamic_metadata: Option<Struct>) -> &mut Self {
            self.dynamic_metadata = dynamic_metadata;
            self
        }

        fn set_http_response(&mut self, http_response: impl ToHttpResponse) -> &mut Self {
            self.http_response = Some(http_response.to_http_response());
            self
        }
    }

    fn push_header(
        headers: &mut Vec<HeaderValueOption>,
        key: impl Into<String>,
        value: impl Into<String>,
        append_action: Option<HeaderAppendAction>,
        keep_empty_value: bool,
    ) {
        #[allow(deprecated)]
        headers.push(HeaderValueOption {
            header: Some(HeaderValue {
                key: key.into(),
                value: value.into(),
            }),
            append: None, // Deprecated field
            append_action: append_action
                .unwrap_or(HeaderAppendAction::AppendIfExistsOrAdd)
                .into(),
            keep_empty_value,
        });
    }

    #[derive(Debug, Default)]
    pub struct OkHttpResponseBuilder {
        headers: Vec<HeaderValueOption>,
        headers_to_remove: Vec<String>,
        response_headers_to_add: Vec<HeaderValueOption>,
        query_parameters_to_remove: Vec<String>,
        query_parameters_to_set: Vec<QueryParameter>,
    }

    impl OkHttpResponseBuilder {
        pub fn new() -> Self {
            OkHttpResponseBuilder::default()
        }

        pub fn add_header(
            &mut self,
            key: impl Into<String>,
            value: impl Into<String>,
            append_action: Option<HeaderAppendAction>,
            keep_empty_value: bool,
        ) -> &mut Self {
            push_header(
                &mut self.headers,
                key,
                value,
                append_action,
                keep_empty_value,
            );
            self
        }

        pub fn remove_header(&mut self, header: impl Into<String>) -> &mut Self {
            self.headers_to_remove.push(header.into());
            self
        }

        pub fn add_response_header(
            &mut self,
            key: impl Into<String>,
            value: impl Into<String>,
            append_action: Option<HeaderAppendAction>,
            keep_empty_value: bool,
        ) -> &mut Self {
            push_header(
                &mut self.response_headers_to_add,
                key,
                value,
                append_action,
                keep_empty_value,
            );
            self
        }

        pub fn remove_query_parameter(&mut self, parameter: impl Into<String>) -> &mut Self {
            self.query_parameters_to_remove.push(parameter.into());
            self
        }

        pub fn set_query_parameter(
            &mut self,
            key: impl Into<String>,
            value: impl Into<String>,
        ) -> &mut Self {
            self.query_parameters_to_set.push(QueryParameter {
                key: key.into(),
                value: value.into(),
            });
            self
        }

        pub fn build(self) -> OkHttpResponse {
            #[allow(deprecated)]
            OkHttpResponse {
                headers: self.headers,
                headers_to_remove: self.headers_to_remove,
                dynamic_metadata: None, // Deprecated field
                response_headers_to_add: self.response_headers_to_add,
                query_parameters_to_remove: self.query_parameters_to_remove,
                query_parameters_to_set: self.query_parameters_to_set,
            }
        }
    }

    impl Into<OkHttpResponse> for OkHttpResponseBuilder {
        fn into(self) -> OkHttpResponse {
            self.build()
        }
    }

    impl crate::sealed::Sealed for OkHttpResponseBuilder {}

    impl ToHttpResponse for OkHttpResponseBuilder {
        fn to_http_response(self) -> HttpResponse {
            self.build().to_http_response()
        }
    }

    #[derive(Debug, Default)]
    pub struct DeniedHttpResponseBuilder {
        status: Option<HttpStatus>,
        headers: Vec<HeaderValueOption>,
        body: String,
    }

    impl DeniedHttpResponseBuilder {
        pub fn new() -> Self {
            DeniedHttpResponseBuilder::default()
        }

        pub fn set_http_status(&mut self, http_status_code: HttpStatusCode) -> &mut Self {
            self.status = Some(HttpStatus {
                code: http_status_code.into(),
            });
            self
        }

        pub fn add_header(
            &mut self,
            key: impl Into<String>,
            value: impl Into<String>,
            append_action: Option<HeaderAppendAction>,
            keep_empty_value: bool,
        ) -> &mut Self {
            push_header(
                &mut self.headers,
                key,
                value,
                append_action,
                keep_empty_value,
            );
            self
        }

        pub fn add_body(&mut self, body: impl Into<String>) -> &mut Self {
            self.body = body.into();
            self
        }

        pub fn build(self) -> DeniedHttpResponse {
            DeniedHttpResponse {
                status: self.status,
                headers: self.headers,
                body: self.body,
            }
        }
    }

    impl Into<DeniedHttpResponse> for DeniedHttpResponseBuilder {
        fn into(self) -> DeniedHttpResponse {
            self.build()
        }
    }

    impl crate::sealed::Sealed for DeniedHttpResponseBuilder {}

    impl ToHttpResponse for DeniedHttpResponseBuilder {
        fn to_http_response(self) -> HttpResponse {
            self.build().to_http_response()
        }
    }
}
