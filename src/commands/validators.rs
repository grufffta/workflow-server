use std::net::IpAddr;
use std::ops::RangeInclusive;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

pub(crate) fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a port number"))?;
    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    }
    Err(format!("'{s}' port no in range {}-{}", PORT_RANGE.start(), PORT_RANGE.end()))
}

pub(crate) fn is_valid_ip_address(s: &str) -> Result<IpAddr, String> {
    let ip: IpAddr = s.parse().map_err(|_| format!("'{s}' is not a valid ip address"))?;
    Ok(ip)
}