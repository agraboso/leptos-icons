#[cfg(feature = "FaSolidBezierCurve")]
use leptos::*;
#[cfg(feature = "FaSolidBezierCurve")]
///This icon requires the feature `FaSolidBezierCurve` to be enabled.
#[component]
pub fn BezierCurve(
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
        "M296 136V88h48v48H296zM288 32c-26.5 0-48 21.5-48 48v4H121.6C111.2 62.7 89.3 48 64 48C28.7 48 0 76.7 0 112s28.7 64 64 64c25.3 0 47.2-14.7 57.6-36h66.9c-58.9 39.6-98.9 105-104 180H80c-26.5 0-48 21.5-48 48v64c0 26.5 21.5 48 48 48h64c26.5 0 48-21.5 48-48V368c0-26.5-21.5-48-48-48h-3.3c5.9-67 48.5-123.4 107.5-149.1c8.6 12.7 23.2 21.1 39.8 21.1h64c16.6 0 31.1-8.4 39.8-21.1c59 25.7 101.6 82.1 107.5 149.1H496c-26.5 0-48 21.5-48 48v64c0 26.5 21.5 48 48 48h64c26.5 0 48-21.5 48-48V368c0-26.5-21.5-48-48-48h-4.5c-5-75-45.1-140.4-104-180h66.9c10.4 21.3 32.3 36 57.6 36c35.3 0 64-28.7 64-64s-28.7-64-64-64c-25.3 0-47.2 14.7-57.6 36H400V80c0-26.5-21.5-48-48-48H288zM88 376h48v48H88V376zm416 48V376h48v48H504z"
        /> < title > { title } < / title > < / svg >
    }
}
