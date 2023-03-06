//! The compound trigger.
//!
//! Requires the `compound_trigger` feature.
//! compound_trigger includes size trigger and date trigger
//! The log will roll if one of the following condition is satisfied:
//! 1, The log's size is large than the limit
//! 2, The date changes.

#[cfg(feature = "config_parsing")]
use serde::de;
#[cfg(feature = "config_parsing")]
use std::fmt;

use crate::append::rolling_file::{policy::compound::trigger::Trigger, LogFile};
use super::size::deserialize_limit;
use super::size::SizeTrigger;

#[cfg(feature = "config_parsing")]
use crate::config::{Deserialize, Deserializers};

use chrono;

/// Configuration for the compound trigger.
#[cfg(feature = "config_parsing")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompoundTriggerConfig {
    #[serde(deserialize_with = "deserialize_limit")]
    limit: u64,
    #[serde(deserialize_with = "deserialize_date")]
    date: bool,
}

#[cfg(feature = "config_parsing")]
fn deserialize_date<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct V;

    impl<'de2> de::Visitor<'de2> for V {
        type Value = bool;

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str("true or false")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error, {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<bool, E>
        where
            E: de::Error, {
            if v.eq("true") {
                return Ok(true);
            }  
            Ok(false)
        }
    }

    d.deserialize_any(V)
}

/// A trigger which rolls the log once it has passed a certain size
/// or the date changes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CompoundTrigger {
    size_trigger: SizeTrigger,
}

/// The trigger is referred as immutable when the trigger is called.
/// But we must update the latest log date to see whether the date of log should be rolled.
/// I put the log date global so the updating is able to be done in the trigger.
static mut LOG_DATE: Option<String> = None;

impl CompoundTrigger {
    /// Returns a new trigger 
    pub fn new(limit: u64, date: bool) -> CompoundTrigger {
        if date {
            unsafe {
                LOG_DATE = Some(chrono::Local::now().format("%Y%m%d").to_string());
            }
        }
        CompoundTrigger { 
            size_trigger: SizeTrigger::new(limit),
        }
    }
}

impl Trigger for CompoundTrigger {
    fn trigger(&self, file: &LogFile) -> anyhow::Result<bool> {
        unsafe {
            if let Some(now) = &LOG_DATE {
                let new_now = chrono::Local::now().format("%Y%m%d").to_string();
                if *now != new_now {
                    LOG_DATE = Some(new_now.clone());
                    return Ok(true);
                }
            }
        }
        self.size_trigger.trigger(file)
    }
}

/// A deserializer for the `CompoundTrigger`.
///
/// # Configuration
///
/// ```yaml
/// kind: compound 
///
/// # The size limit in bytes. The following units are supported (case insensitive):
/// # "b", "kb", "kib", "mb", "mib", "gb", "gib", "tb", "tib". The unit defaults to
/// # bytes if not specified. Required.
/// limit: 10 mb
/// 
/// # Whether the log is rolled when the date changes:
/// # true or false
/// # false if not configured
/// date: true
/// ```
#[cfg(feature = "config_parsing")]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CompoundTriggerDeserializer;

#[cfg(feature = "config_parsing")]
impl Deserialize for CompoundTriggerDeserializer {
    type Trait = dyn Trigger;

    type Config = CompoundTriggerConfig;

    fn deserialize(
        &self,
        config: CompoundTriggerConfig,
        _: &Deserializers,
    ) -> anyhow::Result<Box<dyn Trigger>> {
        Ok(Box::new(CompoundTrigger::new(config.limit, config.date)))
    }
}
