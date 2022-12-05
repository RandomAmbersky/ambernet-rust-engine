mod amberskynet;
mod ext_core;

pub async fn run() {
    let asn = amberskynet::new();
    let ext = ext_core::ExternalCore::new();
    amberskynet::run(&asn, &ext)
}
