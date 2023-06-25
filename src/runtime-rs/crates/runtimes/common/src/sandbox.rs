// Copyright (c) 2019-2022 Alibaba Cloud
// Copyright (c) 2019-2022 Ant Group
//
// SPDX-License-Identifier: Apache-2.0
//

use std::collections::{hash_map::RandomState, HashMap};

use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone)]
pub struct SandboxNetworkEnv {
    pub netns: Option<String>,
    pub network_created: bool,
}

#[derive(Clone)]
pub struct CreateOpt {
    pub hostname: String,
    pub dns: Vec<String>,
    pub network_env: SandboxNetworkEnv,
    pub annotations: HashMap<String, String, RandomState>,
}

#[derive(Default, Clone, Debug)]
pub struct SandboxStatus {
    pub sandbox_id: String,
    pub pid: u32,
    pub state: String,
    pub info: std::collections::HashMap<String, String>,
    pub create_at: std::time::Duration,
    pub exited_at: std::time::Duration,
}

#[async_trait]
pub trait Sandbox: Send + Sync {
    async fn create(&self, opt: &CreateOpt) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn run(
        &self,
        dns: Vec<String>,
        spec: &oci::Spec,
        state: &oci::State,
        network_env: SandboxNetworkEnv,
    ) -> Result<()>;
    async fn status(&self) -> Result<SandboxStatus>;
    async fn wait(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn cleanup(&self) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;

    // utils
    async fn set_iptables(&self, is_ipv6: bool, data: Vec<u8>) -> Result<Vec<u8>>;
    async fn get_iptables(&self, is_ipv6: bool) -> Result<Vec<u8>>;
    async fn direct_volume_stats(&self, volume_path: &str) -> Result<String>;
    async fn direct_volume_resize(&self, resize_req: agent::ResizeVolumeRequest) -> Result<()>;
    async fn agent_sock(&self) -> Result<String>;
}
