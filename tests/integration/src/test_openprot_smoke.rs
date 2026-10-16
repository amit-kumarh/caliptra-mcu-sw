// Licensed under the Apache-2.0 license

//! One-off smoke test: boot an OpenPRoT (pw_kernel) FPGA image as the MCU
//! runtime and wait for it to report exit status via the debug FIFO.

#[cfg(feature = "fpga_realtime")]
#[cfg(test)]
pub mod test {
    use crate::test::{finish_runtime_hw_model, start_runtime_hw_model, TestParams, TEST_LOCK};

    #[test]
    pub fn test_openprot_fpga_smoke() {
        let _lock = TEST_LOCK.lock().unwrap();

        let mut hw = start_runtime_hw_model(TestParams {
            custom_mcu_runtime: Some(include_bytes!("openprot_unittest_runner.bin").to_vec()),
            ..Default::default()
        });

        let result = finish_runtime_hw_model(&mut hw);
        assert_eq!(0, result, "OpenPRoT unittest_runner did not exit successfully");
    }
}
