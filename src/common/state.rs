use crate::core::transport::Transport;

#[derive(Clone, Debug)]
pub struct GatewayState<T>
where
    T: Transport,
{
    pub client: T,
}
