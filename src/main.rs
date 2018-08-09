#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate virt;
extern crate clap;
mod domain_state;
mod info;

use virt::connect::*;
use virt::domain::*;
use clap::{App, Arg};
use std::process::exit;
use info::*;
use domain_state::*;

#[derive(Serialize)]
struct Domain {
    pub name: String,
    pub state: State,

    #[serde(with = "DomainInfoDef")]
    pub info: DomainInfo,
}

fn main() {
    let matches = App::new("hypermon")
        .arg(Arg::with_name("connect")
        .short("c")
        .long("connect")
        .value_name("URI")
        .help("Specify the libvirt connection URI")
        .required(true)
        .takes_value(true))
        .get_matches();

    let conn_uri = matches.value_of("connect").expect("Connection URI not found");
    let conn = match Connect::open_read_only(&conn_uri) {
        Ok(c) => c,
        Err(_) => exit(1),
    };
    let running = conn.list_all_domains(VIR_CONNECT_LIST_DOMAINS_ACTIVE).unwrap();

    let mut list: Vec<Domain> = Vec::new();

    for domain in running {
        let name = domain.get_name().unwrap();
        let (s, _) = domain.get_state().unwrap();
        let state = State::new(s);
        let info = domain.get_info().unwrap();

        let domain = Domain { name, state, info };
        list.push(domain);
    }

    println!("{}", serde_json::to_string_pretty(&list).unwrap());
}
