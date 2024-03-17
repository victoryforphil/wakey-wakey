use mac_address::get_mac_address;

pub struct NetworkInfo {
    pub mac: String,
}

impl NetworkInfo {
    pub fn fetch() -> NetworkInfo {
        let mac = mac_address::get_mac_address().unwrap().unwrap();
        NetworkInfo {
            mac: mac.to_string(),
        }
    }
}
