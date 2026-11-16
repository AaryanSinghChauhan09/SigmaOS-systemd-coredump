// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-coredump: systemd coredump component model.
//!
//! Real implementation: coredumps, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coredump {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdCoredumpError {
    EmptyCoredumpName,
    DuplicateCoredump(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdCoredumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCoredumpName => write!(f, "coredump name must not be empty"),
            Self::DuplicateCoredump(id) => write!(f, "duplicate coredump {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdCoredumpError {}

pub struct SystemdCoredump {
    pub name: String,
    coredumps: BTreeMap<String, Coredump>,
    events: BTreeMap<String, Event>,
}

impl SystemdCoredump {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            coredumps: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_coredump(&mut self, item: Coredump) -> Result<(), SystemdCoredumpError> {
        if item.name.trim().is_empty() {
            return Err(SystemdCoredumpError::EmptyCoredumpName);
        }
        let id = item.id.clone();
        if self.coredumps.contains_key(&id) {
            return Err(SystemdCoredumpError::DuplicateCoredump(id.clone()));
        }
        self.coredumps.insert(id, item);
        Ok(())
    }

    pub fn coredump_count(&self) -> usize {
        self.coredumps.len()
    }

    pub fn get_coredump(&self, id: &str) -> Option<&Coredump> {
        self.coredumps.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdCoredumpError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdCoredumpError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_coredump(id: &str, name: &str) -> Coredump {
        Coredump {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdCoredump::new("SystemdCoredump");
        assert_eq!(m.name, "SystemdCoredump");
        assert_eq!(m.coredump_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_coredump() {
        let mut m = SystemdCoredump::new("SystemdCoredump");
        m.add_coredump(sample_coredump("a1", "Alpha")).unwrap();
        assert_eq!(m.coredump_count(), 1);
        assert!(m.get_coredump("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdCoredump::new("SystemdCoredump");
        assert!(m.add_coredump(sample_coredump("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdCoredump::new("SystemdCoredump");
        m.add_coredump(sample_coredump("x", "One")).unwrap();
        assert!(m.add_coredump(sample_coredump("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdCoredump::new("SystemdCoredump");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
