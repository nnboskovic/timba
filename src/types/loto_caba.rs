pub struct LotoResult {
    result: Vec<u32>,
}

impl Default for LotoResult {
    fn default() -> Self {
        LotoResult {
            result: vec![0, 0, 0, 0, 0, 0, 0],
        }
    }
}

struct DatosSorteo {
    version: usize,
    entidad: String,
    juego: String,
    sorteo: usize,
    fecha_sorteo: String,
    hora_sorteo: String,
    fecha_prescripcion: String,
    fecha_proximo_sorteo: String,
    hora_proximo_sorteo: String,
    pozo_estimado: String,
    extractos: Vec<Extracto>,
}

struct Extracto {
    modalidad: String,
    suerte: Vec<usize>,
    ganadores: Vec<usize>,
    premios: Vec<f32>,
    pozos: Vec<f32>,
}

struct Autoridades {
    autoridades: Vec<Autoridad>,
}

struct Autoridad {
    autoridad_tipo: String,
    autoridad_nombre: String,
}