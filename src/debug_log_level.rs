use core::fmt;

use bevy::color::palettes::css;
use bevy::log::tracing::level_filters::LevelFilter;
use bevy::log::Level;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DebugLogLevel(Level);

impl DebugLogLevel {
    pub const TRACE: Self = Self(Level::TRACE);
    pub const DEBUG: Self = Self(Level::DEBUG);
    pub const INFO: Self = Self(Level::INFO);
    pub const WARN: Self = Self(Level::WARN);
    pub const ERROR: Self = Self(Level::ERROR);

    pub fn title_case(&self) -> String {
        match self.0 {
            Level::TRACE => "Trace",
            Level::DEBUG => "Debug",
            Level::INFO => "Info",
            Level::WARN => "Warn",
            Level::ERROR => "Error",
        }
        .to_string()
    }
}

impl From<Level> for DebugLogLevel {
    fn from(level: Level) -> Self {
        Self(level)
    }
}

impl From<DebugLogLevel> for Level {
    fn from(log_level: DebugLogLevel) -> Self {
        log_level.0
    }
}

impl From<DebugLogLevel> for Srgba {
    fn from(log_level: DebugLogLevel) -> Self {
        match log_level.0 {
            Level::TRACE => css::MEDIUM_ORCHID,
            Level::DEBUG => css::DEEP_SKY_BLUE,
            Level::INFO => css::LIME,
            Level::WARN => css::YELLOW,
            Level::ERROR => css::RED,
        }
    }
}

impl From<DebugLogLevel> for Color {
    fn from(log_level: DebugLogLevel) -> Self {
        let color: Srgba = log_level.into();
        color.into()
    }
}

impl From<DebugLogLevel> for String {
    fn from(log_level: DebugLogLevel) -> Self {
        log_level.0.to_string()
    }
}

impl TryFrom<LevelFilter> for DebugLogLevel {
    type Error = ();

    fn try_from(level_filter: LevelFilter) -> Result<Self, Self::Error> {
        match level_filter {
            LevelFilter::TRACE => Ok(DebugLogLevel(Level::TRACE)),
            LevelFilter::DEBUG => Ok(DebugLogLevel(Level::DEBUG)),
            LevelFilter::INFO => Ok(DebugLogLevel(Level::INFO)),
            LevelFilter::WARN => Ok(DebugLogLevel(Level::WARN)),
            LevelFilter::ERROR => Ok(DebugLogLevel(Level::ERROR)),
            LevelFilter::OFF => Err(()),
        }
    }
}

impl fmt::Display for DebugLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
