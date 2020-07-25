extern crate dbus;

use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use dbus::strings::Path;
use std::time::Duration;

#[derive(Debug)]
pub enum Status {
    Running,
    Stopped,
    Failed,
    Unavailable,
    Unknown,
}

pub struct Services {
    conn: Connection,
}

impl Services {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::new_system()?;
        Ok(Services { conn })
    }

    pub fn state(&self, service: &str) -> Status {
        self._state(service).unwrap_or(Status::Unavailable)
    }

    fn _state(&self, service: &str) -> Result<Status, Box<dyn std::error::Error>> {
        let interface = Path::new(format!(
            "/org/freedesktop/systemd1/unit/{}_2eservice",
            service.replace("@", "_40")
        ))?;
        let proxy = self.conn.with_proxy(
            "org.freedesktop.systemd1",
            interface,
            Duration::from_millis(5000),
        );
        let load: String = proxy.get("org.freedesktop.systemd1.Unit", "LoadState")?;
        let active: String = proxy.get("org.freedesktop.systemd1.Unit", "ActiveState")?;
        let detail: String = proxy.get("org.freedesktop.systemd1.Unit", "SubState")?;
        Ok(match (&load[..], &active[..], &detail[..]) {
            ("loaded", "active", "running") => Status::Running,
            ("loaded", "inactive", _) => Status::Stopped,
            ("loaded", "failed", _) => Status::Failed,
            ("not-found", _, _) => Status::Unavailable,
            _ => Status::Unknown,
        })
    }
}
