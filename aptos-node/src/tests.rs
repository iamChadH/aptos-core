// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::setup_environment_and_start_node;
use aptos_config::config::{NodeConfig, WaypointConfig};
use aptos_temppath::TempPath;
use aptos_types::waypoint::Waypoint;

#[test]
#[should_panic(expected = "Validator networks must always have mutual_authentication enabled!")]
fn test_mutual_authentication_validators() {
    // Create a default node config for the validator
    let temp_path = TempPath::new();
    let mut node_config = NodeConfig::default_for_validator();
    node_config.set_data_dir(temp_path.path().to_path_buf());
    node_config.base.waypoint = WaypointConfig::FromConfig(Waypoint::default());

    // Disable mutual authentication for the config
    let validator_network = node_config.validator_network.as_mut().unwrap();
    validator_network.mutual_authentication = false;

    // Starting the node should panic
    setup_environment_and_start_node(node_config, None, None).unwrap();
}

#[cfg(feature = "check-vm-features")]
#[test]
fn test_aptos_vm_does_not_have_test_natives() {
    aptos_vm::natives::assert_no_test_natives(crate::utils::ERROR_MSG_BAD_FEATURE_FLAGS)
}
