#[macro_use] extern crate prettytable;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate virt;
extern crate clap;
mod domain_state;

use prettytable::Table;
use virt::connect::*;
use virt::domain::*;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifaces: Option<Vec<IfaceStats>>,
}

#[derive(Serialize)]
struct IfaceStats {
    name: String,
    addresses: Vec<String>,
    rx_bytes: i64,
    tx_bytes: i64,
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
        .arg(Arg::with_name("table")
             .short("t")
             .long("table")
             .help("Output table instead of JSON"))
        .arg(Arg::with_name("all")
             .short("a")
             .long("all")
             .help("List all domains (Default: list currently running domains)"))
        .get_matches();

    let conn_uri = matches.value_of("connect").expect("Connection URI not found");
    let conn = match Connect::open(&conn_uri) {
        Ok(c) => c,
        Err(_) => exit(1),
    };

    let selection = if matches.is_present("all") { 0 } else { VIR_CONNECT_LIST_DOMAINS_ACTIVE };
    let vms = conn.list_all_domains(selection).unwrap();

    let is_table = matches.is_present("table");
    let mut list: Vec<Domain> = Vec::new();

    for domain in vms {
        let name = domain.get_name().unwrap();
        let (s, _) = domain.get_state().unwrap();
        let state = State::new(s);
        let info = domain.get_info().unwrap();

        let ifaces = match domain.interface_addresses(VIR_DOMAIN_INTERFACE_ADDRESSES_SRC_LEASE) {
            Ok(interfaces) => {
                let mut iface_stats: Vec<IfaceStats> = Vec::new();

                for interface in interfaces {
                    let stats: InterfaceStats = domain.interface_stats(&interface.hwaddr).unwrap();
                    let mut addresses: Vec<String> = Vec::new();
                    for addr in interface.addrs {
                        addresses.push(format!("{}/{}", addr.addr, addr.prefix));
                    }
                    iface_stats.push(
                        IfaceStats {
                            name: interface.name.to_owned(),
                            addresses: addresses,
                            rx_bytes: stats.rx_bytes,
                            tx_bytes: stats.tx_bytes,
                        }
                    );
                }
                Some(iface_stats)
            },
            Err(_) => {
                None
            },
        };

        let domain = Domain {
            name: name,
            state: state,
            memory: info.memory,
            max_mem: info.max_mem,
            nr_virt_cpu: info.nr_virt_cpu,
            cpu_time: info.cpu_time,
            ifaces: ifaces,
        };

        list.push(domain);
    }

    match is_table {
        false => {
            println!("{}", serde_json::to_string_pretty(&list).unwrap());
        },
        true => {
            let mut table = Table::new();
            table.add_row(row!["Name", "State", "Memory", "Max Memory", "Virt. CPUs", "CPU Time", "Bytes Rx", "Bytes Tx"]);
            for domain in list {
                let rx = if let Some(ifaces) = &domain.ifaces { ifaces.iter().fold(0, |acc, iface| acc + iface.rx_bytes).to_string() } else { "".to_string() };
                let tx = if let Some(ifaces) = &domain.ifaces { ifaces.iter().fold(0, |acc, iface| acc + iface.tx_bytes).to_string() } else { "".to_string() };

                table.add_row(row![
                    domain.name, 
                    domain.state,
                    domain.memory,
                    domain.max_mem,
                    domain.nr_virt_cpu,
                    domain.cpu_time,
                    rx,
                    tx,
                ]);
            }
            table.printstd();
        },
    }
}
