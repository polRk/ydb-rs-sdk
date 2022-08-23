use crate::client::TimeoutSettings;
use crate::grpc_wrapper::raw_errors::RawResult;
use crate::grpc_wrapper::raw_services::{GrpcServiceForDiscovery, Service};
use crate::grpc_wrapper::raw_table_service::create_session::{
    RawCreateSessionRequest, RawCreateSessionResult,
};
use crate::grpc_wrapper::raw_ydb_operation::RawOperationParams;
use crate::grpc_wrapper::runtime_interceptors::InterceptedChannel;
use tracing::trace;
use ydb_grpc::ydb_proto::table::v1::table_service_client::TableServiceClient;

pub(crate) struct RawTableClient {
    operation_timeout: std::time::Duration,
    service: TableServiceClient<InterceptedChannel>,
}

impl RawTableClient {
    pub fn new(service: InterceptedChannel) -> Self {
        Self {
            service: TableServiceClient::new(service),
            operation_timeout: TimeoutSettings::default().operation_timeout,
        }
    }

    pub fn with_timeout(mut self, operation_timeout: std::time::Duration) -> Self {
        self.operation_timeout = operation_timeout;
        self
    }

    pub async fn create_session(&mut self) -> RawResult<RawCreateSessionResult> {
        let req = RawCreateSessionRequest {
            operation_params: RawOperationParams::new_with_timeout(self.operation_timeout),
        };

        request_with_result!(
            self.service.create_session,
            req => ydb_grpc::ydb_proto::table::CreateSessionRequest,
            ydb_grpc::ydb_proto::table::CreateSessionResult => RawCreateSessionResult
        );
    }
}

impl GrpcServiceForDiscovery for RawTableClient {
    fn get_grpc_discovery_service() -> Service {
        Service::Table
    }
}
