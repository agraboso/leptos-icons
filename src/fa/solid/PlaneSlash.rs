#[cfg(feature = "FaSolidPlaneSlash")]
use leptos::*;
#[cfg(feature = "FaSolidPlaneSlash")]
///This icon requires the feature `FaSolidPlaneSlash` to be enabled.
#[component]
pub fn PlaneSlash(
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
        "M514.3 192c34.2 0 93.7 29 93.7 64c0 36-59.5 64-93.7 64H440.6L630.8 469.1c10.4 8.2 12.3 23.3 4.1 33.7s-23.3 12.3-33.7 4.1L9.2 42.9C-1.2 34.7-3.1 19.6 5.1 9.2S28.4-3.1 38.8 5.1L238.1 161.3 197.8 20.4C194.9 10.2 202.6 0 213.2 0h56.2c11.5 0 22.1 6.2 27.8 16.1L397.7 192l116.6 0zM41.5 128.7l321 252.9L297.2 495.9c-5.7 10-16.3 16.1-27.8 16.1l-56.2 0c-10.6 0-18.3-10.2-15.4-20.4l49-171.6H144l-43.2 57.6c-3 4-7.8 6.4-12.8 6.4H46c-7.8 0-14-6.3-14-14c0-1.3 .2-2.6 .5-3.9L64 256 32.5 145.9c-.4-1.3-.5-2.6-.5-3.9c0-6.2 4-11.4 9.5-13.3z"
        /> < title > { title } < / title > < / svg >
    }
}
