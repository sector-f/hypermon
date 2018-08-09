// use virt::connect::*;
use virt::domain::DomainInfo;

#[derive(Serialize, Deserialize)]
#[serde(remote = "DomainInfo")]
pub struct DomainInfoDef {
    #[serde(skip)]
    pub state: u32,
    pub max_mem: u64,
    pub memory: u64,
    pub nr_virt_cpu: u32,
    pub cpu_time: u64,
}
