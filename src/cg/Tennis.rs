#[cfg(feature = "CgTennis")]
use leptos::*;
#[cfg(feature = "CgTennis")]
///This icon requires the feature `CgTennis` to be enabled.
#[component]
pub fn Tennis(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M19.0711 19.0711C22.9763 15.1658 22.9763 8.83418 19.0711 4.92893C15.1658 1.02369 8.83418 1.02369 4.92893 4.92893C1.02369 8.83418 1.02369 15.1658 4.92893 19.0711C8.83418 22.9763 15.1658 22.9763 19.0711 19.0711ZM19.9426 11.0407C19.7364 9.32374 18.9745 7.6608 17.6569 6.34315C16.3343 5.0206 14.6639 4.25792 12.9402 4.0551L12.93 4.11139C12.1086 8.58308 8.58304 12.1086 4.11136 12.93L4.05511 12.9402C4.25793 14.6639 5.02061 16.3343 6.34315 17.6569C7.66079 18.9745 9.32373 19.7364 11.0407 19.9426C11.0449 19.9191 11.0491 19.8956 11.0534 19.8721C11.8748 15.4004 15.4003 11.8748 19.872 11.0534C19.8955 11.0491 19.9191 11.0449 19.9426 11.0407ZM19.872 13.4292V13.0951C16.5073 13.8586 13.8586 16.5073 13.095 19.8721H13.429C14.9777 19.5924 16.4598 18.8539 17.6569 17.6569C18.8539 16.4598 19.5923 14.9778 19.872 13.4292ZM4.11136 10.8884L4.11136 10.666C4.37766 9.08301 5.12159 7.5647 6.34315 6.34315C7.56465 5.12165 9.08288 4.37773 10.6658 4.11139L10.8883 4.11139C10.1248 7.47615 7.47612 10.1249 4.11136 10.8884Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
