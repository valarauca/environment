use std::net::{IpAddr, SocketAddr};

pub fn parse_socket<S: AsRef<str>>(arg: &S) -> Option<SocketAddr> {
    <SocketAddr as ::std::str::FromStr>::from_str(arg.as_ref()).ok()
}
pub fn parse_ip<S: AsRef<str>>(arg: &S) -> Option<IpAddr> {
    <IpAddr as ::std::str::FromStr>::from_str(arg.as_ref()).ok()
}
