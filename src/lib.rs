use constents::SERVER_ADDR;
use httprequest::HttpRequest;
use httpresponse::HttpResponse;
use route::router;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, join, net::TcpListener, sync::mpsc};
mod httpmethod;
mod httprequest;
mod httpresponse;
mod constents;
mod route;
pub async fn run() {
    let (tx,mut rx) = mpsc::channel(32);

    
    let (a,b) = join!(tokio::spawn(async move{
        loop {
            let (stream,_) = TcpListener::bind(SERVER_ADDR).await.unwrap().accept().await.unwrap();
            let (rd,wt) = stream.into_split();
            let mut reader = BufReader::new(rd);
            let mut s = String::new();
            reader.read_line(&mut s).await.unwrap();
            let request = HttpRequest::build(s).await;
            let mut response = HttpResponse::default();
            let result = router(request, &mut response).await;
            tx.send((result,wt)).await.unwrap()
        }
    })

    ,

    tokio::spawn(async move {
        loop {
            let (result,mut wt) = rx.recv().await.unwrap();
            wt.write_all(result.as_bytes()).await.unwrap()
        }
    }));
    a.unwrap();
    b.unwrap()
    
}
