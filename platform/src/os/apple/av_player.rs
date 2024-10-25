use {
    std::sync::atomic::{AtomicBool, Ordering},
    std::sync::Arc,
    crate::{
        makepad_live_id::*,
        os::apple::apple_sys::*,
    },
};

use crate::ns_dict;

pub fn define_video_output_delegate() -> *const Class {
    extern fn new_frame_available(this: &Object, _: Sel, _notification: ObjcId) {
        unsafe {
            println!("new_frame_available");
            let frame_available_ptr: *mut c_void = *this.get_ivar("frame_available");
            let frame_available = &*(frame_available_ptr as *const Arc<AtomicBool>);
            frame_available.store(true, Ordering::Release);
            
            // Post notification to trigger redraw
            let notification_center: ObjcId = msg_send![class!(NSNotificationCenter), defaultCenter];
            let name = NSString::new("MakepadVideoFrameAvailable");
            let () = msg_send![notification_center,
                postNotificationName:name
                object:nil
            ];
        }
    }
    
    extern fn dealloc(this: &Object, _: Sel) {
        unsafe {
            let frame_available_ptr: *mut c_void = *this.get_ivar("frame_available");
            // Drop the Arc
            let _ = Box::from_raw(frame_available_ptr as *mut Arc<AtomicBool>);
            let _: () = msg_send![super(this, class!(NSObject)), dealloc];
        }
    }
    
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("VideoOutputDelegate", superclass).unwrap();
    
    unsafe {
        decl.add_method(
            sel!(newFrameAvailable:),
            new_frame_available as extern fn(&Object, Sel, ObjcId)
        );
        decl.add_method(
            sel!(dealloc),
            dealloc as extern fn(&Object, Sel)
        );
    }
    
    decl.add_ivar::<*mut c_void>("frame_available");
    
    return decl.register();
}


#[derive(Default)]
pub struct AvPlayerAccess {
    pub player: Option<RcObjcId>,
    pub player_item: Option<RcObjcId>,
    pub output: Option<RcObjcId>,
    pub texture_cache: Option<RcObjcId>,
    frame_available: Arc<AtomicBool>,
    texture_handle: Option<u32>,
}

impl AvPlayerAccess {
    pub fn new() -> Self {
        Self {
            player: None,
            player_item: None,
            output: None,
            texture_cache: None,
            frame_available: Arc::new(AtomicBool::new(false)),
            texture_handle: None,
        }
    }

    pub fn prepare_playback(&mut self, _video_id: LiveId, source: &str, texture_handle: u32, metal_device: ObjcId) -> Result<(), String> {
        println!("prepare_playback start");
        unsafe {
            // Create AVPlayer and AVPlayerItem
            let url = nsurl_from_str(source);
            let asset: ObjcId = msg_send![class!(AVURLAsset), assetWithURL: url];
            if asset.is_null() {
                return Err("Failed to create AVURLAsset".to_string());
            }
            
            let player_item: ObjcId = msg_send![class!(AVPlayerItem), playerItemWithAsset: asset];
            if player_item.is_null() {
                return Err("Failed to create AVPlayerItem".to_string());
            }
            
            let player: ObjcId = msg_send![class!(AVPlayer), playerWithPlayerItem:player_item];
            if player.is_null() {
                return Err("Failed to create AVPlayer".to_string());
            }
            
            println!("player: {:?}", player);
            
            // Create output with Metal texture compatibility
            println!("Creating output settings dictionary");
            let pixel_format_key: ObjcId = msg_send![class!(NSNumber), numberWithUnsignedInt:kCVPixelFormatType_32BGRA];
            let metal_compatibility_key: ObjcId = msg_send![class!(NSNumber), numberWithBool:YES];
            
            let output_settings = ns_dict! {
                kCVPixelBufferPixelFormatTypeKey => pixel_format_key,
                kCVPixelBufferMetalCompatibilityKey => metal_compatibility_key,
            };
            
            if output_settings.is_null() {
                return Err("Failed to create output settings dictionary".to_string());
            }
            
            println!("Output settings dictionary created: {:?}", output_settings);
            
            let output: ObjcId = msg_send![class!(AVPlayerItemVideoOutput), alloc];
            if output.is_null() {
                return Err("Failed to allocate AVPlayerItemVideoOutput".to_string());
            }
            
            let output: ObjcId = msg_send![output, initWithOutputSettings:output_settings];
            if output.is_null() {
                return Err("Failed to initialize AVPlayerItemVideoOutput".to_string());
            }
            
            println!("output: {:?}", output);
            
            // Create Metal texture cache
            let mut cache: CVMetalTextureCacheRef = std::ptr::null_mut();
            let result = CVMetalTextureCacheCreate(
                kCFAllocatorDefault,
                std::ptr::null(),
                metal_device,
                std::ptr::null(),
                &mut cache,
            );
            
            if result != kCVReturnSuccess {
                return Err("Failed to create Metal texture cache".to_string());
            }

            // Add output to player item
            let () = msg_send![player_item, addOutput:output];
            
            println!("player_item: {:?}", player_item);
            self.player = Some(RcObjcId::from_owned(NonNull::new(player).unwrap()));
            self.player_item = Some(RcObjcId::from_owned(NonNull::new(player_item).unwrap()));
            self.output = Some(RcObjcId::from_owned(NonNull::new(output).unwrap()));
            self.texture_cache = Some(RcObjcId::from_owned(NonNull::new(cache as ObjcId).unwrap()));
            self.texture_handle = Some(texture_handle);
            
            let () = msg_send![player, play];

            println!("prepare_playback end");

            Ok(())
        }
    }

    pub fn update_if_needed(&mut self) -> Result<bool, String> {
        println!("update_if_needed");
        if !self.frame_available.load(Ordering::Acquire) {
            println!("frame_available false");
            return Ok(false);
        }
    
        unsafe {
            if let (Some(output), Some(cache)) = (&self.output, &self.texture_cache) {
                println!("output and cache");
                let current_time: CMTime = msg_send![self.player.as_ref().unwrap().as_id(), currentTime];
                
                // Check if new frame is available
                let has_new_frame: BOOL = msg_send![output.as_id(),
                    hasNewPixelBufferForItemTime:current_time
                    itemTimeForDisplay:std::ptr::null_mut::<CMTime>()];
                
                if has_new_frame == YES {
                    // Get the pixel buffer
                    let pixel_buffer: CVPixelBufferRef = msg_send![output.as_id(), 
                        copyPixelBufferForItemTime:current_time
                        itemTimeForDisplay:std::ptr::null_mut::<CMTime>()];
                    
                    if !pixel_buffer.is_null() {
                        // The texture_handle is already bound to our Metal texture
                        // CVMetalTextureCacheCreateTextureFromImage will update it
                        let width = CVPixelBufferGetWidth(pixel_buffer);
                        let height = CVPixelBufferGetHeight(pixel_buffer);
                        
                        // Update the Metal texture
                        let mut texture_ref: CVMetalTextureRef = std::ptr::null_mut();
                        CVMetalTextureCacheCreateTextureFromImage(
                            kCFAllocatorDefault,
                            cache.as_id() as CVMetalTextureCacheRef,
                            pixel_buffer,
                            std::ptr::null(),
                            MTLPixelFormat::BGRA8Unorm,
                            width,
                            height,
                            0,
                            &mut texture_ref
                        );
                        
                        CVPixelBufferRelease(pixel_buffer);
                        if !texture_ref.is_null() {
                            CFRelease(texture_ref as CFTypeRef);  // Use CFRelease instead of CVMetalTextureRelease
                        }
                    }
                    
                    self.frame_available.store(false, Ordering::Release);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
