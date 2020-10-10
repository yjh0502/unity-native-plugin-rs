use crate::define_unity_interface;
use crate::interface::UnityInterface;
use unity_native_plugin_sys::*;

define_unity_interface!(
    UnityProfiler,
    unity_native_plugin_sys::IUnityProfiler,
    0x2CE79ED8316A4833_u64,
    0x87076B2013E1571F_u64
);

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum BuiltinProfilerCategory {
    Render = UnityBuiltinProfilerCategory__kUnityProfilerCategoryRender as u16,
    Scripts = UnityBuiltinProfilerCategory__kUnityProfilerCategoryScripts as u16,
    ManagedJobs = UnityBuiltinProfilerCategory__kUnityProfilerCategoryManagedJobs as u16,
    BurstJobs = UnityBuiltinProfilerCategory__kUnityProfilerCategoryBurstJobs as u16,
    GUI = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGUI as u16,
    Physics = UnityBuiltinProfilerCategory__kUnityProfilerCategoryPhysics as u16,
    Animation = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAnimation as u16,
    AI = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAI as u16,
    Audio = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudio as u16,
    AudioJob = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudioJob as u16,
    AudioUpdateJob = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAudioUpdateJob as u16,
    Video = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVideo as u16,
    Particles = UnityBuiltinProfilerCategory__kUnityProfilerCategoryParticles as u16,
    Gi = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGi as u16,
    Network = UnityBuiltinProfilerCategory__kUnityProfilerCategoryNetwork as u16,
    Loading = UnityBuiltinProfilerCategory__kUnityProfilerCategoryLoading as u16,
    Other = UnityBuiltinProfilerCategory__kUnityProfilerCategoryOther as u16,
    GC = UnityBuiltinProfilerCategory__kUnityProfilerCategoryGC as u16,
    VSync = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVSync as u16,
    Overhead = UnityBuiltinProfilerCategory__kUnityProfilerCategoryOverhead as u16,
    PlayerLoop = UnityBuiltinProfilerCategory__kUnityProfilerCategoryPlayerLoop as u16,
    Director = UnityBuiltinProfilerCategory__kUnityProfilerCategoryDirector as u16,
    VR = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVR as u16,
    Allocation = UnityBuiltinProfilerCategory__kUnityProfilerCategoryAllocation as u16,
    Internal = UnityBuiltinProfilerCategory__kUnityProfilerCategoryInternal as u16,
    FileIO = UnityBuiltinProfilerCategory__kUnityProfilerCategoryFileIO as u16,
    UISystemLayout = UnityBuiltinProfilerCategory__kUnityProfilerCategoryUISystemLayout as u16,
    UISystemRender = UnityBuiltinProfilerCategory__kUnityProfilerCategoryUISystemRender as u16,
    VFX = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVFX as u16,
    BuildInterface = UnityBuiltinProfilerCategory__kUnityProfilerCategoryBuildInterface as u16,
    Input = UnityBuiltinProfilerCategory__kUnityProfilerCategoryInput as u16,
    VirtualTexturing = UnityBuiltinProfilerCategory__kUnityProfilerCategoryVirtualTexturing as u16,
}

pub type ProfilerCategoryId = UnityProfilerCategoryId;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerFlag {
    Default = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagDefault as u16,
    ScriptUser = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptUser as u16,
    ScriptInvoke = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptInvoke as u16,
    ScriptEnterLeave = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptEnterLeave as u16,
    AvailabilityEditor = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityEditor as u16,
    AvailabilityNonDev = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityNonDev as u16,
    Warning = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagWarning as u16,
    Counter = UnityProfilerMarkerFlag__kCounter as u16,
    VerbosityDebug = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityDebug as u16,
    VerbosityInternal = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityInternal as u16,
    VerbosityAdvanced = UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityAdvanced as u16,
}

#[derive(Copy, Clone)]
pub struct ProfilerMarkerFlags {
    pub flag: u16,
}

impl From<UnityProfilerMarkerFlags> for ProfilerMarkerFlags {
    fn from(value: u16) -> Self {
        ProfilerMarkerFlags { flag: value }
    }
}

impl Into<UnityProfilerMarkerFlags> for ProfilerMarkerFlags {
    fn into(self) -> UnityProfilerMarkerFlags {
        self.flag as UnityProfilerMarkerFlags
    }
}

