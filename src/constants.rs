use sdl2::sys::SDL_KeyCode;

pub const FPS: i32 = 60;
pub const MILLISECS_PER_FRAME: i32 = 1000 / FPS;
pub const PIXELS_PER_METER: f32 = 200.;

// SDL2 crate gives event.key.keysym.sym as an i32
// This let's me use match arms without having to type convert
pub const SDLK_ESCAPE: i32 = SDL_KeyCode::SDLK_ESCAPE as i32;
pub const SDLK_UP: i32 = SDL_KeyCode::SDLK_UP as i32;
pub const SDLK_DOWN: i32 = SDL_KeyCode::SDLK_DOWN as i32;
pub const SDLK_LEFT: i32 = SDL_KeyCode::SDLK_LEFT as i32;
pub const SDLK_RIGHT: i32 = SDL_KeyCode::SDLK_RIGHT as i32;
