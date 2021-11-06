mod server;
mod agent;
use std::env;
use std::process;

/// The main entry take at least 3 parameters: the base url, the port for this service, and the
/// attributes for identifying the divs that you want to remove from the browser.
///
/// The way that the program work is:
/// This program listens at the given port, and wait for the access from your browser. If it is
/// received from your browser, it will then send the request to the base url plus the uri that
/// is from your browser. The response from the destination will be filtered by remving some of the
/// \<div\> tag that you specified in the command line. Multiple div properties could be applied,
/// thus you can remove multiple \<div\> tags. For example:
/// ```
/// dyn-proxy 'https://www.merriam-webster.com' 3000 'id="definition-right-rail"' 'class="border-box mobile-fixed-ad"' 'class="abl mw-ad-slot-top"'
/// ```
/// 
/// If you open your browser and access `http://localhost:3000/dictionary/time` you will not see
/// the advertisements.
///
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{} URL PORT DIV-PROP{{1..n}}", args[0]);
        process::exit(1);
    }
    let url = args[1].clone();
    let port = args[2].parse::<u16>().unwrap();
    let div_props = &args[3..];
    server::make_server(agent::AgentService::new("proxy-server".to_owned(), url, port, div_props.into())).await;
}

