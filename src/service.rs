use psutil::process::{Process, ProcessResult, processes};

#[derive(Debug,PartialEq)]
pub enum Status {
    Good,
    Bad,
}

fn map_proc(prc: ProcessResult<Process>) -> ProcessResult<(String, Status)> {
    let prc = prc?;
    let status = match prc.status()? {
        Running => Status::Good,
        Sleeping => Status::Good,
        Idle => Status::Good,
        _ => Status::Bad
    };
    Ok((prc.name()?, status))
}

pub fn state<'a>(names: &[&'a str]) -> Vec<(&'a str, Status)> {
    let mut result: Vec<_> = names.iter().map(|&n| (n, Status::Bad)).collect();
    result.sort_by_key(|e| e.0);
    for prc in processes().unwrap() {
        if let Ok((name, state)) = map_proc(prc) {
            if let Ok(pos) = result.binary_search_by(|e| e.0.cmp(&name[..])) {
                if result[pos].1 == Status::Bad {
                    result[pos].1 = state;
                }
            }
        }
    }
    result
}
