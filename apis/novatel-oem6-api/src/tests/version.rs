//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Test requesting and processing system version information
//
// Note: Also including test cases for non-good responses. get_response() is a private
// function, so can't be explicitly tested.

use super::*;
use messages::commands::ResponseID;
use messages::ReceiverStatusFlags;

#[test]
fn test_request_version_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x25, 0x0, 0x0,
        0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x8E, 0x33, 0x3C,
    ]);

    mock.read.set_output(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x80, 0x20, 0x6, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB,
        0x6E, 0x5D, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4F,
        0x4B, 0x92, 0x8F, 0x77, 0x4A,
    ]);

    let oem = mock_new!(mock);

    assert_eq!(oem.request_version(), Ok(()));
}

#[test]
fn test_request_version_bad_no_response() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x25, 0x0, 0x0,
        0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x8E, 0x33, 0x3C,
    ]);

    let oem = mock_new!(mock);

    assert_eq!(oem.request_version().unwrap_err(), OEMError::NoResponse);
}

#[test]
fn test_request_version_bad_response_crc() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x25, 0x0, 0x0,
        0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x8E, 0x33, 0x3C,
    ]);

    mock.read.set_output(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x80, 0x20, 0x6, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB,
        0x6E, 0x5D, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4F,
        0x4B, 0x92, 0x8F, 0x77, 0x4B,
    ]);

    let oem = mock_new!(mock);

    assert_eq!(oem.request_version().unwrap_err(), OEMError::NoResponse);
}

#[test]
fn test_request_version_fail_response() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x25, 0x0, 0x0,
        0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x8E, 0x33, 0x3C,
    ]);

    mock.read.set_output(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x80, 0x20, 0x15, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB,
        0x6E, 0x5D, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1F, 0x0, 0x0, 0x0, 0x4D,
        0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x69, 0x6D, 0x65, 0x64, 0x20, 0x6F, 0x75,
        0x74, 0xCB, 0xE5, 0x83, 0x92,
    ]);

    let oem = mock_new!(mock);

    assert_eq!(
        oem.request_version().unwrap_err(),
        OEMError::CommandError {
            id: ResponseID::Timeout,
            description: "Message timed out".to_owned(),
        }
    );
}

#[test]
fn test_get_version() {
    let mut mock = MockStream::default();

    mock.read.set_output(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x25, 0x0, 0x0, 0x20, 0x70, 0x0, 0x0, 0x0, 0x7D, 0x78, 0xD1, 0xB,
        0x38, 0x5E, 0xC9, 0x9, 0x0, 0x0, 0x48, 0x0, 0x81, 0x36, 0xFA, 0x33, 0x1, 0x0, 0x0, 0x0,
        0x1, 0x0, 0x0, 0x0, 0x47, 0x31, 0x53, 0x42, 0x30, 0x47, 0x54, 0x54, 0x30, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x42, 0x4A, 0x59, 0x41, 0x31, 0x35, 0x31, 0x32, 0x30, 0x30, 0x33, 0x38,
        0x48, 0x0, 0x0, 0x0, 0x4F, 0x45, 0x4D, 0x36, 0x31, 0x35, 0x2D, 0x32, 0x2E, 0x30, 0x30, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x4F, 0x45, 0x4D, 0x30, 0x36, 0x30, 0x36, 0x30, 0x30, 0x52, 0x4E, 0x30,
        0x30, 0x30, 0x30, 0x0, 0x4F, 0x45, 0x4D, 0x30, 0x36, 0x30, 0x32, 0x30, 0x31, 0x52, 0x42,
        0x30, 0x30, 0x30, 0x30, 0x0, 0x32, 0x30, 0x31, 0x35, 0x2F, 0x4A, 0x61, 0x6E, 0x2F, 0x32,
        0x38, 0x0, 0x31, 0x35, 0x3A, 0x32, 0x37, 0x3A, 0x32, 0x39, 0x0, 0x0, 0x0, 0x0, 0xC6, 0x5E,
        0x86, 0x47,
    ]);

    let oem = mock_new!(mock);

    let expected: Log = Log::Version(VersionLog {
        recv_status: ReceiverStatusFlags::CLOCK_MODEL_INVALID
            | ReceiverStatusFlags::POSITION_SOLUTION_INVALID,
        time_status: 120,
        week: 3025,
        ms: 164191800,
        num_components: 1,
        components: vec![Component {
            comp_type: 1,
            model: "G1SB0GTT0".to_owned(),
            serial_num: "BJYA15120038H".to_owned(),
            hw_version: "OEM615-2.00".to_owned(),
            sw_version: "OEM060600RN0000".to_owned(),
            boot_version: "OEM060201RB0000".to_owned(),
            compile_date: "2015/Jan/28".to_owned(),
            compile_time: "15:27:29".to_owned(),
        }],
    });

    assert_eq!(oem.get_log().unwrap(), expected);
}
