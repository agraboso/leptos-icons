#[cfg(feature = "BiRegularAtom")]
use leptos::*;
#[cfg(feature = "BiRegularAtom")]
///This icon requires the feature `BiRegularAtom` to be enabled.
#[component]
pub fn Atom(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.102 20.898c.698.699 1.696 1.068 2.887 1.068 1.742 0 3.855-.778 6.012-2.127 2.156 1.35 4.27 2.127 6.012 2.127 1.19 0 2.188-.369 2.887-1.068 1.269-1.269 1.411-3.413.401-6.039-.358-.932-.854-1.895-1.457-2.859a16.792 16.792 0 0 0 1.457-2.859c1.01-2.626.867-4.771-.401-6.039-.698-.699-1.696-1.068-2.887-1.068-1.742 0-3.855.778-6.012 2.127-2.156-1.35-4.27-2.127-6.012-2.127-1.19 0-2.188.369-2.887 1.068C1.833 4.371 1.69 6.515 2.7 9.141c.359.932.854 1.895 1.457 2.859A16.792 16.792 0 0 0 2.7 14.859c-1.01 2.626-.867 4.77.402 6.039zm16.331-5.321c.689 1.79.708 3.251.052 3.907-.32.32-.815.482-1.473.482-1.167 0-2.646-.503-4.208-1.38a26.611 26.611 0 0 0 4.783-4.784c.336.601.623 1.196.846 1.775zM12 17.417a23.568 23.568 0 0 1-2.934-2.483A23.998 23.998 0 0 1 6.566 12 23.74 23.74 0 0 1 12 6.583a23.568 23.568 0 0 1 2.934 2.483 23.998 23.998 0 0 1 2.5 2.934A23.74 23.74 0 0 1 12 17.417zm6.012-13.383c.657 0 1.152.162 1.473.482.656.656.638 2.117-.052 3.907-.223.579-.51 1.174-.846 1.775a26.448 26.448 0 0 0-4.783-4.784c1.562-.876 3.041-1.38 4.208-1.38zM4.567 8.423c-.689-1.79-.708-3.251-.052-3.907.32-.32.815-.482 1.473-.482 1.167 0 2.646.503 4.208 1.38a26.448 26.448 0 0 0-4.783 4.784 13.934 13.934 0 0 1-.846-1.775zm0 7.154c.223-.579.51-1.174.846-1.775a26.448 26.448 0 0 0 4.783 4.784c-1.563.877-3.041 1.38-4.208 1.38-.657 0-1.152-.162-1.473-.482-.656-.656-.637-2.117.052-3.907z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "2.574"
        /> < title > { title } < / title > < / svg >
    }
}
