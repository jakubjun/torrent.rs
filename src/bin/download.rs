use futures::io;
use rand::Rng;
use tokio::net::{lookup_host, UdpSocket};

type TransactionId = u32;
type ConnectionId = u64;
type Action = u32;
type Buffer = [u8; 1024];
type ConnectRequest = [u8; 16];
type AnnounceRequest = [u8; 98];

#[repr(u32)]
enum TrackerAction {
    Connect = 0,
    Announce = 1,
}

impl TrackerAction {
    fn from_u32(value: Action) -> TrackerAction {
        match value {
            0 => TrackerAction::Connect,
            1 => TrackerAction::Announce,
            _ => panic!(),
        }
    }
}

const MAGIC_CONSTANT: u64 = 0x41727101980;

fn make_connect_request() -> (ConnectRequest, TransactionId) {
    let mut rng = rand::thread_rng();
    let mut res: ConnectRequest = [0x00; 16];
    let trans_id: u32 = rng.gen();
    res[0..8].copy_from_slice(&MAGIC_CONSTANT.to_be_bytes());
    res[8..12].copy_from_slice(&(TrackerAction::Connect as u32).to_be_bytes());
    res[12..16].copy_from_slice(&trans_id.to_be_bytes());
    (res, trans_id)
}

fn parse_connect_response(buf: &Buffer) -> (TrackerAction, TransactionId, ConnectionId) {
    let received_action_bytes = <[u8; 4]>::try_from(&buf[0..4]).unwrap();
    let received_action_id = u32::from_be_bytes(received_action_bytes);

    let received_trans_id_bytes = <[u8; 4]>::try_from(&buf[4..8]).unwrap();
    let received_trans_id = u32::from_be_bytes(received_trans_id_bytes);

    let received_conn_bytes = <[u8; 8]>::try_from(&buf[8..16]).unwrap();
    let received_connection_id = u64::from_be_bytes(received_conn_bytes);

    (
        TrackerAction::from_u32(received_action_id),
        received_trans_id,
        received_connection_id,
    )
}

fn make_announce_request(
    connection_id: &ConnectionId,
    transaction_id: &TransactionId,
    info_hash: String,
    downloaded: u64,
    left: u64,
    uploaded: u64,
    event: u32,
    ip: u32,
    num_want: u32,
    key: u32,
    port: u16,
) -> AnnounceRequest {
    let mut res: AnnounceRequest = [0x00; 98];
    res[0..8].copy_from_slice(&connection_id.to_be_bytes());
    res[8..12].copy_from_slice(&(TrackerAction::Announce as u32).to_be_bytes());
    res[12..16].copy_from_slice(&transaction_id.to_be_bytes());
    res[16..36].copy_from_slice(info_hash.as_bytes());
    res[36..56].copy_from_slice("lorem ipsum dolor si".as_bytes());
    res[56..64].copy_from_slice(&downloaded.to_be_bytes());
    res[64..72].copy_from_slice(&left.to_be_bytes());
    res[72..80].copy_from_slice(&uploaded.to_be_bytes());
    res[80..84].copy_from_slice(&event.to_be_bytes());
    res[84..88].copy_from_slice(&ip.to_be_bytes());
    res[88..92].copy_from_slice(&key.to_be_bytes());
    res[92..96].copy_from_slice(&num_want.to_be_bytes());
    res[96..98].copy_from_slice(&port.to_be_bytes());

    res
}

fn parse_announce_response(buf: &Buffer) {}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let sock = UdpSocket::bind("0.0.0.0:34567").await?;

    let mut ip = lookup_host("open.stealth.si:80").await?;
    let remote_addr = ip.next().unwrap();

    sock.connect(remote_addr).await?;
    println!("connected");

    let mut buf: Buffer = [0; 1024];
    let (payload, trans_id) = make_connect_request();

    let len = sock.send(&payload).await?;

    let len = sock.recv(&mut buf).await?;

    let (received_action, received_trans_id, received_connection_id) = parse_connect_response(&buf);

    let left ;
    let info_hash;
    let announce_request = make_announce_request(received_connection_id, trans_id, info_hash, 0, left, 0, 0, 0, -1, key, port)

    dbg!(received_connection_id);
    Ok(())
}
