use once_cell::sync::Lazy;
use crate::connection_info::ConnectionInfo;

pub(crate) static CONNECTION_INFO: Lazy<ConnectionInfo> = Lazy::new(||
    ConnectionInfo::parse(std::env::var("YDB_CONNECTION_STRING").unwrap().as_str()).unwrap()
);
