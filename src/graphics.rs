use crate::physics::vec2::Vec2;
use sdl2::sys::{
    gfx::primitives::{boxColor, circleColor, filledCircleColor, filledPolygonColor, lineColor},
    SDL_CreateRenderer, SDL_CreateWindow, SDL_DestroyRenderer, SDL_DestroyWindow,
    SDL_GetCurrentDisplayMode, SDL_Init, SDL_Quit, SDL_Rect, SDL_RenderClear, SDL_RenderCopyEx,
    SDL_RenderPresent, SDL_Renderer, SDL_RendererFlags, SDL_RendererFlip, SDL_SetRenderDrawColor,
    SDL_Texture, SDL_Window, SDL_WindowFlags, Uint32, SDL_INIT_EVERYTHING,
};
use std::{mem::MaybeUninit, ptr};

static mut RENDERER: *mut SDL_Renderer = ptr::null_mut();
static mut WINDOW: *mut SDL_Window = ptr::null_mut();
static mut WINDOW_HEIGHT: i32 = 0;
static mut WINDOW_WIDTH: i32 = 0;

pub fn width() -> i32 {
    unsafe { WINDOW_WIDTH }
}
pub fn height() -> i32 {
    unsafe { WINDOW_HEIGHT }
}

pub fn open_window() -> bool {
    unsafe {
        if SDL_Init(SDL_INIT_EVERYTHING) != 0 {
            panic!("Error inialising SDL")
        }

        /* -------------------------------------------------------------------------- */
        // Original C++ code looks like this:
        // SDL_DisplayMode display_mode;
        // SDL_GetCurrentDisplayMode(0, &display_mode);

        // In Rust:
        // let display_mode: SDL_DisplayMode;
        // SDL_GetCurrentDisplayMode(0, &mut display_mode as *mut SDL_DisplayMode);

        // But Rust does not let you work with uninitialized variables. Gives the error:
        // Error0381 https://doc.rust-lang.org/error-index.html#E0381

        // This solution uses MaybeUninit as a workaround
        // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html

        // There may be better ways to do this that I'm not aware of. But this complies correctly.

        let mut uninitialized_display_mode = MaybeUninit::uninit();
        SDL_GetCurrentDisplayMode(0, uninitialized_display_mode.as_mut_ptr());
        let display_mode = uninitialized_display_mode.assume_init();
        /* -------------------------------------------------------------------------- */

        WINDOW_WIDTH = display_mode.w;
        WINDOW_HEIGHT = display_mode.h;

        // Original C++ code passes in null for the `title` param
        // This replicates it by creating a null raw pointer and passing it in
        let null_raw_pointer: *const i8 = ptr::null();
        WINDOW = SDL_CreateWindow(
            null_raw_pointer,
            0,
            0,
            0,
            WINDOW_WIDTH,
            // C++ code uses SDL_WINDOW_BORDERLESS, which works fine on my machine.
            // For some reason, that doesn't work on my machine when using Rust.
            // I need to use the FULLSCREEN flag to get the screen to show properly.
            // Possibly because I'm running WSL2?
            SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as Uint32,
        );

        if WINDOW.is_null() {
            panic!("Error creating SDL window")
        }

        RENDERER = SDL_CreateRenderer(
            WINDOW,
            -1,
            SDL_RendererFlags::SDL_RENDERER_ACCELERATED as Uint32
                | SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as Uint32,
        );
        true
    }
}

pub fn clear_screen(color: u32) {
    // Rust requires explicitly casting the u32 to a u8
    let r = (color >> 16) as u8;
    let g = (color >> 8) as u8;
    let b = color as u8;
    unsafe {
        SDL_SetRenderDrawColor(RENDERER, r, g, b, 255);
        SDL_RenderClear(RENDERER);
    }
}

pub fn render_frame() {
    unsafe {
        SDL_RenderPresent(RENDERER);
    }
}
pub fn draw_line(x0: i16, y0: i16, x1: i16, y1: i16, color: u32) {
    unsafe {
        lineColor(RENDERER, x0, y0, x1, y1, color);
    }
}

