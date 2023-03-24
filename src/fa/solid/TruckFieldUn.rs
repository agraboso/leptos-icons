#[cfg(feature = "FaSolidTruckFieldUn")]
use leptos::*;
#[cfg(feature = "FaSolidTruckFieldUn")]
///This icon requires the feature `FaSolidTruckFieldUn` to be enabled.
#[component]
pub fn TruckFieldUn(
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
        "M96 32C60.7 32 32 60.7 32 96v32c-17.7 0-32 14.3-32 32v96c0 17.7 14.3 32 32 32v32c-17.7 0-32 14.3-32 32s14.3 32 32 32H64c0 53 43 96 96 96s96-43 96-96H384c0 53 43 96 96 96s96-43 96-96h32c17.7 0 32-14.3 32-32s-14.3-32-32-32V288c0-35.3-28.7-64-64-64h-4.2c-.4-1.1-.9-2.1-1.3-3.2L485.7 102c-10.3-23.1-33.2-38-58.5-38H375.4C364.4 44.9 343.7 32 320 32H96zm288 96h43.2l42.7 96H384V128zM112 384a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zm368-48a48 48 0 1 1 0 96 48 48 0 1 1 0-96zM253.3 135.1l34.7 52V144c0-8.8 7.2-16 16-16s16 7.2 16 16v96c0 7.1-4.6 13.3-11.4 15.3s-14-.6-17.9-6.4l-34.7-52V240c0 8.8-7.2 16-16 16s-16-7.2-16-16V144c0-7.1 4.6-13.3 11.4-15.3s14 .6 17.9 6.4zM128 144v64c0 8.8 7.2 16 16 16s16-7.2 16-16V144c0-8.8 7.2-16 16-16s16 7.2 16 16v64c0 26.5-21.5 48-48 48s-48-21.5-48-48V144c0-8.8 7.2-16 16-16s16 7.2 16 16z"
        /> < title > { title } < / title > < / svg >
    }
}
