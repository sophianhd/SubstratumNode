// Copyright (c) 2017-2018, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.
#![cfg (target_os = "macos")]
extern crate sub_lib;
extern crate dns_utility_lib;

mod utils;

use utils::TestCommand;
use dns_utility_lib::dynamic_store_dns_modifier::StoreWrapper;
use dns_utility_lib::dynamic_store_dns_modifier::StoreWrapperReal;

#[test]
fn macos_subvert_and_revert_integration() {
    let store_wrapper = StoreWrapperReal::new ("integration-test");
    let original_dns_ips = get_current_dns_ips (&store_wrapper);
    assert! (!original_dns_ips.is_empty ());
    assert_ne! (original_dns_ips[0], "127.0.0.1".to_string ());

    let mut subvert_command = TestCommand::start ("dns_utility", vec! ("subvert"));
    let exit_status = subvert_command.wait ();
    assert_eq! (exit_status, Some (0), "{}", subvert_command.output ());

    let subverted_dns_ips = get_current_dns_ips (&store_wrapper);
    assert_eq! (subverted_dns_ips, vec! ("127.0.0.1".to_string ()));

    let mut revert_command = TestCommand::start ("dns_utility", vec! ("revert"));
    let exit_status = revert_command.wait ();
    assert_eq! (exit_status, Some (0), "{}", revert_command.output ());

    let reverted_dns_ips = get_current_dns_ips (&store_wrapper);
    assert_eq! (reverted_dns_ips, original_dns_ips);
}

#[test]
fn store_wrapper_real_returns_none_if_store_does_not_contain_dictionary_at_path_integration () {
    let store_wrapper = StoreWrapperReal::new ("integration-test");

    let result = store_wrapper.get_dictionary_string_cfpl("State:/Booga/Booga");

    assert! (result.is_none ());
}

fn get_current_dns_ips (store_wrapper: &StoreWrapper) -> Vec<String> {
    let state_global_network_ipv4 = store_wrapper.get_dictionary_string_cfpl("State:/Network/Global/IPv4").unwrap ();
    let primary_service_cfpl = state_global_network_ipv4.get("PrimaryService").unwrap ();
    let primary_service_uuid = store_wrapper.cfpl_to_string(primary_service_cfpl).unwrap ();
    let primary_service_path = format!(
        "State:/Network/Service/{}/DNS",
        primary_service_uuid
    );

    let state_network_service_uuid_dns = store_wrapper.get_dictionary_string_cfpl (primary_service_path.as_str ()).unwrap ();
    let server_addresses_cfpl = state_network_service_uuid_dns.get ("ServerAddresses").unwrap ();
    store_wrapper.cfpl_to_vec (server_addresses_cfpl).unwrap ().iter ().map (|cfpl| {
        store_wrapper.cfpl_to_string (cfpl).unwrap ()
    }).collect::<Vec<String>> ()
}