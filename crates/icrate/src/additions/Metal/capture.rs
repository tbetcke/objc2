#![allow(unused_imports)]
use crate::common::*;
use crate::Metal::*;

// SAFETY: The documentation for captureObject specifies that the object
// may be one of these three:
// <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237248-captureobject?language=objc>
impl MTLCaptureDescriptor {
    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLDevice")]
    pub fn set_capture_device(&self, device: &ProtocolObject<dyn MTLDevice>) {
        let device: *const _ = device;
        let device: *const AnyObject = device.cast();
        unsafe { self.setCaptureObject(Some(&*device)) }
    }

    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLCaptureScope")]
    pub fn set_capture_scope(&self, scope: &ProtocolObject<dyn MTLCaptureScope>) {
        let scope: *const _ = scope;
        let scope: *const AnyObject = scope.cast();
        unsafe { self.setCaptureObject(Some(&*scope)) }
    }

    #[doc(alias = "setCaptureObject")]
    #[cfg(feature = "Metal_MTLCommandQueue")]
    pub fn set_capture_command_queue(&self, command_queue: &ProtocolObject<dyn MTLCommandQueue>) {
        let command_queue: *const _ = command_queue;
        let command_queue: *const AnyObject = command_queue.cast();
        unsafe { self.setCaptureObject(Some(&*command_queue)) }
    }
}
