#[cfg(feature = "TbCurrencyKroneDanish")]
use leptos::*;
#[cfg(feature = "TbCurrencyKroneDanish")]
///This icon requires the feature `TbCurrencyKroneDanish` to be enabled.
#[component]
pub fn CurrencyKroneDanish(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-currency-krone-danish" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 6v12" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 12c3.5 0 6 -3 6 -6" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 12c3.5 0 6 3 6 6" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 10v8" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 10a4 4 0 0 0 -4 4" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 18.01v-.01" /> < title > { title } < / title > < / svg >
    }
}
