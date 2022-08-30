use native_windows_derive as nwd;

use nwd::NwgUi;

use crate::consts::BITMAP;

#[derive(Default, NwgUi)]
pub struct SplashScreen {
    #[nwg_control(size: (128, 128), position: (700, 300), center: true, flags: "POPUP|VISIBLE", topmost: true)]
    window: nwg::Window,
    
    #[nwg_resource(source_file: BITMAP)]
    splash: nwg::Bitmap,

    #[nwg_control(size: (128, 128), bitmap: Some(&data.splash) )]
    image_frame: nwg::ImageFrame
}

impl SplashScreen {
    pub fn exit(&self) {
        self.window.close();
    }
}