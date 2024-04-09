use http::{httprequest::HttpRequest, httpresponse::HttpResponse, Http};
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};
mod http;
mod process;
pub async fn run() {
    loop {
        tokio::spawn(async move {
            let (stream, _) = TcpListener::bind("127.0.0.1:7878")
                .await
                .unwrap()
                .accept()
                .await
                .unwrap();

            let (rd, mut wt) = stream.into_split();

            let mut s = String::new();

            BufReader::new(rd).read_line(&mut s).await.unwrap();

            let foruser = Http::new(HttpRequest::build(s).await, HttpResponse::default())
                .processer()
                .await;

            wt.write_all(foruser.as_bytes()).await.unwrap()
        })
        .await
        .unwrap()
    }
}
