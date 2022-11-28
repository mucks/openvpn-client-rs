#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Undetermined,
}

impl From<String> for Protocol {
    fn from(s: String) -> Self {
        match s.as_str() {
            "TCP" => Self::Tcp,
            "UDP" => Self::Udp,
            _ => Self::Undetermined,
        }
    }
}
