#[cfg(feature = "SiLodash")]
use leptos::*;
#[cfg(feature = "SiLodash")]
///This icon requires the feature `SiLodash` to be enabled.
#[component]
pub fn Lodash(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "m0 20.253h24v2.542h-24zm18.061-15.041.223.031c1.933-.071 3.885 1.006 4.882 2.674.844 1.566.976 3.458.712 5.187-.204 1.657-1.149 3.234-2.644 4.027-2.177 1.139-5.085 1.017-7.017-.59-1.994-1.942-2.461-5.136-1.444-7.678.711-2.207 3-3.661 5.288-3.63zm.234 1.8h-.183c-1.424-.03-2.777.915-3.285 2.237-.732 1.831-.732 4.17.691 5.695 1.17 1.434 3.458 1.597 4.882.438 1.525-1.312 1.83-3.59 1.322-5.451-.275-1.648-1.78-2.929-3.458-2.929zm-18.295-5.807h2.237v14.847h8.848v1.831h-11.085z"
        /> < title > { title } < / title > < / svg >
    }
}
