use rand::random;

const PROTOCOL_ID: u64 = 0x41727101980;

#[repr(u32)]
enum Action {
    Connect = 0,
    Announce = 1,
}

enum TrackerProtocol {
    Http,
    Udp,
}

struct TrackerConnection {
    protocol: TrackerProtocol,
    location: String,
    port: Option<u32>,
    connection_id: Option<u64>,
}

impl TrackerConnection {
    fn new(url: String) {}

    fn connect(self) {
        let mut rng = rand::thread_rng();

        let transaction_id: u32 = random();
    }

    fn announce(self) {}
}

struct ConnectPayload {}

struct AnnouncePayload {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tracker_connect() {
        todo!();
    }
}
