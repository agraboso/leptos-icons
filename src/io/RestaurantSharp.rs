#[cfg(feature = "IoRestaurantSharp")]
use leptos::*;
#[cfg(feature = "IoRestaurantSharp")]
///This icon requires the feature `IoRestaurantSharp` to be enabled.
#[component]
pub fn RestaurantSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M342.7,223.94h14.87a79.48,79.48,0,0,0,56.58-23.44L496,118,473.78,95.6,390.2,179.18l-17.37-17.37,83.58-83.59-23-22.31-83.27,83.26L332.82,121.8,416.4,38.21,394,16,311.5,97.85a79.49,79.49,0,0,0-23.44,56.59V169.3l-43.13,43.13L48,16C3.72,70.87,29.87,171.71,79.72,221.57l85.5,85.5c26.55,26.55,31.82,28.92,61.94,16.8,6.49-2.61,8.85-2.32,14.9,3.72l13,12.13c2.93,3,3,3.88,3,9.62v5.54c0,21.08,13.48,33.2,22.36,42.24L384,496l72-72L299.57,267.07Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M227.37,354.59c-29.82,6.11-48.11,11.74-83.08-23.23-.56-.56-1.14-1.1-1.7-1.66l-19.5-19.5L16,416l80,80L240,352Z"
        /> < title > { title } < / title > < / svg >
    }
}
