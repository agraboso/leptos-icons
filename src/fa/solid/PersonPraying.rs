#[cfg(feature = "FaSolidPersonPraying")]
use leptos::*;
#[cfg(feature = "FaSolidPersonPraying")]
///This icon requires the feature `FaSolidPersonPraying` to be enabled.
#[component]
pub fn PersonPraying(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M320 64A64 64 0 1 0 192 64a64 64 0 1 0 128 0zM200.7 264l22.9 31.5c6.5 8.9 16.3 14.7 27.2 16.1s21.9-1.7 30.4-8.7l88-72c17.1-14 19.6-39.2 5.6-56.3s-39.2-19.6-56.3-5.6l-55.2 45.2-26.2-36C221.6 156.7 196.6 144 170 144c-30.9 0-59.2 17.1-73.6 44.4L47.8 280.9c-20.2 38.5-9.4 85.9 25.6 111.8L126.6 432H40c-22.1 0-40 17.9-40 40s17.9 40 40 40H248c17.3 0 32.6-11.1 38-27.5s-.3-34.4-14.2-44.7L155.7 354l45-90z"
        /> < title > { title } < / title > < / svg >
    }
}
