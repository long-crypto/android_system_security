// Copyright 2020, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate implements the Android Keystore 2.0 service.

mod crypto;
pub mod database;
pub mod error;
pub mod globals;
/// Internal Representation of Key Parameter and convenience functions.
pub mod key_parameter;
pub mod operation;
pub mod permission;
pub mod security_level;
pub mod service;
pub mod utils;
