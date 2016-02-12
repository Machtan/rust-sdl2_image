extern crate sdl2;
extern crate libc;
extern crate sdl2_sys as sys;

use libc::{c_int, c_char};
use std::ffi::CString;
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::get_error;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_image")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_image")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_image")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

/// The supported image formats of sdl2_image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    Cur,
    Ico,
    Bmp,
    Pnm,
    Xpm,
    Xcf,
    Pcx,
    Gif,
    Jpg,
    Tif,
    Png,
    Tga,
    Lbm,
    Xv,
    Webp,
}

/// Context manager for sdl2_image used to access functionality and handle
/// initialization and clean-up.
#[must_use]
#[derive(Debug)]
pub struct Sdl2ImageContext;

impl Drop for Sdl2ImageContext {
    fn drop(&mut self) {
        unsafe { ffi::IMG_Quit(); }
    }
}

impl Sdl2ImageContext {
    /// Loads the image in the given file with the given renderer.
    pub fn load_texture<'a>(&self, renderer: &Renderer<'a>, file: &Path)
            -> Result<Texture, String> {
        
        let c_str = CString::new(file.to_str().unwrap()).unwrap();
        unsafe {
            let raw = ffi::IMG_LoadTexture(
                renderer.raw(), c_str.as_ptr() as *const _
            );
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Texture::from_ll(renderer, raw))
            }
        }
    }
    
    /// Loads a surface from the given file.
    pub fn load_surface_from_file<'a>(&self, file: &Path)
            -> Result<Surface<'a>, String> {
        
        let c_str = CString::new(file.to_str().unwrap()).unwrap();
        unsafe {
            let raw = ffi::IMG_Load(c_str.as_ptr() as *const _);
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
    }
    
    /// Loads a surface from the given xpm array.
    pub fn load_surface_from_xpm_array<'a>(&self, xpm: *const *const i8)
            -> Result<Surface<'a>, String> {
        
        unsafe {
            let raw = ffi::IMG_ReadXPMFromArray(xpm as *const *const c_char);
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
    }
    
    /// Loads an image with the specified format from the given RWops object.
    pub fn load_surface_from_rwops<'a>(&self, rwops: 
            &RWops<'a>, format: Option<ImageFormat>)
            -> Result<Surface<'a>, String> {
        
        use self::ImageFormat::*;
        unsafe { 
            let raw = match format {
                None => {
                    ffi::IMG_Load_RW(rwops.raw(), 0)
                },
                Some(format) => {
                    match format {
                        Cur     => ffi::IMG_LoadCUR_RW(rwops.raw()),
                        Ico     => ffi::IMG_LoadICO_RW(rwops.raw()),
                        Bmp     => ffi::IMG_LoadBMP_RW(rwops.raw()),
                        Pnm     => ffi::IMG_LoadPNM_RW(rwops.raw()),
                        Xpm     => ffi::IMG_LoadXPM_RW(rwops.raw()),
                        Xcf     => ffi::IMG_LoadXCF_RW(rwops.raw()),
                        Pcx     => ffi::IMG_LoadPCX_RW(rwops.raw()),
                        Gif     => ffi::IMG_LoadGIF_RW(rwops.raw()),
                        Jpg     => ffi::IMG_LoadJPG_RW(rwops.raw()),
                        Tif     => ffi::IMG_LoadTIF_RW(rwops.raw()),
                        Png     => ffi::IMG_LoadPNG_RW(rwops.raw()),
                        Tga     => ffi::IMG_LoadTGA_RW(rwops.raw()),
                        Lbm     => ffi::IMG_LoadLBM_RW(rwops.raw()),
                        Xv      => ffi::IMG_LoadXV_RW(rwops.raw()),
                        Webp    => ffi::IMG_LoadWEBP_RW(rwops.raw()),
                    }
                },
            };
            if (raw as *mut ()).is_null() {
                Err(get_error())
            } else {
                Ok(Surface::from_ll(raw))
            }
        }
        
    }
    
    /// Saves the given surface to the given file.
    pub fn save_surface_to_file<'a>(&self, surface: &Surface<'a>, filename: &Path) 
            -> Result<(), String> {
        
        let c_str = CString::new(filename.to_str().unwrap()).unwrap();
        let result = unsafe {
            ffi::IMG_SavePNG(surface.raw(), c_str.as_ptr() as *const _)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
    
    /// Saves the given surface to the given RWops object.
    pub fn save_surface_to_rw<'a>(&self, surface: &Surface<'a>, dst: &mut RWops)
            -> Result<(), String> {
        
        let result = unsafe {
            ffi::IMG_SavePNG_RW(surface.raw(), dst.raw(), 0)
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
    
    /// Returns whether the given RWops object contains a given image format.
    /// NOTE: TGA is not supported for this function.
    pub fn rwops_is<'a>(&self, rwops: &RWops<'a>, format: ImageFormat) -> bool {
        use self::ImageFormat::*;
        unsafe {
            let res = match format {
                Cur     => ffi::IMG_isCUR(rwops.raw()),
                Ico     => ffi::IMG_isICO(rwops.raw()),
                Bmp     => ffi::IMG_isBMP(rwops.raw()),
                Pnm     => ffi::IMG_isPNM(rwops.raw()),
                Xpm     => ffi::IMG_isXPM(rwops.raw()),
                Xcf     => ffi::IMG_isXCF(rwops.raw()),
                Pcx     => ffi::IMG_isPCX(rwops.raw()),
                Gif     => ffi::IMG_isGIF(rwops.raw()),
                Jpg     => ffi::IMG_isJPG(rwops.raw()),
                Tif     => ffi::IMG_isTIF(rwops.raw()),
                Png     => ffi::IMG_isPNG(rwops.raw()),
                Tga     => 0,
                Lbm     => ffi::IMG_isLBM(rwops.raw()),
                Xv      => ffi::IMG_isXV(rwops.raw()) ,
                Webp    => ffi::IMG_isWEBP(rwops.raw()),
            };
            res == 1
        }
    }
}

/// A partial initialization of the library.
/// Call 'finish' to create the context.
#[must_use]
pub struct PartialInit {
    flags: u32,
}

impl PartialInit {
    /// Activate PNG support for the library.
    pub fn png(mut self) -> PartialInit {
        self.flags |= ffi::IMG_INIT_PNG as u32;
        self
    }
    
    /// Activate JPEG support for the library.
    pub fn jpg(mut self) -> PartialInit {
        self.flags |= ffi::IMG_INIT_JPG as u32;
        self
    }
    
    /// Activate TIF support for the library.
    pub fn tif(mut self) -> PartialInit {
        self.flags |= ffi::IMG_INIT_TIF as u32;
        self
    }
    
    /// Activate WEBP support for the library.
    pub fn webp(mut self) -> PartialInit {
        self.flags |= ffi::IMG_INIT_WEBP as u32;
        self
    }
    
    /// Finishes the initialization. 
    /// Errors if the requested functionalities could not be initialized.
    pub fn finish(self) -> Result<Sdl2ImageContext, String> {
        let supported_flags = unsafe {
            ffi::IMG_Init(self.flags as c_int) as u32
        };
        if (self.flags & supported_flags) == 0 {
            // According to docs, error message text is not always set
            let mut error = get_error();
            if &error == "" {
                let uninitialized_flags = supported_flags ^ self.flags;
                let mut message = String::new();
                if (uninitialized_flags & ffi::IMG_INIT_TIF as u32) != 0 {
                    message.push_str("TIF, ");
                }
                if (uninitialized_flags & ffi::IMG_INIT_PNG as u32) != 0 {
                    message.push_str("PNG, ");
                }
                if (uninitialized_flags & ffi::IMG_INIT_JPG as u32) != 0 {
                    message.push_str("JPG, ");
                }
                if (uninitialized_flags & ffi::IMG_INIT_WEBP as u32) != 0 {
                    message.push_str("WEBP, ");
                }
                message = message.chars().take(message.len()-1).collect();
                error = format!(
                    "Could not initialize the following parts: {}", 
                    message
                );
            }
            Err(error)
        } else {
            Ok(Sdl2ImageContext)
        }
    }
}

/// Initializes SDL2_image with InitFlags.
/// If not every flag is set it returns an error
pub fn init() -> PartialInit {
    PartialInit { flags: 0 }
}

/// Returns the version of the dynamically linked SDL_image library
pub fn get_linked_version() -> Version {
    unsafe {
        Version::from_ll(*ffi::IMG_Linked_Version())
    }
}
