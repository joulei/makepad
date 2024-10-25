use crate::os::apple::apple_sys::*;

pub struct VideoPlayer {
    pub player: ObjcId,
    pub player_item: ObjcId,
    pub video_output: ObjcId,
}

impl VideoPlayer {
    pub fn new(url: &str) -> Self {
        unsafe {
            // Create NSURL from the video URL string.
            let nsurl = nsurl_from_str(url);

            // Create AVPlayerItem with the asset.
            let asset: ObjcId = msg_send![class!(AVURLAsset), assetWithURL: nsurl];
            let player_item: ObjcId = msg_send![class!(AVPlayerItem), playerItemWithAsset: asset];

            // Create AVPlayer with the AVPlayerItem.
            let player: ObjcId = msg_send![class!(AVPlayer), playerWithPlayerItem: player_item];

            Self {
                player,
                player_item,
                video_output: nil,
            }
        }
    }

    pub fn setup_video_output(&mut self) {
        unsafe {
            // Define the pixel buffer attributes.
            let pixel_buffer_attributes = ns_dict! {
                kCVPixelBufferPixelFormatTypeKey => kCVPixelFormatType_32BGRA as u64,
                kCVPixelBufferMetalCompatibilityKey => YES,
            };

            // Create the AVPlayerItemVideoOutput with the attributes.
            let video_output: ObjcId = msg_send![
                class!(AVPlayerItemVideoOutput),
                alloc
            ];
            let video_output: ObjcId = msg_send![
                video_output,
                initWithPixelBufferAttributes: pixel_buffer_attributes
            ];

            // Add the video output to the player item.
            let _: () = msg_send![self.player_item, addOutput: video_output];

            self.video_output = video_output;
        }
    }
}