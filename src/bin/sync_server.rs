extern crate ridews;
use ridews::util;
use ridews::sync_server;
//use ridews::constants::LOG_DIR;
use ridews::constants::URL_WEBSOCKET_SERVER	;


fn main()  {
    util::logger_init()  ;

	
	sync_server::sync_server(URL_WEBSOCKET_SERVER);
	//socket::websocket_async_server(URL_WEBSOCKET_SERVER);
}


