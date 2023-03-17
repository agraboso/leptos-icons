#[cfg(feature = "RiDeviceFillBluetooth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillBluetooth")]
/// *This icon requires the feature* `RiDeviceFillBluetooth` *to be enabled*.
#[component]
pub fn Bluetooth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14.341 12.03l4.343 4.343-5.656 5.656h-2v-6.686l-4.364 4.364-1.415-1.414 5.779-5.778v-.97L5.249 5.765l1.415-1.414 4.364 4.364V2.029h2l5.656 5.657-4.343 4.343zm-1.313 1.514v5.657l2.828-2.828-2.828-2.829zm0-3.03l2.828-2.828-2.828-2.828v5.657z" /></g></svg>
   }
}