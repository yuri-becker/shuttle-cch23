use std::str::FromStr;

use josekit::jwe::{deserialize_json, RSA_OAEP_256};
use josekit::jwk::Jwk;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, routes, Route};

const KEY: &str = r#"{
  "kty": "RSA",
  "n": "tX4yurmjaH70PEgrVcrq6syvAzWCp3EvLmoYeq4JSQruT3r0fsEN_3iRNQ13VALZSL_k9xidlYEDqhNN6owui3uql6L8UrhmhhOeNOOYI4YOpTa9Yda_yuYFii6o_NrOpHv4LmrMzLzCX7kPW3j4GNiS7vYkwGI0n1mtVGqpYs9jic4GR3Be-kBMgNpZanJk9OA2LKf1cyh2n5LkU9lO15ZszvKfA9u_08A5s62b9_MhjEVlmENUWXGJzZtx-pZMWZwZFjV2KrEoCY3BykmwSSNyhdxN1NKp-l3_plOLop96G621k8tabctHbS565BpmDiKT5rNymXDZWpiYod6gpQ",
  "e": "AQAB",
  "d": "LYTySjzHDC1TKk9bdxAGrU8a0e44z7AmijiX3SULNSOls49-BNB8p0dg-_JdrFdukb13OrYUx-tstNpUn2_7OIaSuadqK4EOTbBb7J3siXRU9gDtrL1EqynX19luDsT-MOjazSGCLhNlmMZ8YI_NgcXHzGE1xH6c_h5qx5Jc22gGP6XRsQLsDioUEJOigmCsma97ZO_x2P9ySm19n1cdx4_FWgUSlchHuObWD7a0ubY4KKDY3DCzF4IRFqVw_qlf7osaiX3-oSPHyaAd4NyM7bhKWDlM2-GBQyYrMItfdRF70LHhAmXDrsvY3CwdgIHuNJIUKbBTJFMHsKhLPWM9vQ",
  "p": "3Zq-rd7P8zN887yet08kOUvzHh8fA6EaDLA54Rpql1-dVMlJ7RbVWzO6x5pEv8HkJliXjF_NNyhBPJklT2i_76VY9yZlasGqG7qsOFBgPUuKqfNrCmTcrKf_KyWQZdKNqicmjtWTOeHZNS3jpqroct80PNSgDCYBsaWIsp2wgxs",
  "q": "0amojC5uKSGblMfpyYqZRY4LBvIGXtAG6vz6mTXmr6O-pMXvxKYZqqVO1Xjqsu6SZuMci5iYQ8hmcfsneSW6aXoDOokPAsejD2ayMqnkAFd1fYqFPo9Gmf4C9zkoMiAbxC2RI6ertIMrOEu0peUR4uJsj5wvFDTyNx2Nbnhz5z8",
  "dp": "Ojy7lafzkF9cnBVaxKPIykH6b1UQanzBAsqhO1Yc4xEeoLSRd_xDL8elc2VIYfiLg8ROd3aJ0NAEbO92TasindEfUzxE9MxWbxkcv2PoFtOuakFtRPsCv2Ea_vTNQOUXk7rcODdKjLCcy4v2wssxcVbVPJNISEkIsu3kwcQNKjc",
  "dq": "Ci-j8KuQzo7DcEcGJLSHHcn43y2DAbg5ndEMm8TyoDXkXT0AmR04wgGmAtkNDgRpOHZwPJf9Tc2-rGr3T_t1QwqafY2LHSd11Jm4rp1yZlHZc2_3aUKsu26L1lcAjO7ianWMR58tyGdXAjUrYaPvaoZ1n8SGxQSNf___jw5rEyE",
  "qi": "Q4HW9O6xx699EHeMOnyBUFxewqpEU_kIY5D36PmGeVznmXaRAPXgp3nylvkxjMALQqU4IQ6gLdqjAi-oHk_G83s2vsVc51G1GC5CKleeNpGk_AsEcOgsg_Z1x57wni5p_YmiUPuAkty6KZugCNQLS_6VM4pDs-uyDybmaFEGHWY"
}
"#;

#[derive(Deserialize, Serialize, Debug)]
struct NaughtyItem {
    name: String,
    email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    name: String,
    email: String,
    message: String,
}

#[get("/")]
async fn get_naughty_list() {
    // perform a request to https://www.codehunt.rs/api/naughty with bearer auth
    let key = Jwk::from_bytes(KEY.as_bytes()).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.codehunt.rs/api/naughty")
        .header("Authorization", "Bearer Lb7bB6PyL1kP0hU2")
        .send()
        .await
        .unwrap();
    let encrypted = response.text().await.unwrap();
    let decrypter = RSA_OAEP_256.decrypter_from_jwk(&key).unwrap();
    let (payload, _) = deserialize_json(&encrypted, &decrypter).unwrap();
    let payload = String::from_utf8(payload).unwrap();
    let naughty_list = serde_json::from_str::<Vec<NaughtyItem>>(&payload).unwrap();
    println!("Naughty list: {:?}", payload);
    let mail_batches = naughty_list
        .iter()
        .map(|naughty| Response {
            name: naughty.name.clone(),
            email: naughty.email.clone(),
            message: format!("Dear {}, it seems this year you've made it onto the Naughty List. But don't worry, everyone makes mistakes. This is a chance to learn and grow. We believe in you and can't wait to see the wonderful things you do next year! Keep shining, and remember, Santa's always cheering for you!", naughty.name),
        })
        .collect::<Vec<_>>()
        .chunks(10)
        .map(|it| it.to_vec())
        .collect::<Vec<_>>();

    for chunks in mail_batches {
        let response = client
            .post("https://www.codehunt.rs/api/naughty")
            .header("Authorization", "Bearer Lb7bB6PyL1kP0hU2")
            .json(&chunks)
            .send()
            .await
            .unwrap();
        println!("{:?}", response.status());
    }
}

pub struct Tiebreaker {}

impl Tiebreaker {
    pub fn routes() -> Vec<Route> {
        routes![get_naughty_list]
    }
}
