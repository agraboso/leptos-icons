#[cfg(feature = "HiLgOutlineShoppingBag")]
use leptos::*;
#[cfg(feature = "HiLgOutlineShoppingBag")]
///This icon requires the feature `HiLgOutlineShoppingBag` to be enabled.
#[component]
pub fn ShoppingBag(
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
        "M15.75 10.5V6C15.75 3.92893 14.0711 2.25 12 2.25C9.92893 2.25 8.25 3.92893 8.25 6V10.5M19.606 8.50723L20.8692 20.5072C20.9391 21.1715 20.4183 21.75 19.7504 21.75H4.24963C3.58172 21.75 3.06089 21.1715 3.13081 20.5072L4.39397 8.50723C4.45424 7.93466 4.93706 7.5 5.51279 7.5H18.4872C19.0629 7.5 19.5458 7.93466 19.606 8.50723ZM8.625 10.5C8.625 10.7071 8.4571 10.875 8.25 10.875C8.04289 10.875 7.875 10.7071 7.875 10.5C7.875 10.2929 8.04289 10.125 8.25 10.125C8.4571 10.125 8.625 10.2929 8.625 10.5ZM16.125 10.5C16.125 10.7071 15.9571 10.875 15.75 10.875C15.5429 10.875 15.375 10.7071 15.375 10.5C15.375 10.2929 15.5429 10.125 15.75 10.125C15.9571 10.125 16.125 10.2929 16.125 10.5Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
