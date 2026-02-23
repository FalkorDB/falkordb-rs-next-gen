//! Runtime value representation for Cypher expressions.
//!
//! This module defines the [`Value`] enum which represents all possible values
//! in Cypher queries at runtime. Values can be:
//!
//! - Primitives: Null, Bool, Int, Float, String
//! - Temporal: Datetime, Date, Time, Duration
//! - Collections: List, Map
//! - Graph entities: Node, Relationship, Path
//! - Special: Point (geographic), VecF32 (vector embeddings)
//!
//! ## Type Coercion
//!
//! Values support implicit coercion in operations:
//! - Int + Float → Float
//! - String + anything → String concatenation
//! - Null propagates through most operations
//!
//! ## Comparison Rules
//!
//! - Nulls compare as neither less than, equal to, nor greater than any value
//! - Different types have a defined ordering for sorting
//! - Nodes/Relationships compare by their IDs

#![allow(clippy::cast_precision_loss)]

use json_escape::escape_str;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Rem, Sub},
    sync::Arc,
};

use thin_vec::{ThinVec, thin_vec};

use crate::{
    ast::Variable,
    graph::graph::{LabelId, NodeId, RelationshipId, TypeId},
    runtime::{functions::Type, ordermap::OrderMap},
};

/// A trait for formatting values as JSON, similar to Display but for JSON output
pub trait DisplayJson {
    fn fmt_json(
        &self,
        f: &mut fmt::Formatter<'_>,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> fmt::Result;
}

/// Snapshot of a deleted node's data for query result consistency.
///
/// When a node is deleted during query execution, its data is preserved
/// here so that RETURN clauses can still access it.
#[derive(Clone, Debug, PartialEq)]
pub struct DeletedNode {
    pub labels: HashSet<LabelId>,
    pub attrs: OrderMap<Arc<String>, Value>,
}

impl DeletedNode {
    #[must_use]
    pub const fn new(
        labels: HashSet<LabelId>,
        attrs: OrderMap<Arc<String>, Value>,
    ) -> Self {
        Self { labels, attrs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeletedRelationship {
    pub type_id: TypeId,
    pub attrs: OrderMap<Arc<String>, Value>,
}

impl DeletedRelationship {
    #[must_use]
    pub const fn new(
        type_id: TypeId,
        attrs: OrderMap<Arc<String>, Value>,
    ) -> Self {
        Self { type_id, attrs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    #[must_use]
    pub const fn new(
        latitude: f32,
        longitude: f32,
    ) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    pub fn distance(
        &self,
        other: &Self,
    ) -> f64 {
        let lat1 = self.latitude.to_radians() as f64;
        let lon1 = self.longitude.to_radians() as f64;
        let lat2 = other.latitude.to_radians() as f64;
        let lon2 = other.longitude.to_radians() as f64;

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // Earth's radius in meters
        const EARTH_RADIUS: f64 = 6378140.0;
        EARTH_RADIUS * c
    }

    /// Validates that the point coordinates are within valid ranges
    pub fn validate(&self) -> Result<(), String> {
        // Check for NaN or infinite values first
        if !self.latitude.is_finite() {
            return Err(format!(
                "latitude must be a finite number, got {}",
                self.latitude
            ));
        }
        if !self.longitude.is_finite() {
            return Err(format!(
                "longitude must be a finite number, got {}",
                self.longitude
            ));
        }
        // Then check range bounds
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(format!(
                "latitude should be within the range -90.0 to 90.0, got {}",
                self.latitude
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(format!(
                "longitude should be within the range -180.0 to 180.0, got {}",
                self.longitude
            ));
        }
        Ok(())
    }
}

/// Runtime value type representing all possible Cypher values.
///
/// Values are cloneable and use Arc for large data (strings, shared values)
/// to minimize copying during query execution.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    /// Cypher NULL value - represents missing or unknown data
    #[default]
    Null,
    /// Boolean true or false
    Bool(bool),
    /// 64-bit signed integer
    Int(i64),
    /// 64-bit floating point
    Float(f64),
    /// Unicode string (shared via Arc for efficiency)
    String(Arc<String>),
    /// Ordered list of values
    List(ThinVec<Self>),
    /// Key-value map with string keys
    Map(OrderMap<Arc<String>, Self>),
    /// Reference to a graph node (by ID)
    Node(NodeId),
    /// Reference to a relationship: (edge_id, source_node, target_node)
    Relationship(Box<(RelationshipId, NodeId, NodeId)>),
    /// A path through the graph (alternating nodes and relationships)
    Path(ThinVec<Self>),
    /// Float32 vector (for vector similarity operations)
    VecF32(ThinVec<f32>),
    /// Geographic point (latitude, longitude)
    Point(Point),
    /// DateTime as Unix timestamp in milliseconds
    Datetime(i64),
    /// Date as Unix timestamp in milliseconds (midnight UTC)
    Date(i64),
    /// Time as nanoseconds from midnight
    Time(i64),
    /// Duration in milliseconds
    Duration(i64),
    /// Shared value reference (for lazy evaluation)
    Arc(Arc<Self>),
}

impl Value {
    #[must_use]
    #[inline]
    pub fn get_numeric(&self) -> f64 {
        match &self {
            Self::Int(i) => *i as f64,
            Self::Float(f) => *f,
            Self::Null => 0.0,
            _ => unreachable!("Expected numeric value, got {}", self.name()),
        }
    }

    #[must_use]
    pub fn format_datetime(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    // Format date as ISO-8601: "2025-04-14"
    #[must_use]
    pub fn format_date(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    // Format time as ISO-8601: "06:08:21"
    #[must_use]
    pub fn format_time(timestamp_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        match Utc.timestamp_millis_opt(timestamp_ms) {
            chrono::LocalResult::Single(dt) => dt.format("%H:%M:%S").to_string(),
            _ => format!("<invalid timestamp: {timestamp_ms}>"),
        }
    }

    #[must_use]
    pub fn format_duration(duration_ms: i64) -> String {
        let seconds = duration_ms / 1000;
        format!("PT{seconds}S")
    }

    /// Get a named attribute/component from this value.
    ///
    /// Handles Map key lookup, Point fields, and temporal component extraction.
    /// Node and Relationship property access requires graph state and is handled
    /// by the runtime, not here.
    pub fn get_attr(
        &self,
        attr: &str,
    ) -> Result<Value, String> {
        match self {
            Value::Map(map) => Ok(map.get_str(attr).cloned().unwrap_or(Value::Null)),
            Value::Point(p) => Ok(Self::get_point_component(p, attr)),
            Value::Datetime(ts) => Self::get_datetime_component(*ts, attr),
            Value::Date(ts) => Self::get_date_component(*ts, attr),
            Value::Time(ts) => Self::get_time_component(*ts, attr),
            Value::Duration(dur) => Self::get_duration_component(*dur, attr),
            Value::Null => Ok(Value::Null),
            v => Err(format!(
                "Type mismatch: expected Map, Node, Edge, Datetime, Date, Time, Duration, Null, or Point but was {}",
                v.name()
            )),
        }
    }

    fn get_point_component(
        p: &Point,
        attr: &str,
    ) -> Value {
        if attr.eq_ignore_ascii_case("latitude") {
            Value::Float(p.latitude as f64)
        } else if attr.eq_ignore_ascii_case("longitude") {
            Value::Float(p.longitude as f64)
        } else {
            Value::Null
        }
    }

    /// Extract a component from a datetime value (stored as milliseconds since
    /// epoch).  Mirrors the C `DateTime_getComponent` in `datetime.c`.
    fn get_datetime_component(
        timestamp_ms: i64,
        component: &str,
    ) -> Result<Value, String> {
        use chrono::{Datelike, TimeZone, Timelike, Utc};

        let chrono::LocalResult::Single(dt) = Utc.timestamp_millis_opt(timestamp_ms) else {
            return Ok(Value::Null);
        };

        let c = component;
        let val = if c.eq_ignore_ascii_case("second") {
            i64::from(dt.second())
        } else if c.eq_ignore_ascii_case("minute") {
            i64::from(dt.minute())
        } else if c.eq_ignore_ascii_case("hour") {
            i64::from(dt.hour())
        } else if c.eq_ignore_ascii_case("day") {
            i64::from(dt.day())
        } else if c.eq_ignore_ascii_case("month") {
            i64::from(dt.month())
        } else if c.eq_ignore_ascii_case("year") {
            i64::from(dt.year())
        } else if c.eq_ignore_ascii_case("dayOfWeek") {
            i64::from(dt.weekday().num_days_from_sunday())
        } else if c.eq_ignore_ascii_case("weekDay") {
            // ISO weekday: Monday=1 .. Sunday=7
            let w = dt.weekday().num_days_from_sunday();
            if w == 0 { 7 } else { i64::from(w) }
        } else if c.eq_ignore_ascii_case("ordinalDay") {
            i64::from(dt.ordinal())
        } else if c.eq_ignore_ascii_case("quarter") {
            i64::from((dt.month() - 1) / 3 + 1)
        } else if c.eq_ignore_ascii_case("week") {
            i64::from(dt.iso_week().week())
        } else if c.eq_ignore_ascii_case("weekYear") {
            i64::from(dt.iso_week().year())
        } else if c.eq_ignore_ascii_case("dayOfQuarter") || c.eq_ignore_ascii_case("quarterDay") {
            let q = (dt.month() - 1) / 3 + 1;
            let quarter_start_month = (q - 1) * 3 + 1;
            let quarter_start = Utc
                .with_ymd_and_hms(dt.year(), quarter_start_month, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| "Invalid quarter start date".to_string())?;
            (dt.date_naive() - quarter_start.date_naive()).num_days() + 1
        } else if c.eq_ignore_ascii_case("millisecond") {
            timestamp_ms.rem_euclid(1000)
        } else if c.eq_ignore_ascii_case("microsecond") || c.eq_ignore_ascii_case("nanosecond") {
            // microsecond/nanosecond precision not stored
            0
        } else {
            return Err(format!("unknown datetime component {component}"));
        };

        Ok(Value::Int(val))
    }

    /// Extract a component from a date value (stored as milliseconds since
    /// epoch at midnight UTC).  Mirrors the C `Date_getComponent` in `date.c`.
    fn get_date_component(
        timestamp_ms: i64,
        component: &str,
    ) -> Result<Value, String> {
        use chrono::{Datelike, TimeZone, Utc};

        let chrono::LocalResult::Single(dt) = Utc.timestamp_millis_opt(timestamp_ms) else {
            return Ok(Value::Null);
        };

        let c = component;
        let val = if c.eq_ignore_ascii_case("day") {
            i64::from(dt.day())
        } else if c.eq_ignore_ascii_case("month") {
            i64::from(dt.month())
        } else if c.eq_ignore_ascii_case("year") {
            i64::from(dt.year())
        } else if c.eq_ignore_ascii_case("dayOfWeek") {
            i64::from(dt.weekday().num_days_from_sunday())
        } else if c.eq_ignore_ascii_case("weekDay") {
            let w = dt.weekday().num_days_from_sunday();
            if w == 0 { 7 } else { i64::from(w) }
        } else if c.eq_ignore_ascii_case("ordinalDay") {
            i64::from(dt.ordinal())
        } else if c.eq_ignore_ascii_case("quarter") {
            i64::from((dt.month() - 1) / 3 + 1)
        } else if c.eq_ignore_ascii_case("week") {
            i64::from(dt.iso_week().week())
        } else if c.eq_ignore_ascii_case("weekYear") {
            i64::from(dt.iso_week().year())
        } else if c.eq_ignore_ascii_case("dayOfQuarter") || c.eq_ignore_ascii_case("quarterDay") {
            let q = (dt.month() - 1) / 3 + 1;
            let quarter_start_month = (q - 1) * 3 + 1;
            let quarter_start = Utc
                .with_ymd_and_hms(dt.year(), quarter_start_month, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| "Invalid quarter start date".to_string())?;
            (dt.date_naive() - quarter_start.date_naive()).num_days() + 1
        } else {
            return Err(format!("unknown date component {component}"));
        };

        Ok(Value::Int(val))
    }

    /// Extract a component from a time value (stored as milliseconds since
    /// epoch with a fixed base date).  Mirrors the C `Time_getComponent` in
    /// `time.c`.
    fn get_time_component(
        timestamp_ms: i64,
        component: &str,
    ) -> Result<Value, String> {
        use chrono::{TimeZone, Timelike, Utc};

        let chrono::LocalResult::Single(dt) = Utc.timestamp_millis_opt(timestamp_ms) else {
            return Ok(Value::Null);
        };

        let c = component;
        let val = if c.eq_ignore_ascii_case("second") {
            i64::from(dt.second())
        } else if c.eq_ignore_ascii_case("minute") {
            i64::from(dt.minute())
        } else if c.eq_ignore_ascii_case("hour") {
            i64::from(dt.hour())
        } else {
            return Err(format!("unknown time component {component}"));
        };

        Ok(Value::Int(val))
    }

    /// Extract a component from a duration value (stored as milliseconds from
    /// epoch).  Mirrors the C `Duration_getComponent` in `duration.c` which
    /// calls `duration_from_time_t_utc` to decompose the raw value back into
    /// calendar / clock fields.
    fn get_duration_component(
        duration_ms: i64,
        component: &str,
    ) -> Result<Value, String> {
        use chrono::{Datelike, TimeZone, Utc};

        // Fast-reject unknown components before doing any chrono work.
        let c = component;
        if !(c.eq_ignore_ascii_case("years")
            || c.eq_ignore_ascii_case("months")
            || c.eq_ignore_ascii_case("weeks")
            || c.eq_ignore_ascii_case("days")
            || c.eq_ignore_ascii_case("hours")
            || c.eq_ignore_ascii_case("minutes")
            || c.eq_ignore_ascii_case("seconds"))
        {
            return Err(format!("unknown duration component {component}"));
        }

        // weeks is always 0 in the C decomposition — skip chrono entirely.
        if c.eq_ignore_ascii_case("weeks") {
            return Ok(Value::Float(0.0));
        }

        let chrono::LocalResult::Single(dt) = Utc.timestamp_millis_opt(duration_ms) else {
            return Ok(Value::Null);
        };

        // Decompose into years/months from epoch (1970-01-01) – mirrors the C
        // logic in duration_from_time_t_utc.
        let epoch_year: i32 = 1970;
        let epoch_month: u32 = 1; // January

        let mut year_diff = dt.year() - epoch_year;
        let mut month_diff = dt.month() as i32 - epoch_month as i32;

        // month_diff is always >= 0 (dt.month() is 1..=12, epoch_month is 1),
        // but we keep this branch for parity with the C duration_from_time_t_utc.
        if month_diff < 0 {
            year_diff -= 1;
            month_diff += 12;
        }

        // For years/months we already have the answer — skip the rest.
        if c.eq_ignore_ascii_case("years") {
            return Ok(Value::Float(f64::from(year_diff)));
        }
        if c.eq_ignore_ascii_case("months") {
            return Ok(Value::Float(f64::from(month_diff)));
        }

        // Reconstruct an anchor date that has the same year/month offset from
        // epoch but day=1, midnight – so the remainder gives us days + time.
        let anchor = Utc
            .with_ymd_and_hms(
                epoch_year + year_diff,
                (epoch_month as i32 + month_diff) as u32,
                1,
                0,
                0,
                0,
            )
            .single()
            .ok_or_else(|| {
                format!(
                    "Invalid anchor date for duration decomposition (duration_ms={duration_ms})"
                )
            })?;

        let remaining_secs = (duration_ms / 1000) - anchor.timestamp();

        let val: f64 = if c.eq_ignore_ascii_case("days") {
            (remaining_secs / 86400) as f64
        } else if c.eq_ignore_ascii_case("hours") {
            ((remaining_secs % 86400) / 3600) as f64
        } else if c.eq_ignore_ascii_case("minutes") {
            ((remaining_secs % 3600) / 60) as f64
        } else {
            // "seconds"
            (remaining_secs % 60) as f64
        };

        Ok(Value::Float(val))
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        match self {
            Self::Null => {
                0.hash(state);
            }
            Self::Bool(x) => {
                1.hash(state);
                x.hash(state);
            }
            Self::Int(x) => {
                2.hash(state);
                x.hash(state);
            }
            Self::Float(x) => {
                2.hash(state);
                let casted = *x as i64;
                let diff = *x - casted as f64;
                if diff == 0.0 {
                    casted.hash(state);
                } else {
                    x.to_bits().hash(state);
                }
            }
            Self::String(x) => {
                3.hash(state);
                x.hash(state);
            }
            Self::List(x) => {
                4.hash(state);
                x.hash(state);
            }
            Self::Map(x) => {
                5.hash(state);
                x.hash(state);
            }
            Self::Node(x) => {
                6.hash(state);
                x.hash(state);
            }
            Self::Relationship(rel) => {
                7.hash(state);
                rel.0.hash(state);
            }
            Self::Path(x) => {
                8.hash(state);
                x.hash(state);
            }
            Self::VecF32(x) => {
                9.hash(state);
                for f in x {
                    f.to_bits().hash(state);
                }
            }
            Self::Point(p) => {
                10.hash(state);
                p.latitude.to_bits().hash(state);
                p.longitude.to_bits().hash(state);
            }
            Self::Datetime(x) => {
                11.hash(state);
                x.hash(state);
            }
            Self::Date(x) => {
                12.hash(state);
                x.hash(state);
            }
            Self::Time(x) => {
                13.hash(state);
                x.hash(state);
            }
            Self::Duration(x) => {
                14.hash(state);
                x.hash(state);
            }
            Self::Arc(x) => {
                x.hash(state);
            }
        }
    }
}

#[derive(Default)]
pub struct Env(Vec<Value>);

impl Env {
    pub fn insert(
        &mut self,
        key: &Variable,
        value: Value,
    ) {
        while self.0.len() <= key.id as _ {
            self.0.push(Value::Null);
        }
        self.0[key.id as usize] = value;
    }

    #[must_use]
    pub fn get(
        &self,
        key: &Variable,
    ) -> Option<Value> {
        self.0.get(key.id as usize).cloned()
    }

    /// Takes ownership of a value from the environment, replacing it with `Null`.
    ///
    /// This method is designed for aggregation optimizations where we need to transfer
    /// ownership of large accumulated values (like lists with millions of items) without
    /// cloning them. By replacing the environment entry with `Null`, we ensure the value
    /// can be moved (not cloned) to the aggregation function.
    ///
    /// # Returns
    /// - `Some(value)` if the key exists and contains a non-Null value
    /// - `None` if the key doesn't exist or already contains `Null`
    ///
    /// # Usage
    /// Prefer this over `get()` when:
    /// - You need exclusive ownership of a value
    /// - The value is expensive to clone (e.g., large collections)
    /// - The environment slot won't be read again before being overwritten
    pub fn take(
        &mut self,
        key: &Variable,
    ) -> Option<Value> {
        self.0.get_mut(key.id as usize).and_then(|value| {
            match std::mem::replace(value, Value::Null) {
                Value::Null => None,
                v => Some(v),
            }
        })
    }

    pub fn merge(
        &mut self,
        other: Self,
    ) {
        while self.0.len() < other.0.len() {
            self.0.push(Value::Null);
        }
        for (key, value) in other.0.into_iter().enumerate() {
            if value == Value::Null {
                continue;
            }
            self.0[key] = value;
        }
    }
}

impl AsRef<Vec<Value>> for Env {
    fn as_ref(&self) -> &Vec<Value> {
        &self.0
    }
}

impl Hash for Env {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        for (key, value) in self.0.iter().enumerate() {
            if *value == Value::Null {
                continue;
            }
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_add(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
            (Self::List(a), Self::List(b)) => Ok(Self::List(a.into_iter().chain(b).collect())),
            (Self::List(mut l), rhs) => {
                l.push(rhs);
                Ok(Self::List(l))
            }
            (lhs, Self::List(l)) => {
                let mut new_list = thin_vec![lhs];
                new_list.extend(l);
                Ok(Self::List(new_list))
            }
            (Self::Map(a), Self::Map(b)) => {
                let mut new_map = a;
                for (k, v) in b.iter() {
                    new_map.insert(k.clone(), v.clone());
                }
                Ok(Self::Map(new_map))
            }
            (Self::String(a), Self::String(b)) => Ok(Self::String(Arc::new(format!("{a}{b}")))),
            (Self::String(s), Self::Int(i)) => Ok(Self::String(Arc::new(format!("{s}{i}")))),
            (Self::String(s), Self::Float(f)) => Ok(Self::String(Arc::new(format!("{s}{f:.6}")))),
            (Self::String(s), Self::Bool(b)) => Ok(Self::String(Arc::new(format!("{s}{b}")))),

            (Self::Int(i), Self::String(s)) => Ok(Self::String(Arc::new(format!("{i}{s}")))),
            (Self::Float(f), Self::String(s)) => Ok(Self::String(Arc::new(format!("{f:.6}{s}")))),
            (Self::Bool(b), Self::String(s)) => Ok(Self::String(Arc::new(format!("{b}{s}")))),

            (a, b) => Err(format!(
                "Unexpected types for add operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_sub(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a - b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
            (a, b) => Err(format!(
                "Unexpected types for sub operator ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a.wrapping_mul(b))),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a * b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
            (a, Self::Int(_) | Self::Float(_)) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                a.name(),
            )),
            (Self::Int(_) | Self::Float(_), b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                b.name(),
            )),
            (a, _) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was {}",
                a.name(),
            )),
        }
    }
}

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Int(a.wrapping_div(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a / b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a / b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 / b)),
            (a, b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

impl Rem for Value {
    type Output = Result<Self, String>;

    fn rem(
        self,
        rhs: Self,
    ) -> Self::Output {
        match (self, rhs) {
            (Self::Null, _) | (_, Self::Null) => Ok(Self::Null),
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Self::Int(a.wrapping_rem(b)))
                }
            }
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a % b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a % b as f64)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 % b)),
            (a, b) => Err(format!(
                "Type mismatch: expected Integer, Float, or Null but was ({}, {})",
                a.name(),
                b.name()
            )),
        }
    }
}

trait OrderedEnum {
    fn order(&self) -> u32;
}

impl OrderedEnum for Value {
    fn order(&self) -> u32 {
        match self {
            Self::Null => 1 << 15,
            Self::Bool(_) => 1 << 12,
            Self::Int(_) => 1 << 13,
            Self::Float(_) => 1 << 14,
            Self::String(_) => 1 << 11,
            Self::List(_) => 1 << 3,
            Self::Map(_) => 1 << 0,
            Self::Node(_) => 1 << 1,
            Self::Relationship(_) => 1 << 2,
            Self::Path(_) => 1 << 4,
            Self::Point(_) => 1 << 5,
            Self::Datetime(_) => 1 << 6,
            Self::Date(_) => 1 << 7,
            Self::Time(_) => 1 << 8,
            Self::Duration(_) => 1 << 10,
            Self::VecF32(_) => 1 << 18,

            Self::Arc(inner) => inner.order(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DisjointOrNull {
    Disjoint,
    ComparedNull,
    NaN,
    None,
}

pub trait CompareValue {
    fn compare_value(
        &self,
        other: &Self,
    ) -> (Ordering, DisjointOrNull);
}

impl CompareValue for Value {
    fn compare_value(
        &self,
        b: &Self,
    ) -> (Ordering, DisjointOrNull) {
        match (self, b) {
            (Self::Bool(a), Self::Bool(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Float(a), Self::Float(b)) => compare_floats(*a, *b),
            (Self::String(a), Self::String(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::List(a), Self::List(b)) | (Self::Path(a), Self::Path(b)) => {
                Self::compare_list(a, b)
            }
            (Self::Map(a), Self::Map(b)) => Self::compare_map(a, b),
            (Self::Node(a), Self::Node(b)) => (a.cmp(b), DisjointOrNull::None),
            (Self::Relationship(rel_a), Self::Relationship(rel_b)) => {
                (rel_a.0.cmp(&rel_b.0), DisjointOrNull::None)
            }
            (Self::Point(a), Self::Point(b)) => match a.longitude.partial_cmp(&b.longitude) {
                Some(Ordering::Equal) => a
                    .latitude
                    .partial_cmp(&b.latitude)
                    .map_or((Ordering::Less, DisjointOrNull::NaN), |ord| {
                        (ord, DisjointOrNull::None)
                    }),
                Some(ord) => (ord, DisjointOrNull::None),
                None => (Ordering::Less, DisjointOrNull::NaN),
            },
            (Self::Int(a), Self::Int(b))
            | (Self::Datetime(a), Self::Datetime(b))
            | (Self::Date(a), Self::Date(b))
            | (Self::Time(a), Self::Time(b))
            | (Self::Duration(a), Self::Duration(b)) => (a.cmp(b), DisjointOrNull::None),
            // the inputs have different type - compare them if they
            // are both numerics of differing types
            (Self::Int(i), Self::Float(f)) => compare_floats(*i as f64, *f),
            (Self::Float(f), Self::Int(i)) => compare_floats(*f, *i as f64),
            (Self::Null, _) | (_, Self::Null) => {
                (self.order().cmp(&b.order()), DisjointOrNull::ComparedNull)
            }
            _ => (self.order().cmp(&b.order()), DisjointOrNull::Disjoint),
        }
    }
}

pub trait ValueTypeOf {
    fn value_of_type(
        &self,
        arg_type: &Type,
    ) -> Option<(Type, Type)>;
}

impl ValueTypeOf for Value {
    fn value_of_type(
        &self,
        arg_type: &Type,
    ) -> Option<(Type, Type)> {
        match (self, arg_type) {
            (Self::List(vs), Type::List(ty)) => {
                for v in vs {
                    if let Some(res) = v.value_of_type(ty) {
                        return Some(res);
                    }
                }
                None
            }
            (Self::Null, Type::Null)
            | (Self::Bool(_), Type::Bool)
            | (Self::Int(_), Type::Int)
            | (Self::Float(_), Type::Float)
            | (Self::String(_), Type::String)
            | (Self::Map(_), Type::Map)
            | (Self::Node(_), Type::Node)
            | (Self::Relationship(_), Type::Relationship)
            | (Self::Path(_), Type::Path)
            | (Self::Point(_), Type::Point)
            | (Value::Datetime(_), Type::Datetime)
            | (Value::Date(_), Type::Date)
            | (Value::Time(_), Type::Time)
            | (Value::Duration(_), Type::Duration)
            | (_, Type::Any) => None,
            (Self::Arc(inner), ty) => {
                // If the inner value is a Rc, we need to check its type
                inner.value_of_type(ty)
            }
            (v, Type::Optional(ty)) => v.value_of_type(ty),
            (v, Type::Union(tys)) => {
                for ty in tys {
                    v.value_of_type(ty)?;
                }
                Some((v.get_type(), Type::Union(tys.clone())))
            }
            (v, e) => Some((v.get_type(), e.clone())),
        }
    }
}

pub trait ValueGetType {
    fn get_type(&self) -> Type;
}

impl ValueGetType for Value {
    fn get_type(&self) -> Type {
        match self {
            Self::Null => Type::Null,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
            Self::List(_) => Type::List(Box::new(Type::Any)),
            Self::Map(_) => Type::Map,
            Self::Node(_) => Type::Node,
            Self::Relationship(_) => Type::Relationship,
            Self::Path(_) => Type::Path,
            Self::VecF32(_) => Type::VecF32,
            Self::Point(_) => Type::Point,
            Self::Datetime(_) => Type::Datetime,
            Self::Date(_) => Type::Date,
            Self::Time(_) => Type::Time,
            Self::Duration(_) => Type::Duration,
            Self::Arc(inner) => inner.get_type(),
        }
    }
}

impl Value {
    pub(crate) fn name(&self) -> String {
        match self {
            Self::Null => String::from("Null"),
            Self::Bool(_) => String::from("Boolean"),
            Self::Int(_) => String::from("Integer"),
            Self::Float(_) => String::from("Float"),
            Self::String(_) => String::from("String"),
            Self::List(_) => String::from("List"),
            Self::Map(_) => String::from("Map"),
            Self::Node(_) => String::from("Node"),
            Self::Relationship(_) => String::from("Edge"),
            Self::Path(_) => String::from("Path"),
            Self::VecF32(_) => String::from("VecF32"),
            Self::Point(_) => String::from("Point"),
            Self::Datetime(_) => String::from("Datetime"),
            Self::Date(_) => String::from("Date"),
            Self::Time(_) => String::from("Time"),
            Self::Duration(_) => String::from("Duration"),
            Self::Arc(inner) => inner.name(),
        }
    }

    /// Convert Value to JSON string representation
    pub fn to_json_string(
        &self,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> String {
        struct JsonWrapper<'a> {
            value: &'a Value,
            runtime: &'a crate::runtime::runtime::Runtime,
        }

        impl fmt::Display for JsonWrapper<'_> {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                self.value.fmt_json(f, self.runtime)
            }
        }

        JsonWrapper {
            value: self,
            runtime,
        }
        .to_string()
    }

    fn compare_list<T: CompareValue>(
        a: &[T],
        b: &[T],
    ) -> (Ordering, DisjointOrNull) {
        let len_a = a.len();
        let len_b = b.len();
        if len_a == 0 && len_b == 0 {
            return (Ordering::Equal, DisjointOrNull::None);
        }
        let min_len = len_a.min(len_b);

        let mut first_not_equal = Ordering::Equal;
        let mut null_counter: usize = 0;
        let mut not_equal_counter: usize = 0;

        for (a_value, b_value) in a.iter().zip(b) {
            let (compare_result, disjoint_or_null) = a_value.compare_value(b_value);
            if disjoint_or_null != DisjointOrNull::None {
                if disjoint_or_null == DisjointOrNull::ComparedNull {
                    null_counter += 1;
                }
                not_equal_counter += 1;
                if first_not_equal == Ordering::Equal {
                    first_not_equal = compare_result;
                }
            } else if compare_result != Ordering::Equal {
                not_equal_counter += 1;
                if first_not_equal == Ordering::Equal {
                    first_not_equal = compare_result;
                }
            }
        }

        // if all the elements in the shared range yielded false comparisons
        if not_equal_counter == min_len && null_counter < not_equal_counter {
            return (first_not_equal, DisjointOrNull::None);
        }

        // if there was a null comparison on non-disjoint arrays
        if null_counter > 0 && len_a == len_b {
            return (first_not_equal, DisjointOrNull::ComparedNull);
        }

        // if there was a difference in some member, without any null compare
        if first_not_equal != Ordering::Equal {
            return (first_not_equal, DisjointOrNull::None);
        }

        (len_a.cmp(&len_b), DisjointOrNull::None)
    }

    fn compare_map(
        a: &OrderMap<Arc<String>, Self>,
        b: &OrderMap<Arc<String>, Self>,
    ) -> (Ordering, DisjointOrNull) {
        let a_key_count = a.len();
        let b_key_count = b.len();
        if a_key_count != b_key_count {
            return (a_key_count.cmp(&b_key_count), DisjointOrNull::None);
        }

        // sort keys
        let mut a_keys: Vec<&Arc<String>> = a.keys().collect();
        a_keys.sort();
        let mut b_keys: Vec<&Arc<String>> = b.keys().collect();
        b_keys.sort();

        // iterate over keys count
        for (a_key, b_key) in a_keys.iter().zip(b_keys) {
            if *a_key != b_key {
                return ((*a_key).cmp(b_key), DisjointOrNull::None);
            }
        }

        // iterate over values
        for key in a_keys {
            let a_value = &a[key];
            let b_value = &b[key];
            let (compare_result, disjoint_or_null) = a_value.compare_value(b_value);
            if disjoint_or_null == DisjointOrNull::ComparedNull
                || disjoint_or_null == DisjointOrNull::Disjoint
            {
                return (Ordering::Equal, disjoint_or_null);
            } else if compare_result != Ordering::Equal {
                return (compare_result, disjoint_or_null);
            }
        }
        (Ordering::Equal, DisjointOrNull::None)
    }
}

impl DisplayJson for Value {
    fn fmt_json(
        &self,
        f: &mut fmt::Formatter<'_>,
        runtime: &crate::runtime::runtime::Runtime,
    ) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => {
                if fl.is_nan() || fl.is_infinite() {
                    write!(f, "null")
                } else {
                    write!(f, "{fl}")
                }
            }
            Self::String(s) => write_json_string(f, s),
            Self::List(list) => {
                write!(f, "[")?;
                for (i, v) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "]")
            }
            Self::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write_json_string(f, k)?;
                    write!(f, ":")?;
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "}}")
            }
            Self::Node(id) => write_node_json(f, runtime, *id, true),
            Self::Relationship(rel) => {
                let (rel_id, start_id, end_id) = **rel;
                let rel_id_u64 = u64::from(rel_id);
                let properties = runtime.get_relationship_attrs(rel_id);
                let type_name = runtime
                    .get_relationship_type(rel_id)
                    .unwrap_or_else(|| Arc::new(String::new()));

                write!(
                    f,
                    r#"{{"type":"relationship","id":{rel_id_u64},"relationship":"#
                )?;
                write_json_string(f, &type_name)?;
                write!(f, r#","properties":{{"#)?;

                for (i, (k, v)) in properties.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write_json_string(f, k)?;
                    write!(f, ":")?;
                    v.fmt_json(f, runtime)?;
                }

                write!(f, r#"}},"start":"#)?;
                write_node_json(f, runtime, start_id, false)?;
                write!(f, r#","end":"#)?;
                write_node_json(f, runtime, end_id, false)?;
                write!(f, "}}")
            }
            Self::Path(values) => {
                write!(f, "[")?;
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    v.fmt_json(f, runtime)?;
                }
                write!(f, "]")
            }
            Self::VecF32(vec) => {
                write!(f, "[")?;
                for (i, fl) in vec.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    if fl.is_nan() || fl.is_infinite() {
                        write!(f, "null")?;
                    } else {
                        write!(f, "{fl}")?;
                    }
                }
                write!(f, "]")
            }
            Self::Point(point) => {
                write!(f, r#"{{"crs":"wgs-84","latitude":"#)?;
                write!(f, "{:.6}", f64::from(point.latitude))?;
                write!(f, r#","longitude":"#)?;
                write!(f, "{:.6}", f64::from(point.longitude))?;
                write!(f, r#","height": null}}"#)
            }
            Self::Datetime(ts) => {
                let formatted = Self::format_datetime(*ts);
                write_json_string(f, &formatted)
            }
            Self::Date(ts) => {
                let formatted = Self::format_date(*ts);
                write_json_string(f, &formatted)
            }
            Self::Time(ts) => {
                let formatted = Self::format_time(*ts);
                write_json_string(f, &formatted)
            }
            Self::Duration(dur) => {
                let formatted = Self::format_duration(*dur);
                write_json_string(f, &formatted)
            }
            Self::Arc(inner) => inner.fmt_json(f, runtime),
        }
    }
}

/// Write a string in JSON format with proper escaping
fn write_json_string(
    f: &mut fmt::Formatter<'_>,
    s: &str,
) -> fmt::Result {
    write!(f, "\"")?;
    for chunk in escape_str(s) {
        write!(f, "{chunk}")?;
    }
    write!(f, "\"")
}

/// Write a node in JSON format with or without the "type" field
fn write_node_json(
    f: &mut fmt::Formatter<'_>,
    runtime: &crate::runtime::runtime::Runtime,
    id: NodeId,
    include_type: bool,
) -> fmt::Result {
    let node_id = u64::from(id);
    let labels = runtime.get_node_labels(id);
    let properties = runtime.get_node_attrs(id);

    write!(f, "{{")?;

    if include_type {
        write!(f, r#""type":"node","#)?;
    }

    write!(f, r#""id":{node_id},"labels":["#)?;

    for (i, label) in labels.iter().enumerate() {
        if i > 0 {
            write!(f, ",")?;
        }
        write_json_string(f, label)?;
    }

    write!(f, r#"],"properties":{{"#)?;

    for (i, (k, v)) in properties.iter().enumerate() {
        if i > 0 {
            write!(f, ",")?;
        }
        write_json_string(f, k)?;
        write!(f, ":")?;
        v.fmt_json(f, runtime)?;
    }

    write!(f, "}}}}")
}

pub trait Contains {
    fn contains(
        &self,
        value: Value,
    ) -> Value;
}

impl Contains for ThinVec<Value> {
    fn contains(
        &self,
        value: Value,
    ) -> Value {
        let mut is_null = false;
        for item in self {
            let (res, dis) = value.compare_value(item);
            is_null = is_null || dis == DisjointOrNull::ComparedNull;
            if res == Ordering::Equal {
                return if dis == DisjointOrNull::ComparedNull {
                    Value::Null
                } else {
                    Value::Bool(true)
                };
            }
        }
        if is_null {
            Value::Null
        } else {
            Value::Bool(false)
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        let (ordering, disjoint_or_null) = self.compare_value(other);
        if disjoint_or_null == DisjointOrNull::ComparedNull {
            None
        } else {
            Some(ordering)
        }
    }
}

fn compare_floats(
    a: f64,
    b: f64,
) -> (Ordering, DisjointOrNull) {
    match a.partial_cmp(&b) {
        Some(Ordering::Equal) => (Ordering::Equal, DisjointOrNull::None),
        Some(Ordering::Less) => (Ordering::Less, DisjointOrNull::None),
        Some(Ordering::Greater) => (Ordering::Greater, DisjointOrNull::None),
        None => (Ordering::Less, DisjointOrNull::NaN),
    }
}
#[derive(Default, Debug)]
pub struct ValuesDeduper {
    seen: RefCell<HashSet<u64>>,
}

impl ValuesDeduper {
    #[must_use]
    pub fn is_seen(
        &self,
        values: &[Value],
    ) -> bool {
        let mut hasher = DefaultHasher::new();
        values.hash(&mut hasher);
        let hash = hasher.finish();
        self.check_and_insert_hash(hash)
    }

    #[must_use]
    pub fn has_hash(
        &self,
        hash: u64,
    ) -> bool {
        self.check_and_insert_hash(hash)
    }

    fn check_and_insert_hash(
        &self,
        hash: u64,
    ) -> bool {
        let mut seen = self.seen.borrow_mut();
        if seen.contains(&hash) {
            true
        } else {
            seen.insert(hash);
            false
        }
    }
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum ValueTypeTag {
    Null = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    List = 5,
    Map = 6,
    VecF32 = 7,
    Point = 8,
    Datetime = 9,
    Date = 10,
    Time = 11,
    Duration = 12,
    Arc = 13,
}

impl Value {
    /// Serializes this value to a byte vector.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.write_bytes(&mut buf);
        buf
    }

    fn write_bytes(
        &self,
        buf: &mut Vec<u8>,
    ) {
        match self {
            Self::Null => buf.push(ValueTypeTag::Null.into()),
            Self::Bool(b) => {
                buf.push(ValueTypeTag::Bool.into());
                buf.push(u8::from(*b));
            }
            Self::Int(i) => {
                buf.push(ValueTypeTag::Int.into());
                buf.extend_from_slice(&i.to_be_bytes());
            }
            Self::Float(f) => {
                buf.push(ValueTypeTag::Float.into());
                buf.extend_from_slice(&f.to_be_bytes());
            }
            Self::String(s) => {
                buf.push(ValueTypeTag::String.into());
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
                buf.extend_from_slice(bytes);
            }
            Self::List(list) => {
                buf.push(ValueTypeTag::List.into());
                buf.extend_from_slice(&(list.len() as u32).to_be_bytes());
                for v in list {
                    v.write_bytes(buf);
                }
            }
            Self::Map(map) => {
                buf.push(ValueTypeTag::Map.into());
                buf.extend_from_slice(&(map.len() as u32).to_be_bytes());
                for (k, v) in map.iter() {
                    let kb = k.as_bytes();
                    buf.extend_from_slice(&(kb.len() as u32).to_be_bytes());
                    buf.extend_from_slice(kb);
                    v.write_bytes(buf);
                }
            }
            Self::VecF32(vec) => {
                buf.push(ValueTypeTag::VecF32.into());
                buf.extend_from_slice(&(vec.len() as u32).to_be_bytes());
                for f in vec {
                    buf.extend_from_slice(&f.to_be_bytes());
                }
            }
            Self::Point(p) => {
                buf.push(ValueTypeTag::Point.into());
                buf.extend_from_slice(&p.latitude.to_be_bytes());
                buf.extend_from_slice(&p.longitude.to_be_bytes());
            }
            Self::Datetime(ts) => {
                buf.push(ValueTypeTag::Datetime.into());
                buf.extend_from_slice(&ts.to_be_bytes());
            }
            Self::Date(ts) => {
                buf.push(ValueTypeTag::Date.into());
                buf.extend_from_slice(&ts.to_be_bytes());
            }
            Self::Time(ns) => {
                buf.push(ValueTypeTag::Time.into());
                buf.extend_from_slice(&ns.to_be_bytes());
            }
            Self::Duration(ms) => {
                buf.push(ValueTypeTag::Duration.into());
                buf.extend_from_slice(&ms.to_be_bytes());
            }
            Self::Arc(inner) => {
                buf.push(ValueTypeTag::Arc.into());
                inner.write_bytes(buf);
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// Deserializes a value from a byte slice, returning the value and bytes consumed.
    #[must_use]
    pub fn from_bytes(data: &[u8]) -> Option<(Self, usize)> {
        if data.is_empty() {
            return None;
        }
        let tag = data[0];
        let rest = &data[1..];
        match ValueTypeTag::try_from(tag).ok()? {
            ValueTypeTag::Null => Some((Self::Null, 1)),
            ValueTypeTag::Bool => Some((Self::Bool(rest.first().copied()? != 0), 2)),
            ValueTypeTag::Int => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Int(i64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::Float => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Float(f64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::String => {
                let len = u32::from_be_bytes(rest.get(..4)?.try_into().ok()?) as usize;
                let s = std::str::from_utf8(rest.get(4..4 + len)?).ok()?;
                Some((Self::String(Arc::new(s.to_owned())), 1 + 4 + len))
            }
            ValueTypeTag::List => {
                let len = u32::from_be_bytes(rest.get(..4)?.try_into().ok()?) as usize;
                let mut list = ThinVec::with_capacity(len);
                let mut offset = 5;
                for _ in 0..len {
                    let (v, consumed) = Self::from_bytes(&data[offset..])?;
                    list.push(v);
                    offset += consumed;
                }
                Some((Self::List(list), offset))
            }
            ValueTypeTag::Map => {
                let len = u32::from_be_bytes(rest.get(..4)?.try_into().ok()?) as usize;
                let mut map = OrderMap::default();
                let mut offset = 5;
                for _ in 0..len {
                    let klen =
                        u32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?) as usize;
                    offset += 4;
                    let k = std::str::from_utf8(data.get(offset..offset + klen)?).ok()?;
                    offset += klen;
                    let (v, consumed) = Self::from_bytes(&data[offset..])?;
                    map.insert(Arc::new(k.to_owned()), v);
                    offset += consumed;
                }
                Some((Self::Map(map), offset))
            }
            ValueTypeTag::VecF32 => {
                let len = u32::from_be_bytes(rest.get(..4)?.try_into().ok()?) as usize;
                let mut vec = ThinVec::with_capacity(len);
                let mut offset = 5;
                for _ in 0..len {
                    let bytes: [u8; 4] = data.get(offset..offset + 4)?.try_into().ok()?;
                    vec.push(f32::from_be_bytes(bytes));
                    offset += 4;
                }
                Some((Self::VecF32(vec), offset))
            }
            ValueTypeTag::Point => {
                let lat: [u8; 4] = rest.get(..4)?.try_into().ok()?;
                let lon: [u8; 4] = rest.get(4..8)?.try_into().ok()?;
                Some((
                    Self::Point(Point::new(f32::from_be_bytes(lat), f32::from_be_bytes(lon))),
                    9,
                ))
            }
            ValueTypeTag::Datetime => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Datetime(i64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::Date => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Date(i64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::Time => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Time(i64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::Duration => {
                let bytes: [u8; 8] = rest.get(..8)?.try_into().ok()?;
                Some((Self::Duration(i64::from_be_bytes(bytes)), 9))
            }
            ValueTypeTag::Arc => {
                let (v, consumed) = Self::from_bytes(rest)?;
                Some((Self::Arc(Arc::new(v)), 1 + consumed))
            }
        }
    }
}
