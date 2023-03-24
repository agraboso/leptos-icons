#[cfg(feature = "TiScissors")]
use leptos::*;
#[cfg(feature = "TiScissors")]
///This icon requires the feature `TiScissors` to be enabled.
#[component]
pub fn Scissors(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.625 5.515c-1-1.522-2.915-1.67-4.397-.824l-.186.107-.076-.003c-1.042 0-2.01.511-2.604 1.369l-.034.045c-.43.645-.723 1.236-1.005 1.809-.255.516-.5 1.01-.824 1.483-.325-.475-.57-.97-.826-1.486-.283-.571-.575-1.162-1.004-1.806l-.033-.044c-.593-.859-1.562-1.37-2.603-1.37-1.747 0-3.167 1.42-3.167 3.166 0 1.747 1.421 3.168 3.167 3.168.775 0 1.515-.287 2.087-.791l.652 1.198c-1.621 1.876-2.979 4.054-3.019 4.121-1.236 1.702.705 4.42.789 4.534.094.131.245.207.405.207.204-.012.357-.11.439-.261l3.112-5.717 3.113 5.717c.082.15.235.249.407.26.174.019.336-.066.437-.206.083-.114 2.024-2.832.809-4.504l-.323-.521c-1.076-1.736-1.187-1.916-2.715-3.634l.651-1.195c.572.504 1.313.791 2.088.791 1.746 0 3.167-1.421 3.167-3.168 0-.634-.191-1.246-.547-1.768.472-.27.997-.123 1.456.095.466.191.897-.377.584-.772zm-13.625 3.485c-.552 0-1-.447-1-1s.448-1 1-1 1 .447 1 1-.448 1-1 1zm4.5 3.395c-.277 0-.5-.225-.5-.5 0-.277.223-.5.5-.5s.5.223.5.5c0 .275-.223.5-.5.5zm4.5-3.395c-.552 0-1-.447-1-1s.448-1 1-1 1 .447 1 1-.448 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
