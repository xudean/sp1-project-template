//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

// use zktls_att_verification::*;
use zktls_att_verification::attestation_data::{PublicData,AttestationConfig};
use serde_json;
use serde_json::Value;


use alloy_sol_types::SolType;
use fibonacci_lib::{AttestationDataVerified};

pub fn main() {
    // Read an input to the program.
    // let n = sp1_zkvm::io::read::<u32>();
    let json_string = sp1_zkvm::io::read::<String>();

    // 解析 public_data 并添加错误处理
    let public_data: Result<PublicData, _> = serde_json::from_str(&json_string);
    //print public_data
    // println!("{}",json_string);
    let config_str = r#"{
        "attestor_addr": "0xdb736b13e2f522dbe18b2015d0291e4b193d8ef6",
        "url": ["https://api.x.com/1.1/account/settings.json"],
        "path": ["$.screen_name"]
    }"#;
    let attestation_config: Result<AttestationConfig, _> = serde_json::from_str(config_str);

    // // 验证 attestation data 根据 attestation config
    if let (Ok(public_data), Ok(attestation_config)) = (public_data, attestation_config) {
        public_data.verify_without_aes(&attestation_config);
        println!("verify ok");
        println!("public_data.data:{}", public_data.data);

        let v: Value = serde_json::from_str(&public_data.data).unwrap();

        // 取出字段并转换为 Vec<u8>
        let screen_name_str = v["screen_name"].as_str().unwrap();
        let screen_name = screen_name_str.as_bytes().to_vec(); // 转为 Vec<u8>

        let data_source = b"x".to_vec(); // 模拟一个 data_source 字段，也转 Vec<u8>

        // println!("screen_name: {}", screen_name_str);

        // ABI 编码结构体
        let bytes = AttestationDataVerified::abi_encode(&AttestationDataVerified {
            screen_name: screen_name.into(),
            data_source: data_source.into(),
        });
        // println!("abi encoded bytes: 0x{}", hex::encode(bytes));
        sp1_zkvm::io::commit_slice(&bytes);
    }else{
        panic!("verify failed");
    }
}
