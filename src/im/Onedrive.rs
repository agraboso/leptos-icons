#[cfg(feature = "ImOnedrive")]
use leptos::*;
#[cfg(feature = "ImOnedrive")]
///This icon requires the feature `ImOnedrive` to be enabled.
#[component]
pub fn Onedrive(
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
        "M5.482 12.944c-0.942-0.235-1.466-0.984-1.468-2.095-0-0.355 0.025-0.525 0.114-0.754 0.217-0.56 0.793-0.982 1.55-1.138 0.377-0.077 0.493-0.16 0.493-0.353 0-0.060 0.045-0.24 0.1-0.399 0.249-0.724 0.71-1.327 1.202-1.573 0.515-0.258 0.776-0.316 1.399-0.313 0.886 0.005 1.327 0.197 1.945 0.846l0.34 0.357 0.304-0.105c1.473-0.51 2.942 0.358 3.061 1.809l0.032 0.397 0.29 0.104c0.829 0.297 1.218 0.92 1.148 1.837-0.046 0.599-0.326 1.078-0.77 1.315l-0.209 0.112-4.638 0.009c-3.564 0.007-4.697-0.006-4.893-0.055v0zM1.613 12.281c-0.565-0.142-1.164-0.67-1.445-1.273-0.159-0.342-0.168-0.393-0.168-0.998 0-0.576 0.014-0.668 0.14-0.954 0.267-0.603 0.78-1.038 1.422-1.21 0.136-0.036 0.263-0.094 0.283-0.128s0.043-0.221 0.050-0.415c0.045-1.206 0.794-2.269 1.839-2.61 0.565-0.184 1.306-0.202 1.92 0.058 0.195 0.082 0.173 0.1 0.585-0.471 0.244-0.338 0.705-0.695 1.108-0.909 0.435-0.231 0.887-0.337 1.428-0.336 1.512 0.004 2.815 1.003 3.297 2.529 0.154 0.487 0.146 0.624-0.035 0.628-0.079 0.002-0.306 0.048-0.505 0.102l-0.361 0.099-0.329-0.348c-0.928-0.98-2.441-1.192-3.728-0.522-0.514 0.268-0.927 0.652-1.239 1.153-0.222 0.357-0.506 1.024-0.506 1.189 0 0.117-0.090 0.176-0.474 0.309-1.189 0.412-1.883 1.364-1.882 2.582 0 0.443 0.108 0.986 0.258 1.296 0.057 0.117 0.088 0.228 0.070 0.247-0.046 0.049-1.525 0.032-1.73-0.019v0z"
        /> < title > { title } < / title > < / svg >
    }
}
