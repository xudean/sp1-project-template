//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
// use fibonacci_lib::PublicValuesStruct;
use fibonacci_lib::{AttestationDataVerified};

use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::fs;
use hex;



/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "20")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let json_data = fs::read("data/attestation_data_without_aes.json").expect("Failed to read file");
    let json_str = String::from_utf8(json_data).expect("Invalid UTF-8 data");

    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // stdin.write(&args.n);
    stdin.write(&json_str);


    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = AttestationDataVerified::abi_decode(output.as_slice()).unwrap();
        let AttestationDataVerified { screen_name, data_source } = decoded;
        // screen_name 和 data_source 是 Bytes 类型，转换成 String：
        let screen_name_str = String::from_utf8(screen_name.into()).expect("Invalid UTF-8 in screen_name");
        let data_source_str = String::from_utf8(data_source.into()).expect("Invalid UTF-8 in data_source");

        println!("screen_name: {}", screen_name_str);
        println!("data_source: {}", data_source_str);

        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);
        // Print the verification key.
        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");
        //print proof
        // let proof_hex = format!("0x{}", hex::encode(proof.bytes()));
        // println!("proof (hex): {}", proof_hex);

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
        let public_values = hex::encode(&proof.public_values.as_slice());
        println!("public_values (hex): {}", &public_values);
        let decoded = AttestationDataVerified::abi_decode(proof.public_values.as_slice()).unwrap();
        let AttestationDataVerified { screen_name, data_source } = decoded;
        // screen_name 和 data_source 是 Bytes 类型，转换成 String：
        let screen_name_str = String::from_utf8(screen_name.into()).expect("Invalid UTF-8 in screen_name");
        let data_source_str = String::from_utf8(data_source.into()).expect("Invalid UTF-8 in data_source");

        println!("screen_name: {}", screen_name_str);
        println!("data_source: {}", data_source_str);
    }
}
