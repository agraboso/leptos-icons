#[cfg(feature = "ImScissors")]
use leptos::*;
#[cfg(feature = "ImScissors")]
///This icon requires the feature `ImScissors` to be enabled.
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M14.279 10.62c-1.042-1.628-2.829-2.345-3.992-1.601-0.1 0.064-0.193 0.138-0.277 0.218l-1.241-1.942 2.867-4.5c0.235-0.433 0.321-0.949 0.207-1.468-0.109-0.496-0.383-0.913-0.752-1.207l-0.192-0.122-3.398 5.314-3.398-5.314-0.192 0.122c-0.369 0.294-0.643 0.711-0.752 1.207-0.114 0.519-0.027 1.035 0.207 1.468l2.867 4.5-1.241 1.942c-0.085-0.081-0.177-0.154-0.277-0.218-1.163-0.744-2.95-0.028-3.992 1.601s-0.944 3.551 0.219 4.296c1.163 0.744 2.95 0.028 3.992-1.601l2.567-4.029 2.567 4.029c1.042 1.628 2.829 2.345 3.992 1.601s1.261-2.667 0.219-4.296zM3.67 12.507c-0.469 0.733-1.071 1.089-1.478 1.179-0 0-0 0-0 0-0.133 0.029-0.317 0.047-0.443-0.033-0.139-0.089-0.231-0.324-0.247-0.629-0.025-0.494 0.151-1.076 0.483-1.594 0.469-0.733 1.071-1.089 1.478-1.179 0.133-0.029 0.317-0.047 0.443 0.033 0.139 0.089 0.231 0.324 0.247 0.629 0.025 0.495-0.151 1.076-0.483 1.594zM7.5 8c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5 0.5 0.224 0.5 0.5-0.224 0.5-0.5 0.5zM13.498 13.023c-0.016 0.305-0.108 0.54-0.247 0.629-0.125 0.080-0.31 0.062-0.443 0.033 0 0 0 0-0 0-0.407-0.089-1.009-0.446-1.478-1.179-0.332-0.519-0.508-1.1-0.483-1.594 0.016-0.305 0.108-0.54 0.247-0.629 0.125-0.080 0.31-0.062 0.443-0.033 0.407 0.089 1.009 0.446 1.478 1.179 0.332 0.519 0.508 1.1 0.483 1.594z"
        /> < title > { title } < / title > < / svg >
    }
}
