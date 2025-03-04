use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::MXMetricManager;

#[cfg(feature = "Foundation_NSString")]
pub type MXLaunchTaskID = NSString;

// pub type os_log_t = ProtocolObject<dyn OS_os_log>;

// extern_protocol!(
//     pub unsafe trait OS_os_log: NSObjectProtocol {}
//     unsafe impl ProtocolType for dyn OS_os_log {}
// );

extern_methods!(
    unsafe impl MXMetricManager {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(extendLaunchMeasurementForTaskID:error:_)]
        pub unsafe fn extendLaunchMeasurementForTaskID_error(
            &self,
            task_id: &MXLaunchTaskID,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(finishExtendedLaunchMeasurementForTaskID:error:_)]
        pub unsafe fn finishExtendedLaunchMeasurementForTaskID_error(
            &self,
            task_id: &MXLaunchTaskID,
        ) -> Result<(), Id<NSError>>;

        // #[cfg(feature = "Foundation_NSString")]
        // #[method(makeLogHandleWithCategory:)]
        // pub unsafe fn makeLogHandleWithCategory(category: &NSString) -> Id<os_log_t>;
    }
);
