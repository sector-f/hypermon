extern crate new_libvirt;
use new_libvirt::connect::*;
use new_libvirt::domain::*;

extern crate libvirt_sys;

#[macro_use] extern crate prettytable;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate clap;
mod domain_state;
mod callback;

use prettytable::Table;
use clap::{App, Arg};
use std::ptr;
use std::os::raw::c_void;
use std::process::exit;
use domain_state::*;
use callback::*;

#[derive(Serialize)]
struct DomainOutput {
    pub name: String,
    pub state: State,
    pub max_mem: u64,
    pub memory: u64,
    pub num_virt_cpus: u32,
    pub cpu_time: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifaces: Option<Vec<IfaceStats>>,
}

#[derive(Serialize)]
struct IfaceStats {
    name: String,
    mac: String,
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
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Print error reporting to stderr"))
        .get_matches();

    let verbose = matches.is_present("verbose");

    unsafe {
        libvirt_sys::virSetErrorFunc(ptr::null::<c_void>() as *mut c_void, Some(do_nothing));
    }

    let conn_uri = matches.value_of("connect").expect("Connection URI not found");
    let conn = match Connect::open(&conn_uri) {
        Ok(c) => c,
        Err(_) => exit(1),
    };

    let selection = if matches.is_present("all") { &[ListAllDomainsFlags::All] } else { &[ListAllDomainsFlags::Active] };

    let vms = match conn.list_all_domains(selection) {
        Ok(vms) => vms,
        Err(e) => {
            if verbose {
                eprintln!("virConnectListAllDomains failed: {}; falling back to legacy mode", e);
            }

            let mut domains = Vec::new();

            for id in conn.list_active_domains().expect("Legacy mode failed (virConnectListDomains) {") {
                domains.push(Domain::lookup_by_id(&conn, id).unwrap());
            }

            if matches.is_present("all") {
                for name in conn.list_defined_domains().expect("Legacy mode failed (virConnectListDefinedDomains)") {
                    domains.push(Domain::lookup_by_name(&conn, &name).unwrap());
                }
            }

            domains
        },
    };

    let is_table = matches.is_present("table");
    let mut list: Vec<DomainOutput> = Vec::new();

    for domain in vms {
        let name = domain.get_name().unwrap();
        let info = domain.get_info().unwrap();
        let state = State::new((info.state as u8).into());

        let ifaces = match domain.interface_addresses(InterfaceAddressSource::Lease) {
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
                            mac: interface.hwaddr.to_owned(),
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

        let domain = DomainOutput {
            name: name,
            state: state,
            memory: info.memory,
            max_mem: info.max_mem,
            num_virt_cpus: info.nr_virt_cpu,
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
            table.add_row(row!["Name", "State", "Memory", "Max Memory", "Virt. CPUs", "CPU Time", "Interfaces", "Bytes Rx", "Bytes Tx"]);
            for domain in list {
                let num_ifaces = if let Some(ifaces) = &domain.ifaces { ifaces.len().to_string() } else { "".to_string() };
                let rx = if let Some(ifaces) = &domain.ifaces { ifaces.iter().fold(0, |acc, iface| acc + iface.rx_bytes).to_string() } else { "".to_string() };
                let tx = if let Some(ifaces) = &domain.ifaces { ifaces.iter().fold(0, |acc, iface| acc + iface.tx_bytes).to_string() } else { "".to_string() };

                table.add_row(row![
                    domain.name,
                    domain.state,
                    domain.memory,
                    domain.max_mem,
                    domain.num_virt_cpus,
                    domain.cpu_time,
                    num_ifaces,
                    rx,
                    tx,
                ]);
            }
            table.printstd();
        },
    }
}
