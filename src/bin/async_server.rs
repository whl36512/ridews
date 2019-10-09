extern crate ridews;
use ridews::util;
use ridews::socket;
//use ridews::constants::LOG_DIR;
use ridews::constants::URL_WEBSOCKET_SERVER	;


fn main()  {
    util::logger_init()  ;

/*
	struct AscynServer {};
	impl socket::WebsockerServer for AscynServer {
		process_msg(msg: Text) -> Option<Text> {
			Some(msg);
		}
	}
	
	let async_server= AscynServer {};
	async_server.websocket_async_server(URL_WEBSOCKET_SERVER);
*/
	socket::websocket_async_server(URL_WEBSOCKET_SERVER);
}


