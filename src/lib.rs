/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::process::Command;

/// Returns true if connected to a network (a route is available)
pub fn route_available() -> bool  {
  lazy_static! {
    static ref ROUTE_MATCHER: Regex  = Regex::new(r"(default)").expect("bad regex");
  }

  let out = Command::new("ip")
    .arg("route")
    .arg("show")
    .output()
    .expect("couldn't run command");

  let text = String::from_utf8_lossy(&out.stdout);

  let matches = ROUTE_MATCHER.is_match(&text);
  if !matches { println!("no match on: {} \n\n", text); }

  matches
}

#[cfg(test)]
mod tests {
use route_available ;

    #[test]
    fn it_works() {
      assert!(route_available());
    }
}
