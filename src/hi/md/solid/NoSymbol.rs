#[cfg(feature = "HiMdSolidNoSymbol")]
use leptos::*;
#[cfg(feature = "HiMdSolidNoSymbol")]
///This icon requires the feature `HiMdSolidNoSymbol` to be enabled.
#[component]
pub fn NoSymbol(
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
        "M5.96461 4.90395L15.096 14.0354C15.9755 12.9265 16.5 11.525 16.5 10C16.5 6.41015 13.5899 3.5 10 3.5C8.475 3.5 7.07353 4.02446 5.96461 4.90395ZM14.0354 15.096L4.90395 5.96461C4.02446 7.07353 3.5 8.475 3.5 10C3.5 13.5899 6.41015 16.5 10 16.5C11.525 16.5 12.9265 15.9755 14.0354 15.096ZM4.34315 4.34315C5.79004 2.89625 7.79107 2 10 2C14.4183 2 18 5.58172 18 10C18 12.2089 17.1037 14.21 15.6569 15.6569C14.21 17.1037 12.2089 18 10 18C5.58172 18 2 14.4183 2 10C2 7.79107 2.89625 5.79004 4.34315 4.34315Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
