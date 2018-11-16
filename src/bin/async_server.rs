extern crate ridews;
use ridews::util;
use ridews::socket;
use ridews::constants::LOG_DIR;
use ridews::constants::URL_WEBSOCKET_SERVER	;


fn main()  {
    util::logger_init(LOG_DIR)  ;
	socket::websocket_async_server(URL_WEBSOCKET_SERVER);
}


