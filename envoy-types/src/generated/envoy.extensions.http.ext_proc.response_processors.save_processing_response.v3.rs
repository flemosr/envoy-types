// This file is @generated by prost-build.
/// Extension to save the :ref:`response  <envoy_v3_api_msg_service.ext_proc.v3.ProcessingResponse>` from the external processor as
/// filter state with name
/// "envoy.http.ext_proc.response_processors.save_processing_response\[.:ref:`filter_state_name_suffix  <envoy_v3_api_field_extensions.http.ext_proc.response_processors.save_processing_response.v3.SaveProcessingResponse.filter_state_name>`\].
/// This extension supports saving of request and response headers and trailers,
/// and immediate response.
/// \[\#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveProcessingResponse {
    /// The default filter state name is
    /// "envoy.http.ext_proc.response_processors.save_processing_response".
    /// If defined, `filter_state_name_suffix` is appended to this.
    /// For example, setting `filter_state_name_suffix` to "xyz" will set the
    /// filter state name to "envoy.http.ext_proc.response_processors.save_processing_response.xyz"
    #[prost(string, tag = "1")]
    pub filter_state_name_suffix: ::prost::alloc::string::String,
    /// Save the response to filter state when :ref:`request_headers  <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.request_headers>` is set.
    #[prost(message, optional, tag = "2")]
    pub save_request_headers: ::core::option::Option<
        save_processing_response::SaveOptions,
    >,
    /// Save the response to filter state when :ref:`response_headers  <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.response_headers>` is set.
    #[prost(message, optional, tag = "3")]
    pub save_response_headers: ::core::option::Option<
        save_processing_response::SaveOptions,
    >,
    /// Save the response to filter state when :ref:`request_trailers  <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.request_trailers>` is set.
    #[prost(message, optional, tag = "4")]
    pub save_request_trailers: ::core::option::Option<
        save_processing_response::SaveOptions,
    >,
    /// Save the response to filter state when :ref:`response_trailers  <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.response_trailers>` is set.
    #[prost(message, optional, tag = "5")]
    pub save_response_trailers: ::core::option::Option<
        save_processing_response::SaveOptions,
    >,
    /// Save the response to filter state when :ref:`immediate_response  <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.immediate_response>` is set.
    #[prost(message, optional, tag = "6")]
    pub save_immediate_response: ::core::option::Option<
        save_processing_response::SaveOptions,
    >,
}
/// Nested message and enum types in `SaveProcessingResponse`.
pub mod save_processing_response {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SaveOptions {
        /// Whether or not to save the response for the response type.
        #[prost(bool, tag = "1")]
        pub save_response: bool,
        /// When true, saves the response if there was an error when processing
        /// the response from the external processor.
        #[prost(bool, tag = "2")]
        pub save_on_error: bool,
    }
}
