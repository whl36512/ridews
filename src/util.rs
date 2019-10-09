use env_logger;
pub fn logger_init () {
		env_logger::init();
}


// url::form_urlencoded::parse()
//
//extern crate simplelog;
//use self::simplelog::*;
//use std::fs::File;
//use std::fs::OpenOptions;
/*
pub fn logger_init (log_file: &str) {
        CombinedLogger::init(
            vec![
                TermLogger::init(LevelFilter::Debug, Config::default()).unwrap(),
                WriteLogger::init(LevelFilter::Info, Config::default(),
                    OpenOptions::new()
                        //.read(true)
                        //.write(true)
                        .append(true)
                        .create(true)
                        .open(log_file).unwrap()
                //File::create(log_file).unwrap()
                ),
            ]
        ).unwrap();
}
*/


