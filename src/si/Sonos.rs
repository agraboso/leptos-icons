#[cfg(feature = "SiSonos")]
use leptos::*;
#[cfg(feature = "SiSonos")]
///This icon requires the feature `SiSonos` to be enabled.
#[component]
pub fn Sonos(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12.988 12.36l-2.813-2.634v4.429h.837V11.7l2.813 2.633V9.905h-.837zM6.464 9.665A2.3 2.3 0 0 0 4.13 12c0 1.257 1.077 2.334 2.334 2.334A2.3 2.3 0 0 0 8.798 12a2.3 2.3 0 0 0-2.334-2.334m0 3.83A1.482 1.482 0 0 1 4.968 12c0-.838.658-1.496 1.496-1.496S7.96 11.162 7.96 12s-.658 1.496-1.496 1.496M2.694 12c-.24-.18-.54-.3-.958-.419-.838-.24-.838-.479-.838-.598 0-.24.299-.48.718-.48.36 0 .658.18.778.24l.06.06.658-.479-.06-.06s-.538-.598-1.436-.598c-.419 0-.838.12-1.137.359-.3.24-.479.598-.479.958s.18.718.479.957c.24.18.538.3.957.42.838.239.838.478.838.598 0 .239-.299.478-.718.478-.359 0-.658-.18-.778-.239l-.06-.06-.658.479.06.06s.538.598 1.436.598c.42 0 .838-.12 1.137-.359.3-.24.48-.598.48-.957 0-.36-.18-.659-.48-.958m14.843-2.334A2.3 2.3 0 0 0 15.202 12a2.337 2.337 0 0 0 2.334 2.334A2.3 2.3 0 0 0 19.87 12a2.337 2.337 0 0 0-2.334-2.334m0 3.83A1.482 1.482 0 0 1 16.04 12c0-.838.658-1.496 1.496-1.496s1.496.658 1.496 1.496-.718 1.496-1.496 1.496m3.77-1.556c.24.18.54.3.958.42.838.239.838.478.838.598 0 .239-.299.478-.718.478-.36 0-.658-.18-.778-.239h-.06l-.658.479.06.06s.538.598 1.436.598c.419 0 .838-.12 1.137-.359s.479-.598.479-.958-.18-.718-.479-.957c-.24-.18-.538-.3-.957-.42-.838-.239-.838-.478-.838-.598 0-.239.299-.478.718-.478.359 0 .658.18.778.239l.06.06.658-.479-.06-.06s-.538-.598-1.436-.598c-.42 0-.838.12-1.137.359-.3.24-.48.598-.48.957-.059.36.12.659.48.898"
        /> < title > { title } < / title > < / svg >
    }
}
