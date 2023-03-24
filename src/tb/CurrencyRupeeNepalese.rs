#[cfg(feature = "TbCurrencyRupeeNepalese")]
use leptos::*;
#[cfg(feature = "TbCurrencyRupeeNepalese")]
///This icon requires the feature `TbCurrencyRupeeNepalese` to be enabled.
#[component]
pub fn CurrencyRupeeNepalese(
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
        "icon icon-tabler icon-tabler-currency-rupee-nepalese" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 5h-11h3a4 4 0 1 1 0 8h-3l6 6" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M21 17l-4.586 -4.414a2 2 0 0 0 -2.828 2.828l.707 .707" /> < title > { title
        } < / title > < / svg >
    }
}