// C++ code uses floats, but I'm not sure why since it gets casted to a Sint16 anyway
// x - width / 2.0,

// I'm just going to use ints the whole time
// x - width / 2

// I'm also going to cast angles to i16

pub fn draw_circle(x: i16, y: i16, radius: i16, angle: f32, color: u32) {
    unsafe {
        circleColor(RENDERER, x, y, radius, color);
        lineColor(
            RENDERER,
            x,
            y,
            x + angle.cos() as i16 * radius,
            y + angle.sin() as i16 * radius,
            color,
        );
    }
}
pub fn draw_fill_circle(x: i16, y: i16, radius: i16, angle: f32, color: u32) {
    unsafe {
        filledCircleColor(RENDERER, x, y, radius, color);
    }
}
pub fn draw_rect(x: i16, y: i16, width: i16, height: i16, color: u32) {
    unsafe {
        lineColor(
            RENDERER,
            x - width / 2,
            y - height / 2,
            x + width / 2,
            y - height / 2,
            color,
        );
        lineColor(
            RENDERER,
            x + width / 2,
            y - height / 2,
            x + width / 2,
            y + height / 2,
            color,
        );
        lineColor(
            RENDERER,
            x + width / 2,
            y + height / 2,
            x - width / 2,
            y + height / 2,
            color,
        );
        lineColor(
            RENDERER,
            x - width / 2,
            y + height / 2,
            x - width / 2,
            y - height / 2,
            color,
        );
    }
}

pub fn draw_fill_rect(x: i16, y: i16, width: i16, height: i16, color: u32) {
    unsafe {
        boxColor(
            RENDERER,
            x - width / 2,
            y - height / 2,
            x + width / 2,
            y + height / 2,
            color,
        );
    }
}

// vertices might need to be a slice
pub fn draw_polygon(x: i16, y: i16, vertices: Vec<Vec2>, color: u32) {
    unsafe {
        let vertices_len = vertices.len();

        for i in 0..vertices_len {
            let current_index = i;
            let next_index = (i + 1) % vertices_len;
            lineColor(
                RENDERER,
                vertices[current_index].x as i16,
                vertices[current_index].y as i16,
                vertices[next_index].x as i16,
                vertices[next_index].y as i16,
                color,
            );
        }

        filledCircleColor(RENDERER, x, y, 1, color);
    }
}
pub fn draw_fill_polygon(x: i16, y: i16, vertices: Vec<Vec2>, color: u32) {
    unsafe {
        let mut vx: Vec<i16> = vec![];
        let mut vy: Vec<i16> = vec![];

        let vertices_len = vertices.len();

        // Original code uses two seperate loops to do this - not sure why
        for vertex in vertices {
            vx.push(vertex.x as i16);
            vy.push(vertex.y as i16);
        }

        filledPolygonColor(RENDERER, &vx[0], &vy[0], vertices_len as i32, color);
        filledCircleColor(RENDERER, x, y, 1, 0xFF000000);
    }
}
// This takes i32 as parameters since SDL_Rect wants c_ints (which are i32s).
pub fn draw_texture(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    rotation: f32,
    texture: *mut SDL_Texture,
) {
    unsafe {
        let dst_rect = SDL_Rect {
            x: x - (width / 2),
            y: y - (height / 2),
            w: width,
            h: height,
        };
        let rotation_deg = rotation * 57.2958;

        // Pass in null raw pointers
        SDL_RenderCopyEx(
            RENDERER,
            texture,
            ptr::null(),
            &dst_rect,
            rotation_deg as f64,
            ptr::null(),
            SDL_RendererFlip::SDL_FLIP_NONE,
        );
    }
}

pub fn close_window() {
    unsafe {
        SDL_DestroyRenderer(RENDERER);
        SDL_DestroyWindow(WINDOW);
        SDL_Quit()
    }
}