impl ProfilerMarkerFlags {
    pub fn new(flag: ProfilerMarkerFlag) -> ProfilerMarkerFlags {
        ProfilerMarkerFlags { flag: flag as u16 }
    }

    pub const fn is_default(&self) -> bool {
        self.flag == UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagDefault as u16;
    }

    pub const fn has_flag(&self, flag: ProfilerMarkerFlag) -> bool {
        (self.flag & flag) != 0
    }

    pub const fn set_flag(&self, flag: ProfilerMarkerFlag) -> ProfilerMarkerFlags {
        ProfilerMarkerFlags {
            flag: self.flag | flag,
        }
    }

    pub const fn unset_flag(&self, flag: ProfilerMarkerFlag) -> ProfilerMarkerFlags {
        ProfilerMarkerFlags {
            flag: self.flag & !flag,
        }
    }
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerEventType {
    Begin = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeBegin as u16,
    End = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeEnd as u16,
    Single = UnityProfilerMarkerEventType__kUnityProfilerMarkerEventTypeSingle as u16,
}

pub type ProfilerMarkerId = UnityProfilerMarkerId;

pub struct ProfilerMarkerDesc<'a> {
    pub id: ProfilerMarkerId,
    pub flags: ProfilerMarkerFlags,
    pub category_id: ProfilerCategoryId,
    pub name: &'a std::ffi::CStr,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerDataType {
    None = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeNone as u8,
    InstanceId = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInstanceId as u8,
    Int32 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInt32 as u8,
    UInt32 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeUInt32 as u8,
    Int64 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeInt64 as u8,
    UInt64 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeUInt64 as u8,
    Float = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeFloat as u8,
    Double = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeDouble as u8,
    String = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeString as u8,
    String16 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeString16 as u8,
    Blob8 = UnityProfilerMarkerDataType__kUnityProfilerMarkerDataTypeBlob8 as u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerMarkerDataUnit {
    Undefined = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitUndefined as u8,
    TimeNanoseconds =
        UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitTimeNanoseconds as u8,
    Bytes = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitBytes as u8,
    Count = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitCount as u8,
    Percent = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitPercent as u8,
    FrequencyHz = UnityProfilerMarkerDataUnit__kUnityProfilerMarkerDataUnitFrequencyHz as u8,
}

#[repr(C)]
pub struct ProfilerMarkerData<'a> {
    pub data_type: ProfilerMarkerDataType,
    reserved0: u8,
    reserved1: u16,
    size: u32,
    ptr: &'a ::std::os::raw::c_void,
}

