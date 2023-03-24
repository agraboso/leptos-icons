#[cfg(feature = "HiMdSolidMusicalNote")]
use leptos::*;
#[cfg(feature = "HiMdSolidMusicalNote")]
///This icon requires the feature `HiMdSolidMusicalNote` to be enabled.
#[component]
pub fn MusicalNote(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M17.721 1.59894C17.8974 1.74132 18 1.95589 18 2.18262V13.4723C18 14.5317 17.261 15.4476 16.2255 15.6715L14.1848 16.1127C12.804 16.4113 11.5 15.3591 11.5 13.9464C11.5 12.9032 12.2275 12.0012 13.247 11.7803L15.9088 11.2035C16.2538 11.1287 16.5 10.8235 16.5 10.4705V6.11212L8.5 7.84185V15.5263C8.5 16.5857 7.76097 17.5016 6.7255 17.7255L4.68397 18.1669C3.30357 18.4654 2 17.4136 2 16.0013C2 14.9574 2.72854 14.0552 3.74894 13.8353L6.408 13.2622C6.7534 13.1878 7 12.8824 7 12.5291V4.23668C7 3.88354 7.24634 3.57825 7.5915 3.50362L17.0915 1.44956C17.3131 1.40165 17.5445 1.45656 17.721 1.59894Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
