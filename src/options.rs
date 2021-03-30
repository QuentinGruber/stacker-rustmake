pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "w", long = "width", default_value = "450")]
    pub width: i32,
    #[structopt(short = "h", long = "height", default_value = "700")]
    pub height: i32,
    #[structopt(long = "fps", default_value = "10")]
    pub fps: u32,
}

impl Opt {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Opt::from_args()
    }
    pub fn open_window(&self, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
        let (rl, thread) = raylib::init()
            .size(self.width, self.height)
            .title(name)
            .build();
        (rl, thread)
    }
}
