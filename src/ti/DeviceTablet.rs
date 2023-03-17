#[cfg(feature = "TiDeviceTablet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiDeviceTablet")]
/// *This icon requires the feature* `TiDeviceTablet` *to be enabled*.
#[component]
pub fn DeviceTablet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M17 4h-9c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h3.5c0 .553.448 1 1 1s1-.447 1-1h3.5c.55 0 1-.45 1-1v-12c0-.55-.45-1-1-1zm0 13h-9v-12h9v12zM18 1h-11c-1.654 0-3 1.346-3 3v15c0 1.654 1.346 3 3 3h11c1.654 0 3-1.346 3-3v-15c0-1.654-1.346-3-3-3zm1 18c0 .551-.449 1-1 1h-11c-.551 0-1-.449-1-1v-15c0-.551.449-1 1-1h11c.551 0 1 .449 1 1v15z" /></g></svg>
   }
}