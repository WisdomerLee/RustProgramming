/*
web programming
Socket, Tcp, udp, ..

서버의 응답 구조, 형태
http-version status-code Reason-phrase CRLF
headers CRLF
message-body

프로젝트 내부에 html 문서를 생성
<!DOCTYPE html>
<html lang = "en">
<head>
<meta charset = "utf-8">
<title>Hello</title>
</head>
<body>
<h1>Hello!</h1>
<p>Hi from Rust programming Master Class </p>
</body>
</html>
//또한 프로젝트 내부에 서버의 응답이 실패, 등등의 경우를 모두 html로 문서로 만들고....


 */

use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude;
use std::fs;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    //let stream = listener.accept();

    println!("소켓의 스트림 {:?} \n 소켓 {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);
    // //10번 정도 클라이언트 정보를 띄우기
    // for i in 0..10 {
    //     match listener.accept() {
    //         Ok((socket, addr)) => println!("클라이언트 정보 {:?}", addr),
    //         Err(e) => println!("클라이언트 정보를 얻을 수 없습니다 {:?}", e),
    //     }
    // }

    let mut active_request = Arc::new(Mutex::new(0));
    //클라이언트는 서버로부터 응답을 받기를 바라므로 (성공이든 실패든) : 위의 경우 서버에서 응답 여부를 설정하지 않았으므로 한 번의 요청에 여러번의 시도가 진행됨...
    for stream in listener.incoming(){
        let active_request = Arc::clone(&active_request);
        let stream = stream.unwrap();
        thread::spawn(move ||{
            {
                let mut connection = active_request.lock().unwrap();
            *connection += 1;
                if *connection >= 3{
                    thread::sleep(Duration::from_secs(2));
                }
            }
            
            handle_connection(stream);
            {
                let mut connection = active_request.lock().unwrap();
                *connection -= 1;
            }
        });
        //handle_connection(stream);
    }
}
//이 경우는 동기적으로 반응하므로 하나의 응답에 반응하는 동안 서버가 정지함...? > 비동기 형태로 응답에 반응하게 하면 됨
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let http_request = buf_reader.lines()
    .map(|result| result.unwrap()) //결과의 데이터를 얻고
    .take_while(|lines| !lines.isempty()) //응답 결과가 빈 칸이 될때까지 계속 받기
    .collect::<Vec<String>>(); //해당 데이터를 문자열 벡터 형태로 받기
    println!("Request {:?}", http_request);
    // //서버의 응답이 OK외엔 아무것도 없으므로 홈페이지는 빈칸으로 출력됨
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // //아래와 같이 처리하면 서버를 향한 아무 응답에도 OK를 내주게 되어 모두 같은 화면을 보게 됨 >> 
    // //서버의 응답 메시지
    // let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    // //프로젝트 내부에 작성했던 html 문서 불러오기
    // let contents = fs::read_to_string("index.html").unwrap();
    // //
    // let length = contents.len();
    // let response = format!("{} 콘텐츠 길이: {}\r\n\r\n{}", status_line, length, contents);
    // stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    let mut request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n\r\n"), Some("index.html")),
        "GET / page1 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n\r\n"), Some("page1.html")),
        "GET / page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n\r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let response = format!("{} contents-length {} \r\n\r\n{}", status_line.unwrap(), contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}