#[cfg(feature = "FaSolidTemperatureHigh")]
use leptos::*;
#[cfg(feature = "FaSolidTemperatureHigh")]
///This icon requires the feature `FaSolidTemperatureHigh` to be enabled.
#[component]
pub fn TemperatureHigh(
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
        "M416 64a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm0 128A96 96 0 1 0 416 0a96 96 0 1 0 0 192zM96 112c0-26.5 21.5-48 48-48s48 21.5 48 48V276.5c0 17.3 7.1 31.9 15.3 42.5C217.8 332.6 224 349.5 224 368c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-18.5 6.2-35.4 16.7-48.9C88.9 308.4 96 293.8 96 276.5V112zM144 0C82.1 0 32 50.2 32 112V276.5c0 .1-.1 .3-.2 .6c-.2 .6-.8 1.6-1.7 2.8C11.2 304.2 0 334.8 0 368c0 79.5 64.5 144 144 144s144-64.5 144-144c0-33.2-11.3-63.8-30.1-88.1c-.9-1.2-1.5-2.2-1.7-2.8c-.1-.3-.2-.5-.2-.6V112C256 50.2 205.9 0 144 0zm0 416c26.5 0 48-21.5 48-48c0-20.9-13.4-38.7-32-45.3V112c0-8.8-7.2-16-16-16s-16 7.2-16 16V322.7c-18.6 6.6-32 24.4-32 45.3c0 26.5 21.5 48 48 48z"
        /> < title > { title } < / title > < / svg >
    }
}
