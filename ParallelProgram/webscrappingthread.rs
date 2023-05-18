/*
Web Scrapping
 */

use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use ureq::{Agent, AgentBuilder}; //2.5.0버전으로 씀...?

fn main() -> Result<(), ureq::Error>{
    let webpages = vec![
        "http://www.mediatoday.co.kr/news/articleView.html?idxno=310215",
        "http://www.mediatoday.co.kr/news/articleView.html?idxno=310212",
        "http://www.daejonilbo.com/news/articleView.html?idxno=2064722",
        "http://www.daejonilbo.com/news/articleView.html?idxno=2064720",
        "http://www.daejonilbo.com/news/articleView.html?idxno=2064801",
        
    ];

    let agent = ureq::AgentBuilder::new().build();
    let now = Instant::now();

    for web_page in &webpages{
        //? 함수 결과가 제대로 나오면 함수 결과를 그렇지 않으면 아무 결과도 띄우지 않고 무시함: 
        let web_body = agent.get(web_page).call()?.into_string()?;
    }
    println!("스레드 없이 걸린 시간: {:.2?}", now.elapsed());

    let now = Instant::now();
    let agent = Arc::new(agent);
    let mut handles = Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    for web_page in webpages{
        let agent_thread = agent.clone();
        let t = thread::spawn(move||{
            let web_body = agent_thread
            .get(web_page).call()?.into_string()?;
            Ok(())
        });

        handles.push(t);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("스레드로 걸린 시간: {:.2?}", now.elapsed());
    Ok(())
}