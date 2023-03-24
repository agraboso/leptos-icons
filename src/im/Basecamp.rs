#[cfg(feature = "ImBasecamp")]
use leptos::*;
#[cfg(feature = "ImBasecamp")]
///This icon requires the feature `ImBasecamp` to be enabled.
#[component]
pub fn Basecamp(
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
        "M8 1.666c-2.919 0-5.169 2.444-6.444 4.838-0.719 1.347-1.222 2.822-1.453 4.331-0.025 0.172-0.050 0.344-0.069 0.519-0.009 0.094-0.019 0.188-0.025 0.281-0.009 0.119-0.003 0.156 0.059 0.256 0.187 0.303 0.409 0.584 0.659 0.838 0.512 0.525 1.134 0.928 1.794 1.241 1.503 0.709 3.2 0.966 4.85 1.022 1.703 0.056 3.453-0.084 5.081-0.616 1.391-0.453 2.731-1.244 3.503-2.522 0.084-0.137 0.025-0.341 0.009-0.5-0.019-0.191-0.044-0.378-0.075-0.566-0.056-0.369-0.131-0.731-0.222-1.094-0.181-0.738-0.428-1.463-0.728-2.159-1.088-2.525-3.1-5.219-5.963-5.775-0.322-0.063-0.65-0.094-0.978-0.094zM8.1 13.909c-1.784 0-3.728-0.159-5.334-1.019-0.625-0.334-1.262-0.819-1.563-1.484-0.087-0.194-0.056-0.269-0.016-0.497 0.028-0.147 0.041-0.291 0.106-0.428 0.091-0.191 0.184-0.378 0.281-0.566 0.328-0.634 0.681-1.262 1.091-1.853 0.203-0.291 0.419-0.578 0.669-0.828 0.175-0.175 0.388-0.362 0.634-0.422 0.756-0.181 1.334 0.694 1.794 1.134 0.222 0.213 0.519 0.453 0.85 0.412 0.228-0.028 0.431-0.206 0.594-0.353 0.553-0.497 0.997-1.112 1.456-1.691 0.228-0.284 0.453-0.572 0.7-0.844 0.166-0.184 0.347-0.394 0.569-0.513 0.397-0.216 0.903 0.228 1.178 0.456 0.469 0.391 0.884 0.847 1.281 1.309 0.378 0.441 0.744 0.888 1.066 1.372 0.497 0.75 0.928 1.55 1.322 2.359 0.084 0.175 0.113 0.294 0.144 0.488 0.019 0.106 0.059 0.228 0.044 0.338-0.022 0.153-0.128 0.319-0.206 0.444-0.188 0.297-0.441 0.553-0.719 0.769-1.166 0.903-2.744 1.203-4.178 1.338-0.588 0.056-1.175 0.078-1.762 0.078z"
        /> < title > { title } < / title > < / svg >
    }
}
