use unity_native_plugin_sys::*;
use unity_native_plugin::interface::UnityInterface;
use winapi::shared::{dxgi, dxgiformat, dxgitype, minwindef, winerror};
use winapi::um::{objbase, combaseapi, d3d11, d3dcommon, winnt};
use winit::window::Window;
use raw_window_handle::HasRawWindowHandle;
use wio::com::ComPtr;

struct UnityGraphicsD3D11 {
    device: ComPtr<d3d11::ID3D11Device>,
    interfaces: IUnityGraphicsD3D11,
}

impl UnityGraphicsD3D11 {
    pub fn new(device: ComPtr<d3d11::ID3D11Device>) -> UnityGraphicsD3D11 {
        UnityGraphicsD3D11 {
            device: device,
            interfaces: IUnityGraphicsD3D11 {
                GetDevice: Some(get_device),
                TextureFromRenderBuffer: Some(texture_from_render_buffer),
                TextureFromNativeTexture: Some(texture_from_native_texture),
                RTVFromRenderBuffer: Some(rtv_from_render_buffer),
                SRVFromNativeTexture: Some(srv_from_native_texture),
            },
        }
    }

    pub fn device(&self) -> *mut d3d11::ID3D11Device { self.device.as_raw() }
}

impl crate::interface::UnityInterfaceBase for UnityGraphicsD3D11 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_unity_interface(&self) -> *mut IUnityInterface {
        unsafe { std::mem::transmute::<_, _>(&self.interfaces) }
    }
}

impl crate::interface::UnityInterfaceID for UnityGraphicsD3D11 {
    fn get_interface_guid() -> UnityInterfaceGUID {
        unity_native_plugin::d3d11::UnityGraphicsD3D11::get_interface_guid()
    }
}

extern "system" fn get_device() -> *mut ID3D11Device {
    unsafe { crate::interface::get_unity_interface::<UnityGraphicsD3D11>().device() as _ }
}

extern "system" fn texture_from_render_buffer(buffer: UnityRenderBuffer) -> *mut ID3D11Resource {
    std::ptr::null_mut()
}

extern "system" fn texture_from_native_texture(texture: UnityTextureID) -> *mut ID3D11Resource {
    std::ptr::null_mut()
}

extern "system" fn rtv_from_render_buffer(
    surface: UnityRenderBuffer,
) -> *mut ID3D11RenderTargetView {
    std::ptr::null_mut()
}

extern "system" fn srv_from_native_texture(
    texture: UnityTextureID,
) -> *mut ID3D11ShaderResourceView {
    std::ptr::null_mut()
}

pub fn initialize_interface(device: ComPtr<d3d11::ID3D11Device>) {
    unsafe {
        crate::interface::get_unity_interfaces().register_interface::<UnityGraphicsD3D11>(Some(
            Box::new(UnityGraphicsD3D11::new(device)),
        ));
    }
}

pub struct TesterContext {
    device: ComPtr<d3d11::ID3D11Device>,
    device_context: ComPtr<d3d11::ID3D11DeviceContext>,
    swap_chain: ComPtr<dxgi::IDXGISwapChain>,
}

impl TesterContext {
    fn new(window: &Window) -> Result<Self, winnt::HRESULT> {
        unsafe {
            let size = window.inner_size();
            let desc = dxgi::DXGI_SWAP_CHAIN_DESC {
                BufferDesc: dxgitype::DXGI_MODE_DESC {
                    Width: size.width,
                    Height: size.height,
                    RefreshRate: dxgitype::DXGI_RATIONAL {
                        Numerator: 60,
                        Denominator: 1,
                    },
                    Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                    ScanlineOrdering: dxgitype::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: dxgitype::DXGI_MODE_SCALING_UNSPECIFIED,
                },
                SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                OutputWindow: match window.raw_window_handle() {
                    raw_window_handle::RawWindowHandle::Windows(h) => h.hwnd,
                    _ => std::ptr::null_mut()
                } as _,
                Windowed: minwindef::TRUE,
                SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
                Flags: dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            };

            let mut device: *mut d3d11::ID3D11Device = std::ptr::null_mut();
            let mut swap_chain: *mut dxgi::IDXGISwapChain = std::ptr::null_mut();
            let mut feature: d3d11::D3D11_FEATURE = 0;
            let mut dc: *mut d3d11::ID3D11DeviceContext = std::ptr::null_mut();

            let hr = d3d11::D3D11CreateDeviceAndSwapChain(
                std::ptr::null_mut(),
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                std::ptr::null_mut(),
                d3d11::D3D11_CREATE_DEVICE_DEBUG | d3d11::D3D11_CREATE_DEVICE_SINGLETHREADED,
                std::ptr::null_mut(),
                0,
                d3d11::D3D11_SDK_VERSION,
                &desc,
                &mut swap_chain,
                &mut device,
                &mut feature,
                &mut dc,
            );
            if winerror::SUCCEEDED(hr) {
                Ok(TesterContext {
                    device: ComPtr::from_raw(device),
                    device_context: ComPtr::from_raw(dc),
                    swap_chain: ComPtr::from_raw(swap_chain),
                })
            } else {
                Err(hr)
            }
        }
    }

    pub fn device(&self) -> *mut d3d11::ID3D11Device {
        self.device.as_raw()
    }

    pub fn device_context(&self) -> *mut d3d11::ID3D11DeviceContext {
        self.device_context.as_raw()
    }

    pub fn swap_chain(&self) -> *mut dxgi::IDXGISwapChain {
        self.swap_chain.as_raw()
    }
}

pub fn test_plugin_d3d11<
    FnMain: FnMut(&Window, &mut TesterContext) -> crate::window::LoopResult,
>(
    mut fn_main: FnMain,
    fn_unity_plugin_load: fn(interfaces: &unity_native_plugin::interface::UnityInterfaces),
    fn_unity_plugin_unload: fn()
) -> Result<(), winnt::HRESULT> {
    unsafe { objbase::CoInitialize(std::ptr::null_mut()); }

    crate::interface::initialize_unity_interfaces();
    crate::graphics::initialize_interface(unity_native_plugin::graphics::GfxRenderer::D3D11);

    fn_unity_plugin_load(unity_native_plugin::interface::UnityInterfaces::get());
    crate::window::run_window_app(
        |window| TesterContext::new(window).unwrap(),
        fn_main,
        |window, context| {},
    );
    fn_unity_plugin_unload();

    crate::interface::finalize_unity_interfaces();

    unsafe { combaseapi::CoUninitialize(); }

    Ok(())
}
