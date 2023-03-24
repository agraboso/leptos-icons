#[cfg(feature = "HiMdSolidAtSymbol")]
use leptos::*;
#[cfg(feature = "HiMdSolidAtSymbol")]
///This icon requires the feature `HiMdSolidAtSymbol` to be enabled.
#[component]
pub fn AtSymbol(
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
        "M5.40403 14.5962C2.86563 12.0578 2.86563 7.94221 5.40403 5.40381C7.94244 2.8654 12.058 2.8654 14.5964 5.40381C15.8658 6.67316 16.5002 8.33534 16.5002 10C16.5002 10.6904 15.9404 11.25 15.25 11.25C14.5596 11.25 14 10.6904 14 10C14 7.79086 12.2091 6 10 6C7.79086 6 6 7.79086 6 10C6 12.2091 7.79086 14 10 14C11.4553 14 12.7292 13.2228 13.429 12.0607C13.914 12.4897 14.5516 12.75 15.25 12.75C16.7614 12.75 17.9881 11.5307 17.9999 10.022C18.0001 10.0147 18.0002 10.0074 18.0002 10C18.0002 7.95378 17.219 5.9051 15.6571 4.34315C12.5329 1.21895 7.46757 1.21895 4.34337 4.34315C1.21918 7.46734 1.21918 12.5327 4.34337 15.6569C7.46757 18.781 12.5329 18.781 15.6571 15.6569C15.95 15.364 15.95 14.8891 15.6571 14.5962C15.3642 14.3033 14.8893 14.3033 14.5964 14.5962C12.058 17.1346 7.94244 17.1346 5.40403 14.5962ZM10 7.5C8.61929 7.5 7.5 8.61929 7.5 10C7.5 11.3807 8.61929 12.5 10 12.5C11.3807 12.5 12.5 11.3807 12.5 10C12.5 8.61929 11.3807 7.5 10 7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
