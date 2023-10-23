use core::str::FromStr;

pub const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[derive(Copy, Clone)]
pub struct Ipv4Addr(pub u32);

pub struct SocketAddrV4 {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        let v = u32::from_be_bytes([a, b, c, d]);
        Self(v)
    }
}

impl FromStr for Ipv4Addr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 15 {
            return Err("too long");
        }
        if s == "localhost" {
            return Ok(LOCALHOST);
        }

        let mut iter = s.splitn(4, '.').map(|v| v.parse::<u8>());

        let buf = match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c)), Some(Ok(d))) => [a, b, c, d],
            _ => return Err("bad address"),
        };

        let out = Self(u32::from_be_bytes(buf));

        Ok(out)
    }
}

impl FromStr for SocketAddrV4 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, ':');

        let (host, port) = match (iter.next(), iter.next()) {
            (Some(_), Some("") | None) => return Err("miss port"),
            (Some(a), Some(b)) => (a, b),
            _ => return Err("miss ip and port"),
        };

        let ip = host.parse().map_err(|_| "parse ip")?;
        let port = u16::from_str(port).map_err(|_| "parse port")?;

        let out = Self { ip, port };
        Ok(out)
    }
}