impl ProfilerMarkerData<'_> {
    pub fn new<'a>(data_type: ProfilerMarkerDataType, data: &'a [u8]) -> ProfilerMarkerData<'a> {
        unsafe {
            ProfilerMarkerData {
                data_type: data_type,
                reserved0: 0,
                reserved1: 0,
                size: data.len() as u32,
                ptr: &*(data.as_ptr() as *const ::std::os::raw::c_void),
            }
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProfilerFlowEventType {
    Begin = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeBegin as u8,
    Next = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeNext as u8,
    End = UnityProfilerFlowEventType__kUnityProfilerFlowEventTypeEnd as u8,
}

type ProfilerThreadId = UnityProfilerThreadId;

impl UnityProfiler {
    pub fn emit_event(
        &self,
        marker_desc: &ProfilerMarkerDesc,
        event_type: ProfilerMarkerEventType,
        event_data: &[ProfilerMarkerData],
    ) {
        unsafe {
            self.interface().EmitEvent.expect("EmitEvent")(
                &UnityProfilerMarkerDesc {
                    callback: std::ptr::null(),
                    id: marker_desc.id,
                    flags: marker_desc.flags.into(),
                    categoryId: marker_desc.category_id,
                    name: marker_desc.name.as_ptr(),
                    metaDataDesc: std::ptr::null(),
                },
                event_type as UnityProfilerMarkerEventType,
                event_data.len() as u16,
                event_data.as_ptr() as *const _,
            );
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { self.interface().IsEnabled.expect("IsEnabled")() != 0 }
    }

    pub fn is_available(&self) -> bool {
        unsafe { self.interface().IsAvailable.expect("IsAvailable")() != 0 }
    }

    pub fn create_marker<'a>(
        &'a self,
        name: &std::ffi::CStr,
        category: ProfilerCategoryId,
        flags: ProfilerMarkerFlags,
        event_data_count: ::std::os::raw::c_int,
    ) -> Result<ProfilerMarkerDesc, ::std::os::raw::c_int> {
        unsafe {
            let mut ret = std::ptr::null::<UnityProfilerMarkerDesc>();
            let result = self.interface().CreateMarker.expect("CreateMarker")(
                &mut ret,
                name.as_ptr(),
                category as _,
                flags.flag as _,
                event_data_count,
            );
            if result > 0 {
                Err(result)
            } else {
                Ok(ProfilerMarkerDesc {
                    id: (*ret).id,
                    flags: (*ret).flags.into(),
                    category_id: (*ret).categoryId,
                    name: std::ffi::CStr::from_ptr::<'a>((*ret).name),
                })
            }
        }
    }

    pub fn set_marker_metadata_name(
        &self,
        desc: &ProfilerMarkerDesc,
        index: ::std::os::raw::c_int,
        metadata_name: &std::ffi::CStr,
        metadata_type: ProfilerMarkerDataType,
        metadata_unit: ProfilerMarkerDataUnit,
    ) -> Result<(), ::std::os::raw::c_int> {
        unsafe {
            let result = self
                .interface()
                .SetMarkerMetadataName
                .expect("SetMarkerMetadataName")(
                &UnityProfilerMarkerDesc {
                    callback: std::ptr::null(),
                    id: desc.id,
                    flags: desc.flags.into(),
                    categoryId: desc.category_id,
                    name: desc.name.as_ptr(),
                    metaDataDesc: std::ptr::null(),
                },
                index,
                metadata_name.as_ptr(),
                metadata_type as _,
                metadata_unit as _,
            );
            if result > 0 {
                Err(result)
            } else {
                Ok(())
            }
        }
    }

    pub fn register_thread(
        &self,
        group_name: &std::ffi::CStr,
        name: &std::ffi::CStr,
    ) -> Result<ProfilerThreadId, ::std::os::raw::c_int> {
        unsafe {
            let mut thread_id = std::mem::zeroed::<UnityProfilerThreadId>();

            let result = self.interface().RegisterThread.expect("RegisterThread")(
                &mut thread_id,
                group_name.as_ptr(),
                name.as_ptr(),
            );
            if result > 0 {
                Err(result)
            } else {
                Ok(thread_id)
            }
        }
    }

    pub fn unregister_thread(
        &self,
        thread_id: ProfilerThreadId,
    ) -> Result<(), ::std::os::raw::c_int> {
        unsafe {
            let result = self.interface().UnregisterThread.expect("UnregisterThread")(thread_id);
            if result > 0 {
                Err(result)
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_test() {
        assert_eq!(
            ::std::mem::size_of::<ProfilerMarkerFlags>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityProfilerMarkerFlags>()
        );
        assert_eq!(
            ::std::mem::size_of::<ProfilerMarkerData>(),
            ::std::mem::size_of::<unity_native_plugin_sys::UnityProfilerMarkerData>()
        );
    }

    #[test]
    fn flags_test() {
        let f = [
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptUser,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptInvoke,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagScriptEnterLeave,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityEditor,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagAvailabilityNonDev,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagWarning,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityDebug,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityInternal,
            unity_native_plugin_sys::UnityProfilerMarkerFlag__kUnityProfilerMarkerFlagVerbosityAdvanced,
        ];

        let f2 = [
            ProfilerMarkerFlag::ScriptUser,
            ProfilerMarkerFlag::ScriptInvoke,
            ProfilerMarkerFlag::ScriptEnterLeave,
            ProfilerMarkerFlag::AvailabilityEditor,
            ProfilerMarkerFlag::AvailabilityNonDev,
            ProfilerMarkerFlag::Warning,
            ProfilerMarkerFlag::VerbosityDebug,
            ProfilerMarkerFlag::VerbosityInternal,
            ProfilerMarkerFlag::VerbosityAdvanced,
        ];

        for i in 0..f.len() {
            assert!(ProfilerMarkerFlags::from(f[i] as u16).has_flag(f2[i]));
            assert_eq!(ProfilerMarkerFlags::new(f2[i]).unset_flag(f2[i]), 0);
        }
    }
}
