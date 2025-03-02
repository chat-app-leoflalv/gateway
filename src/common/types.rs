use crate::core::transport::Transport;

pub trait ServiceTransport: Transport + Clone + Send + Sync + 'static {}
impl<T: Transport + Clone + Send + Sync + 'static> ServiceTransport for T {}
