#[cfg(feature = "ImYelp")]
use leptos::*;
#[cfg(feature = "ImYelp")]
///This icon requires the feature `ImYelp` to be enabled.
#[component]
pub fn Yelp(
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
        "M9.514 10.21c-0.27 0.272-0.042 0.768-0.042 0.768l2.033 3.394c0 0 0.334 0.447 0.623 0.447 0.29 0 0.577-0.239 0.577-0.239l1.607-2.297c0 0 0.162-0.29 0.166-0.544 0.006-0.361-0.538-0.46-0.538-0.46l-3.805-1.222c-0 0-0.373-0.099-0.621 0.152zM9.321 8.5c0.195 0.33 0.732 0.234 0.732 0.234l3.796-1.109c0 0 0.517-0.21 0.591-0.491 0.072-0.281-0.085-0.619-0.085-0.619l-1.814-2.137c0 0-0.157-0.27-0.483-0.297-0.36-0.031-0.581 0.405-0.581 0.405l-2.145 3.375c0 0-0.19 0.336-0.010 0.64zM7.527 7.184c0.447-0.11 0.518-0.759 0.518-0.759l-0.030-5.404c0 0-0.067-0.667-0.367-0.847-0.47-0.285-0.609-0.136-0.744-0.116l-3.151 1.171c0 0-0.309 0.102-0.469 0.36-0.23 0.365 0.233 0.899 0.233 0.899l3.276 4.465c0 0 0.323 0.334 0.735 0.233zM6.749 9.371c0.011-0.417-0.5-0.667-0.5-0.667l-3.387-1.711c0 0-0.502-0.207-0.746-0.063-0.187 0.11-0.352 0.31-0.368 0.486l-0.221 2.716c0 0-0.033 0.471 0.089 0.685 0.173 0.304 0.741 0.092 0.741 0.092l3.955-0.874c0.154-0.103 0.423-0.113 0.438-0.664zM7.732 10.837c-0.339-0.174-0.746 0.187-0.746 0.187l-2.648 2.915c0 0-0.33 0.446-0.246 0.72 0.079 0.257 0.21 0.384 0.396 0.474l2.659 0.839c0 0 0.322 0.067 0.567-0.004 0.347-0.1 0.283-0.643 0.283-0.643l0.060-3.947c-0 0-0.014-0.38-0.324-0.541z"
        /> < title > { title } < / title > < / svg >
    }
}
