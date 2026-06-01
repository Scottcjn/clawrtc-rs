use clawrtc::Wallet;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rtc_wallet"));
    fs::create_dir_all(&out_dir)?;

    let wallet = Wallet::generate();
    let address = wallet.address();
    let public_key_hex = wallet.public_key_hex();
    let private_key_hex = wallet.private_key_hex();

    let wallet_json = format!(
        concat!(
            "{{\n",
            "  \"network\": \"RustChain RTC\",\n",
            "  \"address\": \"{}\",\n",
            "  \"public_key_hex\": \"{}\",\n",
            "  \"private_key_hex\": \"{}\",\n",
            "  \"private_key_warning\": \"Keep this private key secret. Anyone with it can control this wallet.\"\n",
            "}}\n"
        ),
        address, public_key_hex, private_key_hex
    );

    fs::write(out_dir.join("rtc_wallet_private.json"), wallet_json)?;
    fs::write(out_dir.join("rtc_address.txt"), format!("{}\n", address))?;
    fs::write(
        out_dir.join("README.txt"),
        concat!(
            "RustChain RTC wallet generated from local clawrtc source.\n",
            "\n",
            "Files:\n",
            "- rtc_address.txt: public RTC address for bounty comments.\n",
            "- rtc_wallet_private.json: private key backup. Keep private.\n",
            "\n",
            "Address derivation used by clawrtc:\n",
            "RTC + first 40 hex chars of SHA-256(Ed25519 public key).\n"
        ),
    )?;

    println!("{}", address);
    Ok(())
}
