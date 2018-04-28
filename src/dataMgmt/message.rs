use evo_sys::prog::prog::Program;


#[derive(Debug)]
pub struct EvalResult {
    pub genome: Program,
    pub map_location: Option<(usize, usize)>,
    pub signal: Message,
}

impl EvalResult {
    pub fn new(genome: Program) -> EvalResult {
        EvalResult {genome, map_location:None, signal: Message::Cont }
    }

    pub fn quit() -> EvalResult {
        EvalResult {
            genome: Program::new_empty(),
            map_location: None,
            signal: Message::Quit,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Cont,
    Quit,
}