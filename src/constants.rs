use sdl2::sys::{SDL_EventType, SDL_KeyCode};

pub const FPS: i32 = 60;
pub const MILLISECS_PER_FRAME: i32 = 1000 / FPS;
pub const PIXELS_PER_METER: f32 = 200.;

// SDL2 crate gives event._type as a u32
// This let's me use match arms without having to type convert
pub const SDLK_QUIT: u32 = SDL_EventType::SDL_QUIT as u32;
pub const SDLK_KEYDOWN: u32 = SDL_EventType::SDL_KEYDOWN as u32;
pub const SDLK_KEYUP: u32 = SDL_EventType::SDL_KEYUP as u32;
pub const SDL_MOUSEBUTTONDOWN: u32 = SDL_EventType::SDL_MOUSEBUTTONDOWN as u32;
pub const SDL_MOUSEBUTTONUP: u32 = SDL_EventType::SDL_MOUSEBUTTONUP as u32;
pub const SDL_MOUSEMOTION: u32 = SDL_EventType::SDL_MOUSEMOTION as u32;

// SDL2 crate gives event.key.keysym.sym as an i32
pub const SDLK_ESCAPE: i32 = SDL_KeyCode::SDLK_ESCAPE as i32;
pub const SDLK_UP: i32 = SDL_KeyCode::SDLK_UP as i32;
pub const SDLK_DOWN: i32 = SDL_KeyCode::SDLK_DOWN as i32;
pub const SDLK_LEFT: i32 = SDL_KeyCode::SDLK_LEFT as i32;
pub const SDLK_RIGHT: i32 = SDL_KeyCode::SDLK_RIGHT as i32;
pub const SDLK_D: i32 = SDL_KeyCode::SDLK_d as i32;
pub const SDLK_G: i32 = SDL_KeyCode::SDLK_g as i32;
