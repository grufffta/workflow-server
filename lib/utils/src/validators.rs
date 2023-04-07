use std::{net::IpAddr, ops::RangeInclusive};

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

/// validates the port is withing the range of valid TCP ports
pub fn is_port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a port number"))?;
    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    }
    Err(format!(
        "'{s}' port no in range {}-{}",
        PORT_RANGE.start(),
        PORT_RANGE.end()
    ))
}

/// validates the string can be parsed to a valid IP address
pub fn is_valid_ip_address(s: &str) -> Result<IpAddr, String> {
    let ip: IpAddr = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid ip address"))?;
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_valid_port() {
        let result = is_port_in_range("8080");
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn it_errors_on_invalid_port() {
        let result = is_port_in_range("99999");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn it_parses_valid_ip_address() {
        let result = is_valid_ip_address("127.0.0.1");
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn it_errors_on_invalid_ip_address() {
        let result = is_valid_ip_address("301.241.257.123");
        assert_eq!(result.is_ok(), false);
    }
}
