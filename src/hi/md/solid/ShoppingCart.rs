#[cfg(feature = "HiMdSolidShoppingCart")]
use leptos::*;
#[cfg(feature = "HiMdSolidShoppingCart")]
///This icon requires the feature `HiMdSolidShoppingCart` to be enabled.
#[component]
pub fn ShoppingCart(
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
        "http://www.w3.org/2000/svg" d =
        "M1 1.75C1 1.33579 1.33579 1 1.75 1H3.37846C4.25256 1 4.99261 1.64498 5.11205 2.51088L5.17955 3.00024C9.75911 3.01263 14.2281 3.49872 18.5398 4.41236C18.9395 4.49706 19.1979 4.88613 19.1209 5.2874C18.7145 7.40548 18.1717 9.47515 17.502 11.4869C17.4 11.7933 17.1134 12 16.7904 12H6C5.88567 12 5.77351 12.0076 5.66393 12.0223C4.78545 12.14 4.05092 12.7153 3.70796 13.5H17.25C17.6642 13.5 18 13.8358 18 14.25C18 14.6642 17.6642 15 17.25 15H2.75948C2.55068 15 2.35133 14.913 2.2094 14.7598C2.06747 14.6067 1.9958 14.4013 2.01164 14.1931C2.13566 12.5628 3.23526 11.2069 4.72829 10.7066L3.62612 2.71584C3.60906 2.59214 3.50333 2.5 3.37846 2.5H1.75C1.33579 2.5 1 2.16421 1 1.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 17.5C6 18.3284 5.32843 19 4.5 19C3.67157 19 3 18.3284 3 17.5C3 16.6716 3.67157 16 4.5 16C5.32843 16 6 16.6716 6 17.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 19C16.3284 19 17 18.3284 17 17.5C17 16.6716 16.3284 16 15.5 16C14.6716 16 14 16.6716 14 17.5C14 18.3284 14.6716 19 15.5 19Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
