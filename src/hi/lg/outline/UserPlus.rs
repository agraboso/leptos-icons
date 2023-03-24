#[cfg(feature = "HiLgOutlineUserPlus")]
use leptos::*;
#[cfg(feature = "HiLgOutlineUserPlus")]
///This icon requires the feature `HiLgOutlineUserPlus` to be enabled.
#[component]
pub fn UserPlus(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 7.5V10.5M19 10.5V13.5M19 10.5H22M19 10.5H16M13.75 6.375C13.75 8.23896 12.239 9.75 10.375 9.75C8.51104 9.75 7 8.23896 7 6.375C7 4.51104 8.51104 3 10.375 3C12.239 3 13.75 4.51104 13.75 6.375ZM4.00092 19.2343C4.00031 19.198 4 19.1615 4 19.125C4 15.6042 6.85418 12.75 10.375 12.75C13.8958 12.75 16.75 15.6042 16.75 19.125V19.1276C16.75 19.1632 16.7497 19.1988 16.7491 19.2343C14.8874 20.3552 12.7065 21 10.375 21C8.04353 21 5.86264 20.3552 4.00092 19.2343Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
