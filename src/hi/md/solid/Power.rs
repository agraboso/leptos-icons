#[cfg(feature = "HiMdSolidPower")]
use leptos::*;
#[cfg(feature = "HiMdSolidPower")]
///This icon requires the feature `HiMdSolidPower` to be enabled.
#[component]
pub fn Power(
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
        "M10 2C10.4142 2 10.75 2.33579 10.75 2.75V10.25C10.75 10.6642 10.4142 11 10 11C9.58579 11 9.25 10.6642 9.25 10.25V2.75C9.25 2.33579 9.58579 2 10 2ZM5.40381 4.34315C5.6967 4.63604 5.6967 5.11091 5.40381 5.40381C2.8654 7.94221 2.8654 12.0578 5.40381 14.5962C7.94221 17.1346 12.0578 17.1346 14.5962 14.5962C17.1346 12.0578 17.1346 7.94221 14.5962 5.40381C14.3033 5.11091 14.3033 4.63604 14.5962 4.34315C14.8891 4.05025 15.364 4.05025 15.6569 4.34315C18.781 7.46734 18.781 12.5327 15.6569 15.6569C12.5327 18.781 7.46734 18.781 4.34315 15.6569C1.21895 12.5327 1.21895 7.46734 4.34315 4.34315C4.63604 4.05025 5.11091 4.05025 5.40381 4.34315Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
