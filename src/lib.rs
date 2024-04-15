use http::{httprequest::HttpRequest, httpresponse::HttpResponse, Http};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{io::BufReader, net::TcpListener};
mod http;
pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let (rd, mut wt) = stream.into_split();

        tokio::spawn(async move {
            let mut vec = Vec::new();

            BufReader::new(rd).read_buf(&mut vec).await.unwrap();

            let foruser = Http::new(
                HttpRequest::build(String::from_utf8_lossy(&vec).to_string()).await,
                HttpResponse::default(),
            )
            .processor()
            .await;

            wt.write_all(foruser.as_bytes()).await.unwrap()
        })
        .await
        .unwrap()
    }
}
