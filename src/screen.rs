use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

static mut SCREEN_WIDTH: usize = 0;
static mut SCREEN_HEIGHT: usize = 0;
static mut STRIDE: usize = 0;
static mut PIXEL_FORMAT: PixelFormat = PixelFormat::Rgb;
static mut FB_PTR: *mut u8 = core::ptr::null::<u8>() as *mut u8;

pub struct Screen;

impl Screen {
    pub fn init() {
        // Open Graphics Output Protocol
        let gop_handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>()
            .expect("[!] Failed to get graphics output protocol handle.");
        let mut gop = uefi::boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)
            .expect("[!] Failed to open graphics output protocol.");

        // Get GOP information
        let mode_info = gop.current_mode_info();
        let (screen_width, screen_height) = mode_info.resolution();
        let stride = mode_info.stride();
        let pixel_format = mode_info.pixel_format();

        // Get frame buffer address
        let mut fb = gop.frame_buffer();
        let fb_ptr = fb.as_mut_ptr();

        unsafe {
            SCREEN_WIDTH = screen_width;
            SCREEN_HEIGHT = screen_height;
            STRIDE = stride;
            PIXEL_FORMAT = pixel_format;
            FB_PTR = fb_ptr;
        }
    }

    pub fn draw_pixel(x: usize, y: usize, color: (u8, u8, u8)) {
        unsafe {
            // Pixel coordinates not valid
            if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
                return;
            }

            let pixel_offset = y * STRIDE + x;

            let pixel_ptr = FB_PTR.add(pixel_offset * 4); // Calculate pixel bytes position in frame buffer

            // Draw pixel color based on pixel format
            match PIXEL_FORMAT {
                PixelFormat::Rgb => {
                    *pixel_ptr.add(0) = color.0;
                    *pixel_ptr.add(1) = color.1;
                    *pixel_ptr.add(2) = color.2;
                    *pixel_ptr.add(3) = 0;
                }
                PixelFormat::Bgr => {
                    *pixel_ptr.add(0) = color.2;
                    *pixel_ptr.add(1) = color.1;
                    *pixel_ptr.add(2) = color.0;
                    *pixel_ptr.add(3) = 0;
                }
                _ => {}
            }
        }
    }
}
