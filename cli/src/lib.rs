// Copyright 2019 PolkaX

use clap::{App, Arg};

pub fn get_terminal() -> Option<String> {
    let matches = App::new("plum")
                      .version("0.1")
                      .author("PolkaX")
                      .arg(Arg::with_name("peer")
                            .short("p")
                            .long("peer")
                            .help("Sets a peer ipfs ip")
                            .takes_value(true))
                      .get_matches();
     if let Some(v) = matches.value_of("peer") {
         Some(v.to_string())
     } else {
         None
     }
}

#[cfg(test)]
mod tests {
}
