#[cfg(feature = "ImXing")]
use leptos::*;
#[cfg(feature = "ImXing")]
///This icon requires the feature `ImXing` to be enabled.
#[component]
pub fn Xing(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM4.884 10.406h-1.728c-0.103 0-0.181-0.047-0.225-0.119-0.047-0.075-0.047-0.169 0-0.266l1.838-3.244c0.003-0.003 0.003-0.006 0-0.009l-1.169-2.025c-0.047-0.097-0.056-0.191-0.009-0.266 0.044-0.072 0.131-0.109 0.237-0.109h1.731c0.266 0 0.397 0.172 0.481 0.325 0 0 1.181 2.063 1.191 2.075-0.069 0.125-1.869 3.303-1.869 3.303-0.094 0.162-0.219 0.334-0.478 0.334zM13.069 2.378l-3.831 6.775c-0.003 0.003-0.003 0.009 0 0.012l2.441 4.456c0.047 0.097 0.050 0.194 0.003 0.269-0.044 0.072-0.125 0.109-0.231 0.109h-1.728c-0.266 0-0.397-0.175-0.484-0.328 0 0-2.453-4.5-2.459-4.512 0.122-0.216 3.85-6.828 3.85-6.828 0.094-0.166 0.206-0.328 0.463-0.328h1.753c0.103 0 0.188 0.041 0.231 0.109 0.044 0.072 0.044 0.169-0.006 0.266z"
        /> < title > { title } < / title > < / svg >
    }
}
