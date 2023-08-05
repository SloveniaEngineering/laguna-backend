use actix_web::{web, HttpResponse};
use laguna_backend_dto::user::UserDTO;
use laguna_backend_tracker::http::announce::{AnnounceRequest, AnnounceResponse};
use laguna_backend_tracker::prelude::peer::{PeerDictStream, PeerStream};

/// GET `/api/peer/announce`
/// # Example
/// ### Request
/// **NOTE**: The -G allows GET to send data via query string. See: <https://stackoverflow.com/questions/13371284/curl-command-line-url-parameters>.
///
/// **FIXME**: This example doesn't work yet, URL encoding info_hash and peer_id is weird ASF.
///        Generally we need to send 20 bytes of info_hash and 20 bytes of peer_id via GET request. Curl might be an issue here.
///
/// ```bash
/// curl -X GET \
///      -G \
///      -i 'http://127.0.0.1:6969/api/peer/announce' \
///      -d 'info_hash=%5B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%2C13%2C14%2C15%2C16%2C17%2C18%2C19%2C20%5D' \
///      -d 'peer_id=%5B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%2C11%2C12%2C13%2C14%2C15%2C16%2C17%2C18%2C19%2C20%5D' \
///      -d 'ip=127.0.0.1' \
///      -d 'port=43222' \
///      -d 'uploaded=0' \
///      -d 'downloaded=0' \
///      -d 'left=0' \
///      -d 'event=started' \
///      -d 'numwant=10' \
///      -d 'compact=true' \
///      -d 'no_peer_id=true' \
///      -d 'key=some%20random%20key' \
///      -d 'trackerid=some%20prev%20tracker%20id' \
///      -H 'X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2OTEzMjIyODksImlhdCI6MTY5MTIzNTg4OSwiaWQiOiJjZjNhNTRlOC01MmUzLTQ3OTktOTNmNS1jMGM0NjE3ZTUxNjYiLCJ1c2VybmFtZSI6InRlc3R4eHgiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDgtMDVUMTE6Mjc6MjQuNDY5NDM4WiIsImxhc3RfbG9naW4iOiIyMDIzLTA4LTA1VDExOjQ0OjQ5LjAyNzI3MFoiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.TDELyQha28VDGd0L5trCi8hiTxLESCmHUnLE9l7h2W4' \
///      -H 'X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2OTEzMjIyODksImlhdCI6MTY5MTIzNTg4OSwiaWQiOiJjZjNhNTRlOC01MmUzLTQ3OTktOTNmNS1jMGM0NjE3ZTUxNjYiLCJ1c2VybmFtZSI6InRlc3R4eHgiLCJmaXJzdF9sb2dpbiI6IjIwMjMtMDgtMDVUMTE6Mjc6MjQuNDY5NDM4WiIsImxhc3RfbG9naW4iOiIyMDIzLTA4LTA1VDExOjQ0OjQ5LjAyNzI3MFoiLCJhdmF0YXJfdXJsIjpudWxsLCJyb2xlIjoiTm9ybWllIiwiYmVoYXZpb3VyIjoiTHVya2VyIiwiaXNfYWN0aXZlIjp0cnVlLCJoYXNfdmVyaWZpZWRfZW1haWwiOmZhbHNlLCJpc19oaXN0b3J5X3ByaXZhdGUiOnRydWUsImlzX3Byb2ZpbGVfcHJpdmF0ZSI6dHJ1ZX0.TDELyQha28VDGd0L5trCi8hiTxLESCmHUnLE9l7h2W4' \
///      -H 'Content-Type: text/plain'
/// ```
/// ### Response
/// ```text
/// ```
pub async fn peer_announce(
    announce_data: web::Query<AnnounceRequest>,
    _user: UserDTO,
) -> HttpResponse {
    println!("{:?}", announce_data);
    HttpResponse::Ok().body(
        serde_bencode::to_bytes(&AnnounceResponse {
            failure_reason: None,
            warning_message: None,
            interval: 0,
            min_interval: None,
            tracker_id: None,
            complete: 0,
            incomplete: 0,
            peers: PeerStream::Dict(PeerDictStream::new()),
        })
        .unwrap(),
    )
}
