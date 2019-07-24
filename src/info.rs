use std::time::SystemTime;
use serde_json::{Value};
use sysinfo::{
    RefreshKind,
    Pid,
    System, SystemExt,
    Process, ProcessExt,
    ProcessorExt, ComponentExt, NetworkExt, DiskExt
};

pub struct Info {
    pid: Pid,
    sys: System
}
impl Info {
    pub fn new () -> Info {
        let pid = sysinfo::get_current_pid().unwrap();
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_system()
                .with_network()
                // .with_processes()
                // .with_disks()
                .with_disk_list()
        );
        let s = Info {pid, sys};
        s
    }
    pub fn refresh (&mut self) {
        // self.sys.refresh_all();
        self.sys.refresh_system(); // memory, swap, CPU usage and components' temperature
        // self.sys.refresh_processes();
        self.sys.refresh_process(self.pid);
        self.sys.refresh_disks(); // reload exist list
        // self.sys.refresh_disk_list(); // re get the list, exclude available_space
        self.sys.refresh_network();
    }
    pub fn get_partial (&self) -> Value {
        // let sys = &self.sys;
        let j = json!({
            "now": SystemTime::now(),
        });
        j
    }
    pub fn get_all (&self) -> Value {
        let sys = &self.sys;
        let processors = sys_build_processors(sys);
        let components = sys_build_components(sys);
        let disks = sys_build_disks(sys);
        let process = sys_build_process(&self.pid, sys);
        // let processes = build_sys_processes(sys);
        let j = json!({
            "now": SystemTime::now(),
            "uptime": sys.get_uptime(), // /proc/uptime, seconds
            "memory": {
                "total": sys.get_total_memory(), // KiB
                "used": sys.get_used_memory(), // KiB
                "free": sys.get_free_memory(), // KiB
            },
            "swap": {
                "total": sys.get_total_swap(), // KiB
                "free": sys.get_free_swap(), // KiB
                "used": sys.get_used_swap(), // KiB
            },
            "network": {
                "input": sys.get_network().get_income(), // bytes
                "output": sys.get_network().get_outcome(), // bytes
            },
            "processors": processors,
            "components": components,
            "disks": disks,
            "process": process,
            // "processes": processes,
        });
        j
    }
}
fn sys_build_processors(sys: &System) -> Vec<Value> {
    let mut processors = vec![];
    for p in sys.get_processor_list() {
        // p[0] is total process usage
        processors.push(json!({
            "cpu_usage": p.get_cpu_usage(), // f32
            "name": p.get_name(), // &str
        }))
    }
    processors
}
fn sys_build_components (sys: &System) -> Vec<Value> {
    let mut components = vec![];
    for c in sys.get_components_list() {
        components.push(json!({
            // celsius degree
            "temperature": c.get_temperature(), // f32
            "max": c.get_max(), // f32,
            "critical": c.get_critical(), // Option<f32>
            "label": c.get_label(), // &str
        }))
    }
    components
}
fn sys_build_disks (sys: &System) -> Vec<Value> {
    let mut disks = vec![];
    for disk in sys.get_disks() {
        disks.push(json!({
            "type": format!("{:?}", disk.get_type()), // DiskType
            "name": disk.get_name().to_str(), // OsStr
            "file_system": disk.get_file_system(), // &[u8]
            "mount_point": disk.get_mount_point(), // &Path
            "total_space": disk.get_total_space(), // u64
            "available_space": disk.get_available_space(), // u64
        }))
    }
    disks
}
fn build_process (p: &Process) -> Value {
    json!({
        "uid": p.uid,
        "gid": p.gid,
        // "tasks": p.tasks, // not impl yet?
        "name": p.name(), // &str
        "cmd": p.cmd(), // &[String], sensitive
        "exe": p.exe(), // &Path, sensitive
        "pid": p.pid(), // Pid
        // "environ": p.environ(), // &[String], sensitive
        "cwd": p.cwd(), // &Path
        // "root": p.root(), // &Path, sensitive: $PATH
        "memory": p.memory(), // u64 kB
        "parent_pid": p.parent(), // Option<Pid>
        "status": p.status().to_string(), // ProcessStatus
        "start_time": p.start_time(), // u64 seconds
        "cpu_usage": p.cpu_usage(), // f32
    })
}
fn sys_build_process (pid: &Pid, sys: &System) -> Value {
    match sys.get_process(*pid) {
        Some(p) => build_process(p),
        None => json!(Value::Null)
    }
}
// fn build_sys_processes (sys: &System) -> Vec<Value> {
//     let mut processes = vec![];
//     for (pid, p) in sys.get_process_list() {
//         processes.push(json!({
//             "pid": pid,
//             "uid": p.uid,
//             "gid": p.gid,
//             // "tasks": p.tasks, // not impl yet?
//             "name": p.name(), // &str
//             "cmd": p.cmd(), // &[String]
//             "exe": p.exe(), // &Path
//             // "pid": p.pid(), // Pid
//             "environ": p.environ(), // &[String]
//             "cwd": p.cwd(), // &Path
//             "root": p.root(), // &Path
//             "memory": p.memory(), // u64 kB
//             "parent_pid": p.parent(), // Option<Pid>
//             "status": p.status().to_string(), // ProcessStatus
//             "start_time": p.start_time(), // u64 seconds
//             "cpu_usage": p.cpu_usage(), // f32
//         }))
//     }
//     processes
// }
