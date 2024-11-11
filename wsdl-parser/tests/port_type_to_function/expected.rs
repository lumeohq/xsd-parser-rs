
#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tds", namespaces = {"tds" = "http://www.onvif.org/ver10/device/wsdl"})]
pub struct GetServices {
    // Indicates if the service capabilities (untyped) should be included in the
    // response.
    #[yaserde(prefix = "tds", rename = "IncludeCapability")]
    pub include_capability: bool,
}

impl Validate for GetServices {}


#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tds", namespaces = {"tds" = "http://www.onvif.org/ver10/device/wsdl"})]
pub struct GetServicesResponse {
    // Each Service element contains information about one service.
    #[yaserde(prefix = "tds", rename = "Service")]
    pub service: Vec<Service>,
}

impl Validate for GetServicesResponse {}

// Returns information about services on the device.
pub async fn get_services<T: transport::Transport>(
    transport: &T,
    request: &GetServices
) -> Result<GetServicesResponse, transport::Error> {
    transport::request(transport, request).await
}