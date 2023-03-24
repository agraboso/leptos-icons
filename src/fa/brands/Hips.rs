#[cfg(feature = "FaBrandsHips")]
use leptos::*;
#[cfg(feature = "FaBrandsHips")]
///This icon requires the feature `FaBrandsHips` to be enabled.
#[component]
pub fn Hips(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M251.6 157.6c0-1.9-.9-2.8-2.8-2.8h-40.9c-1.6 0-2.7 1.4-2.7 2.8v201.8c0 1.4 1.1 2.8 2.7 2.8h40.9c1.9 0 2.8-.9 2.8-2.8zM156.5 168c-16.1-11.8-36.3-17.9-60.3-18-18.1-.1-34.6 3.7-49.8 11.4V80.2c0-1.8-.9-2.7-2.8-2.7H2.7c-1.8 0-2.7.9-2.7 2.7v279.2c0 1.9.9 2.8 2.7 2.8h41c1.9 0 2.8-.9 2.8-2.8V223.3c0-.8-2.8-27 45.8-27 48.5 0 45.8 26.1 45.8 27v122.6c0 9 7.3 16.3 16.4 16.3h27.3c1.8 0 2.7-.9 2.7-2.8V223.3c0-23.4-9.3-41.8-28-55.3zm478.4 110.1c-6.8-15.7-18.4-27-34.9-34.1l-57.6-25.3c-8.6-3.6-9.2-11.2-2.6-16.1 7.4-5.5 44.3-13.9 84 6.8 1.7 1 4-.3 4-2.4v-44.7c0-1.3-.6-2.1-1.9-2.6-17.7-6.6-36.1-9.9-55.1-9.9-26.5 0-45.3 5.8-58.5 15.4-.5.4-28.4 20-22.7 53.7 3.4 19.6 15.8 34.2 37.2 43.6l53.6 23.5c11.6 5.1 15.2 13.3 12.2 21.2-3.7 9.1-13.2 13.6-36.5 13.6-24.3 0-44.7-8.9-58.4-19.1-2.1-1.4-4.4.2-4.4 2.3v34.4c0 10.4 4.9 17.3 14.6 20.7 15.6 5.5 31.6 8.2 48.2 8.2 12.7 0 25.8-1.2 36.3-4.3.7-.3 36-8.9 45.6-45.8 3.5-13.5 2.4-26.5-3.1-39.1zM376.2 149.8c-31.7 0-104.2 20.1-104.2 103.5v183.5c0 .8.6 2.7 2.7 2.7h40.9c1.9 0 2.8-.9 2.8-2.7V348c16.5 12.7 35.8 19.1 57.7 19.1 60.5 0 108.7-48.5 108.7-108.7.1-60.3-48.2-108.6-108.6-108.6zm0 170.9c-17.2 0-31.9-6.1-44-18.2-12.2-12.2-18.2-26.8-18.2-44 0-34.5 27.6-62.2 62.2-62.2 34.5 0 62.2 27.6 62.2 62.2.1 34.3-27.3 62.2-62.2 62.2zM228.3 72.5c-15.9 0-28.8 12.9-28.9 28.9 0 15.6 12.7 28.9 28.9 28.9s28.9-13.1 28.9-28.9c0-16.2-13-28.9-28.9-28.9z"
        /> < title > { title } < / title > < / svg >
    }
}
