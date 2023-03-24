#[cfg(feature = "FaSolidTowerObservation")]
use leptos::*;
#[cfg(feature = "FaSolidTowerObservation")]
///This icon requires the feature `FaSolidTowerObservation` to be enabled.
#[component]
pub fn TowerObservation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M241.7 3.4c9-4.5 19.6-4.5 28.6 0l160 80c15.8 7.9 22.2 27.1 14.3 42.9C439 137.5 427.7 144 416 144v80c0 17.7-14.3 32-32 32h-4.9l32 192H480c17.7 0 32 14.3 32 32s-14.3 32-32 32H384.5c-.4 0-.8 0-1.1 0H128.6c-.4 0-.8 0-1.1 0H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l32-192H128c-17.7 0-32-14.3-32-32V144c-11.7 0-23-6.5-28.6-17.7c-7.9-15.8-1.5-35 14.3-42.9l160-80zM314.5 448L256 399.2 197.5 448h117zM197.8 256l-4.7 28.3L256 336.8l62.9-52.5L314.2 256H197.8zm-13.9 83.2l-11.2 67L218.5 368l-34.6-28.8zM293.5 368l45.8 38.1-11.2-67L293.5 368zM176 128c-8.8 0-16 7.2-16 16s7.2 16 16 16H336c8.8 0 16-7.2 16-16s-7.2-16-16-16H176z"
        /> < title > { title } < / title > < / svg >
    }
}
