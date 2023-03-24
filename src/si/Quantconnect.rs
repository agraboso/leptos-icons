#[cfg(feature = "SiQuantconnect")]
use leptos::*;
#[cfg(feature = "SiQuantconnect")]
///This icon requires the feature `SiQuantconnect` to be enabled.
#[component]
pub fn Quantconnect(
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
        "M24 12c0 6.6274-5.3726 12-12 12v-2.8065c5.0774 0 9.1935-4.116 9.1935-9.1934 0-5.0775-4.1161-9.1935-9.1935-9.1935S2.8065 6.9226 2.8065 12H0C0 5.3726 5.3726 0 12 0s12 5.3726 12 12zm-3.5446 0c0-4.6698-3.7856-8.4554-8.4554-8.4554v2.7973c3.1249 0 5.6581 2.5332 5.6581 5.6581 0 3.1249-2.5332 5.6581-5.6581 5.6581-3.1249 0-5.6581-2.5333-5.6581-5.6581H3.5446c0 4.6698 3.7856 8.4554 8.4554 8.4554S20.4554 16.6698 20.4554 12zM12 9.8819V7.0487c-2.7436 0-4.9677 2.2167-4.9677 4.9513 0 2.7345 2.2241 4.9513 4.9677 4.9513S16.9677 14.7345 16.9677 12h-2.8425c0 1.1698-.9515 2.1182-2.1252 2.1182S9.8748 13.1698 9.8748 12 10.8263 9.8819 12 9.8819z"
        /> < title > { title } < / title > < / svg >
    }
}
