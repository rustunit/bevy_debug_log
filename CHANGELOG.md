# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.3.0] - 2024-12-24

### Changed
* Added level based log filtering (tabs for warn, info, error ..)
* Show amount of logs per level

### Fixed
* Logviewer will no longer panic if a log event arrives after the receiver was dropped

## [0.2.1] - 2024-11-01

### Fixed
* Fixed wasm build

## [0.2.0] - 2024-11-01

### Added
* Logviewer now has buttons for clearing logs and going fullscreen
* Loglines now show timestamps
* Logviewer can now be configured to open automatically when an event of a certain level is received.

### Changed
* Replaced the plugin initialization function `bevy_debug_log::plugin()` with `bevy_debug_log::LogViewerPlugin::default()` 

## [0.1.1] - 2024-10-10

### Added
* Added screenshot to `README.md`

### Changed
* Updated `Cargo.toml` to exclude `assets` directory

## [0.1.0] - 2024-09-01

### Added
* Initial implementation of `bevy_debug_log` with basic logging functionality
