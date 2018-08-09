#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate virt;
extern crate clap;
mod domain_state;

use virt::connect::*;
use clap::{App, Arg};
use std::process::exit;
use domain_state::*;

#[derive(Serialize)]
struct Domain {
    pub name: String,
    pub state: State,
    pub max_mem: u64,
    pub memory: u64,
    pub nr_virt_cpu: u32,
    pub cpu_time: u64,
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

        let domain = Domain {
            name: name,
            state: state,
            memory: info.memory,
            max_mem: info.max_mem,
            nr_virt_cpu: info.nr_virt_cpu,
            cpu_time: info.cpu_time,
        };

        list.push(domain);
    }

    println!("{}", serde_json::to_string_pretty(&list).unwrap());
}
