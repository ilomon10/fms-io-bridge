use std::convert::TryInto;

#[derive(Debug)]
struct UbxNavPvt {
    itow: u32,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
    valid: u8,
    fix_type: u8,
    num_svs: u8,
    lon: f64,
    lat: f64,
    height: i32,
    h_msl: i32,
    h_acc: u32,
    v_acc: u32,
    g_speed: i32,
    head_mot: f64,
}

impl UbxNavPvt {
    fn from_payload(payload: &[u8]) -> Option<Self> {
        if payload.len() < 92 {
            return None; // Ensure the payload is at least 92 bytes
        }

        Some(UbxNavPvt {
            itow: u32::from_le_bytes(payload[0..4].try_into().unwrap()),
            year: u16::from_le_bytes(payload[4..6].try_into().unwrap()),
            month: payload[6],
            day: payload[7],
            hour: payload[8],
            min: payload[9],
            sec: payload[10],
            valid: payload[11],
            fix_type: payload[20],
            num_svs: payload[21],
            lon: i32::from_le_bytes(payload[24..28].try_into().unwrap()) as f64 * 1e-7,
            lat: i32::from_le_bytes(payload[28..32].try_into().unwrap()) as f64 * 1e-7,
            height: i32::from_le_bytes(payload[32..36].try_into().unwrap()),
            h_msl: i32::from_le_bytes(payload[36..40].try_into().unwrap()),
            h_acc: u32::from_le_bytes(payload[40..44].try_into().unwrap()),
            v_acc: u32::from_le_bytes(payload[44..48].try_into().unwrap()),
            g_speed: i32::from_le_bytes(payload[60..64].try_into().unwrap()),
            head_mot: i32::from_le_bytes(payload[64..68].try_into().unwrap()) as f64 * 1e-5,
        })
    }
}

fn main() {
    let payload = [
        208, 72, 135, 0, 233, 7, 2, 23, 2, 27, 28, 7, 37, 25, 0, 0, 28, 38, 254, 255,
        0, 0, 10, 0, 30, 145, 24, 74, 139, 207, 86, 0, 59, 232, 10, 0, 37, 232, 9, 0,
        104, 11, 161, 4, 198, 14, 143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 29, 146, 240, 1, 147, 78, 0, 0, 128, 168, 18, 1, 15, 39, 0, 0, 132, 211, 17, 0
    ];

    match UbxNavPvt::from_payload(&payload) {
        Some(parsed) => println!("{:#?}", parsed),
        None => println!("Invalid payload"),
    }
}
