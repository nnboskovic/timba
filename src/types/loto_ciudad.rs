struct XmlLotoCiudadWinners {
    winners: u8,
    matches: u8,
}

struct XmlLotoCiudadPrizes {
    prize_name: String,
    prize: f32,
}

struct XmlLotoCiudadExtract {
    contest_type: String, // modalidad
    numbers: Vec<u32>, // <Suerte>
    prizes: Vec<XmlLotoCiudadPrizes>, // pozos
    winners: XmlLotoCiudadWinners,
}

struct XmlLotoCiudadAuthority {
    class: String,
    name: String,
}

struct XmlLotoCiudadResult {
    data: Vec<std::collections::HashMap<String, String>>,
    extracts: Vec<XmlLotoCiudadExtract>,
    authorities: Vec<XmlLotoCiudadAuthority>,
}