use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

#[derive(Copy, Clone, Debug)]
pub struct QuinielaNumber {
    pub(crate) number: &'static str,
    pub(crate) lore: &'static str,
}

impl QuinielaNumber {
    pub fn new(number: &'static str, lore: &'static str) -> QuinielaNumber {
        QuinielaNumber { number, lore }
    }

    pub fn populate_from_csv() -> Result<Vec<QuinielaNumber>, anyhow::Error> {
        let mut nums: Vec<QuinielaNumber> = vec![];

        let path = "../../resources/quiniela.csv";

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            let mut split_line: Split<&str> = line.unwrap().clone().split(",");
            let number = split_line.nth(0).unwrap();
            let lore = split_line.nth(1).unwrap();

            nums.push(QuinielaNumber::new(number, lore));
        }

        Ok(nums)
    }

    pub fn populate() -> Vec<QuinielaNumber> {
        let mut nums: Vec<QuinielaNumber> = vec![];

        nums.push(QuinielaNumber::new("00", "Los huevos"));
        nums.push(QuinielaNumber::new("01", "El agua"));
        nums.push(QuinielaNumber::new("02", "El niño"));
        nums.push(QuinielaNumber::new("03", "San Cono"));
        nums.push(QuinielaNumber::new("04", "La cama"));
        nums.push(QuinielaNumber::new("05", "El gato"));
        nums.push(QuinielaNumber::new("06", "El perro"));
        nums.push(QuinielaNumber::new("07", "El revólver"));
        nums.push(QuinielaNumber::new("08", "El incendio"));
        nums.push(QuinielaNumber::new("09", "El arroyo"));
        nums.push(QuinielaNumber::new("10", "El cañón"));
        nums.push(QuinielaNumber::new("11", "El minero"));
        nums.push(QuinielaNumber::new("12", "El soldado"));
        nums.push(QuinielaNumber::new("13", "La yeta"));
        nums.push(QuinielaNumber::new("14", "El borracho"));
        nums.push(QuinielaNumber::new("15", "La niña bonita"));
        nums.push(QuinielaNumber::new("16", "El anillo"));
        nums.push(QuinielaNumber::new("17", "La desgracia"));
        nums.push(QuinielaNumber::new("18", "La sangre"));
        nums.push(QuinielaNumber::new("19", "El pescado"));
        nums.push(QuinielaNumber::new("20", "La fiesta"));
        nums.push(QuinielaNumber::new("21", "La mujer"));
        nums.push(QuinielaNumber::new("22", "El loco"));
        nums.push(QuinielaNumber::new("23", "El cocinero"));
        nums.push(QuinielaNumber::new("24", "El caballo"));
        nums.push(QuinielaNumber::new("25", "La gallina"));
        nums.push(QuinielaNumber::new("26", "La misa"));
        nums.push(QuinielaNumber::new("27", "El peine"));
        nums.push(QuinielaNumber::new("28", "Los cerros"));
        nums.push(QuinielaNumber::new("29", "San Pedro"));
        nums.push(QuinielaNumber::new("30", "Santa Rosa"));
        nums.push(QuinielaNumber::new("31", "La luz"));
        nums.push(QuinielaNumber::new("32", "El dinero"));
        nums.push(QuinielaNumber::new("33", "Cristo"));
        nums.push(QuinielaNumber::new("34", "La cabeza"));
        nums.push(QuinielaNumber::new("35", "El pajarito"));
        nums.push(QuinielaNumber::new("36", "La castaña"));
        nums.push(QuinielaNumber::new("37", "El dentista"));
        nums.push(QuinielaNumber::new("38", "Las piedras"));
        nums.push(QuinielaNumber::new("39", "La lluvia"));
        nums.push(QuinielaNumber::new("40", "El cura"));
        nums.push(QuinielaNumber::new("41", "El cuchillo"));
        nums.push(QuinielaNumber::new("42", "Las zapatillas"));
        nums.push(QuinielaNumber::new("43", "El balcón"));
        nums.push(QuinielaNumber::new("44", "La cárcel"));
        nums.push(QuinielaNumber::new("45", "El vino"));
        nums.push(QuinielaNumber::new("46", "Los tomates"));
        nums.push(QuinielaNumber::new("47", "El muerto"));
        nums.push(QuinielaNumber::new("48", "Muerto que habla"));
        nums.push(QuinielaNumber::new("49", "La carne"));
        nums.push(QuinielaNumber::new("50", "El pan"));
        nums.push(QuinielaNumber::new("51", "El serrucho"));
        nums.push(QuinielaNumber::new("52", "Madre e hijo"));
        nums.push(QuinielaNumber::new("53", "El barco"));
        nums.push(QuinielaNumber::new("54", "La vaca"));
        nums.push(QuinielaNumber::new("55", "La música"));
        nums.push(QuinielaNumber::new("56", "La caída"));
        nums.push(QuinielaNumber::new("57", "El jorobado"));
        nums.push(QuinielaNumber::new("58", "El ahogado"));
        nums.push(QuinielaNumber::new("59", "Las plantas"));
        nums.push(QuinielaNumber::new("60", "La virgen"));
        nums.push(QuinielaNumber::new("61", "Escopetas"));
        nums.push(QuinielaNumber::new("62", "La inundación"));
        nums.push(QuinielaNumber::new("63", "El casamiento"));
        nums.push(QuinielaNumber::new("64", "El llanto"));
        nums.push(QuinielaNumber::new("65", "El cazador"));
        nums.push(QuinielaNumber::new("66", "Las lombrices"));
        nums.push(QuinielaNumber::new("67", "La mordida"));
        nums.push(QuinielaNumber::new("68", "Los sobrinos"));
        nums.push(QuinielaNumber::new("69", "Los vicios"));
        nums.push(QuinielaNumber::new("70", "Muerto sueño"));
        nums.push(QuinielaNumber::new("71", "Excremento"));
        nums.push(QuinielaNumber::new("72", "La sorpresa"));
        nums.push(QuinielaNumber::new("73", "El hospital"));
        nums.push(QuinielaNumber::new("74", "Gente negra"));
        nums.push(QuinielaNumber::new("75", "Los besos"));
        nums.push(QuinielaNumber::new("76", "Las llamas"));
        nums.push(QuinielaNumber::new("77", "Pierna mujer"));
        nums.push(QuinielaNumber::new("78", "La ramera"));
        nums.push(QuinielaNumber::new("79", "El ladrón"));
        nums.push(QuinielaNumber::new("80", "La bocha"));
        nums.push(QuinielaNumber::new("81", "Las flores"));
        nums.push(QuinielaNumber::new("82", "La pelea"));
        nums.push(QuinielaNumber::new("83", "Mal tiempo"));
        nums.push(QuinielaNumber::new("84", "La iglesia"));
        nums.push(QuinielaNumber::new("85", "La linterna"));
        nums.push(QuinielaNumber::new("86", "El humo"));
        nums.push(QuinielaNumber::new("87", "Los piojos"));
        nums.push(QuinielaNumber::new("88", "El Papa"));
        nums.push(QuinielaNumber::new("89", "La rata"));
        nums.push(QuinielaNumber::new("90", "El miedo"));
        nums.push(QuinielaNumber::new("91", "El excusado"));
        nums.push(QuinielaNumber::new("92", "El médico"));
        nums.push(QuinielaNumber::new("93", "El enamorado"));
        nums.push(QuinielaNumber::new("94", "El cementerio"));
        nums.push(QuinielaNumber::new("95", "Los anteojos"));
        nums.push(QuinielaNumber::new("96", "El marido"));
        nums.push(QuinielaNumber::new("97", "La mesa"));
        nums.push(QuinielaNumber::new("98", "La lavandera"));
        nums.push(QuinielaNumber::new("99", "El hermano"));

        nums
    }
}
