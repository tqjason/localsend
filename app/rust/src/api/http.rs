use crate::api::stream;
use flutter_rust_bridge::frb;
pub use localsend::http::client::ClientError;
pub use localsend::http::client::LsHttpClientVersion;
pub use localsend::http::client::ResultWithPublicKey;
pub use localsend::http::dto::PrepareUploadRequestDto;
pub use localsend::http::dto::PrepareUploadResponseDto;
pub use localsend::http::dto::ProtocolType;
pub use localsend::http::dto::RegisterDto;
pub use localsend::http::dto::RegisterResponseDto;

pub struct RsHttpClient {
    inner: localsend::http::client::LsHttpClient,
}

pub fn create_client(
    private_key: String,
    cert: String,
    version: LsHttpClientVersion,
) -> Result<RsHttpClient, ClientError> {
    let inner = localsend::http::client::LsHttpClient::new(&private_key, &cert, version)?;

    Ok(RsHttpClient { inner })
}

impl RsHttpClient {
    pub async fn register(
        &self,
        protocol: ProtocolType,
        ip: &str,
        port: u16,
        payload: RegisterDto,
    ) -> Result<ResultWithPublicKeyRegisterResponseDto, ClientError> {
        let response = self.inner.register(protocol, ip, port, payload).await?;

        Ok(ResultWithPublicKeyRegisterResponseDto {
            public_key: response.public_key,
            body: response.body,
        })
    }

    pub async fn prepare_upload(
        &self,
        protocol: ProtocolType,
        ip: &str,
        port: u16,
        payload: PrepareUploadRequestDto,
        pin: Option<String>,
    ) -> Result<ResultWithPublicKeyPrepareUploadResponseDto, ClientError> {
        let response = self
            .inner
            .prepare_upload(protocol, ip, port, payload, pin.as_deref())
            .await?;

        Ok(ResultWithPublicKeyPrepareUploadResponseDto {
            public_key: response.public_key,
            body: response.body,
        })
    }

    pub async fn upload(
        &self,
        protocol: ProtocolType,
        ip: &str,
        port: u16,
        session_id: &str,
        file_id: &str,
        token: &str,
        binary: stream::Dart2RustStreamReceiver,
    ) -> Result<(), ClientError> {
        self.inner
            .upload(
                protocol,
                ip,
                port,
                session_id,
                file_id,
                token,
                binary.receiver,
            )
            .await?;

        Ok(())
    }

    pub async fn cancel(
        &self,
        protocol: ProtocolType,
        ip: &str,
        port: u16,
        session_id: &str,
    ) -> Result<(), ClientError> {
        self.inner.cancel(protocol, ip, port, session_id).await?;

        Ok(())
    }
}

#[frb(mirror(LsHttpClientVersion))]
pub enum _LsHttpClientVersion {
    V2,
    V3,
}

pub struct ResultWithPublicKeyRegisterResponseDto {
    pub public_key: Option<String>,
    pub body: RegisterResponseDto,
}

pub struct ResultWithPublicKeyPrepareUploadResponseDto {
    pub public_key: Option<String>,
    pub body: PrepareUploadResponseDto,
}
