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

    /// Extension trait aiming to provide convenient methods to get useful
    /// data from [`pb::CheckRequest`].
    ///
    /// This trait is sealed and not meant to be implemented outside of
    /// `envoy-types`.
    pub trait CheckRequestExt: crate::sealed::Sealed {
        /// Returns a reference to the [`pb::CheckRequest`]'s source peer
        /// [`pb::Address::SocketAddress`] inner value, if any.
        ///
        /// In cases where Envoy is acting as an Edge Proxy, this should match
        /// the client's IP.
        fn get_client_address(&self) -> Option<&String>;

        /// Returns a reference to the inner (client) request's headers.
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

    /// Simple trait used to convert local and foreign types to
    /// [`pb::HttpResponse`]. Implemented for [`pb::OkHttpResponse`],
    /// [`pb::DeniedHttpResponse`], [`pb::HttpResponse`],
    /// [`OkHttpResponseBuilder`] and [`DeniedHttpResponseBuilder`].
    ///
    /// This trait is sealed and not meant to be implemented outside of
    /// `envoy-types`.
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

    /// Extension trait aiming to provide convenient associated fn's and methods
    /// to create and edit [`pb::CheckResponse`].
    ///
    /// This trait is sealed and not meant to be implemented outside of
    /// `envoy-types`.
    pub trait CheckResponseExt: crate::sealed::Sealed {
        /// Create a new, empty [`pb::CheckResponse`].
        ///
        /// Please note that if you return an empty [`pb::CheckResponse`], the
        /// request will be denied, since there will not be an inner `Ok`
        /// [`rpc::Status`].
        fn new() -> Self;

        /// Create a new [`pb::CheckResponse`] with the [`rpc::Status`]'s
        /// `code` and `message` matching those of the `tonic::Status` provided.
        ///
        /// Please note that `tonic::Status`'s `details` will not be considered.
        fn with_status(status: tonic::Status) -> Self;

        /// Set the [`pb::CheckResponse`] inner [`rpc::Status`]'s `code` and
        /// `message` as those of the `tonic::Status` provided.
        ///
        /// Please note that `tonic::Status`'s `details` will not be considered.
        fn set_status(&mut self, status: tonic::Status) -> &mut Self;

        /// Set the [`pb::CheckResponse`]'s `dynamic_metadata` field.
        fn set_dynamic_metadata(&mut self, dynamic_metadata: Option<Struct>) -> &mut Self;

        /// Set the [`pb::CheckResponse`]'s `http_response` field.
        ///
        /// Compatible with [`OkHttpResponseBuilder`],
        /// [`DeniedHttpResponseBuilder`], or even [`pb::OkHttpResponse`],
        /// [`pb::DeniedHttpResponse`] and [`pb::HttpResponse`].
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
                // `tonic::Status`'s details are not considered
                details: Vec::new(),
            });
            check_response
        }

        fn set_status(&mut self, status: tonic::Status) -> &mut Self {
            self.status = Some(rpc::Status {
                code: status.code().into(),
                message: status.message().into(),
                // `tonic::Status`'s details are not considered
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

    /// Provides convenient associated fn's and methods used to build a
    /// [`pb::OkHttpResponse`], containing HTTP attributes for an OK response.
    #[derive(Debug, Default)]
    pub struct OkHttpResponseBuilder {
        headers: Vec<HeaderValueOption>,
        headers_to_remove: Vec<String>,
        response_headers_to_add: Vec<HeaderValueOption>,
        query_parameters_to_remove: Vec<String>,
        query_parameters_to_set: Vec<QueryParameter>,
    }

    impl OkHttpResponseBuilder {
        /// Creates a new, empty [`OkHttpResponseBuilder`].
        pub fn new() -> Self {
            OkHttpResponseBuilder::default()
        }

        /// Add, overwrite or append a HTTP header to the original request
        /// before dispatching it upstream.
        ///
        /// The `append_action` field describes what action should be taken to
        /// append/overwrite the given value for an existing header, or to only
        /// add this header if it is not already present. Defaults to
        /// [`pb::HeaderAppendAction::AppendIfExistsOrAdd`] if set as `None`.
        ///
        /// If `keep_empty_value` is set as `false`, custom headers with empty
        /// values will be dropped. If set to `true`, they will be added.
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

        /// Remove a HTTP header from the original request before dispatching
        /// it upstream.
        ///
        /// Useful to consume headers related to auth that should not be exposed
        /// to downstream services.
        ///
        /// Envoy's pseudo headers (such as `:authority`, `:method`, `:path`
        /// etc), as well as the header `Host`, will not be removed, since this
        /// would make the request malformed.
        pub fn remove_header(&mut self, header: impl Into<String>) -> &mut Self {
            self.headers_to_remove.push(header.into());
            self
        }

        /// Add a HTTP response header that will be sent to the downstream
        /// client, in case of success.
        ///
        /// The `append_action` field describes what action should be taken to
        /// append/overwrite the given value for an existing header, or to only
        /// add this header if it is not already present. Defaults to
        /// [`pb::HeaderAppendAction::AppendIfExistsOrAdd`] if set as `None`.
        ///
        /// If `keep_empty_value` is set as `false`, custom headers with empty
        /// values will be dropped. If set to `true`, they will be added.
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

        /// Remove a query parameter from the original request before
        /// dispatching it upstream.
        ///
        /// Please note that the parameter `key` is case-sensitive.
        pub fn remove_query_parameter(&mut self, key: impl Into<String>) -> &mut Self {
            self.query_parameters_to_remove.push(key.into());
            self
        }

        /// Add or overwrite a query parameter of the original request before
        /// dispatching it upstream.
        ///
        /// Please note that the parameter `key` is case-sensitive.
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

        /// Build a [`pb::OkHttpResponse`], consuming the
        /// [`OkHttpResponseBuilder`].
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

    /// Provides convenient associated fn's and methods used to build a
    /// [`pb::DeniedHttpResponse`], containing HTTP attributes for a
    /// denied response.
    #[derive(Debug, Default)]
    pub struct DeniedHttpResponseBuilder {
        status: Option<HttpStatus>,
        headers: Vec<HeaderValueOption>,
        body: String,
    }

    impl DeniedHttpResponseBuilder {
        /// Creates a new, empty [`DeniedHttpResponseBuilder`].
        pub fn new() -> Self {
            DeniedHttpResponseBuilder::default()
        }

        /// Set the HTTP response status code that will be sent to the
        /// downstream client.
        ///
        /// If not set, Envoy will send a `403 Forbidden` HTTP status code.
        pub fn set_http_status(&mut self, http_status_code: HttpStatusCode) -> &mut Self {
            self.status = Some(HttpStatus {
                code: http_status_code.into(),
            });
            self
        }

        /// Add a HTTP response header that will be sent to the downstream
        /// client.
        ///
        /// The `append_action` field describes what action should be taken to
        /// append/overwrite the given value for an existing header, or to only
        /// add this header if it is not already present. Defaults to
        /// [`pb::HeaderAppendAction::AppendIfExistsOrAdd`] if set as `None`.
        ///
        /// If `keep_empty_value` is set as `false`, custom headers with empty
        /// values will be dropped. If set to `true`, they will be added.
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

        /// Set the HTTP response body that will be sent to the downstream
        /// client.
        pub fn set_body(&mut self, body: impl Into<String>) -> &mut Self {
            self.body = body.into();
            self
        }

        /// Build a [`pb::DeniedHttpResponse`], consuming the
        /// [`DeniedHttpResponseBuilder`].
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
