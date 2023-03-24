#[cfg(feature = "ImPointLeft")]
use leptos::*;
#[cfg(feature = "ImPointLeft")]
///This icon requires the feature `ImPointLeft` to be enabled.
#[component]
pub fn PointLeft(
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
        "M9.5 15h-2.5c-0.827 0-1.5-0.673-1.5-1.5 0-0.267 0.070-0.518 0.193-0.736-0.417-0.267-0.693-0.734-0.693-1.264 0-0.384 0.145-0.734 0.383-1-0.238-0.266-0.383-0.616-0.383-1 0-0.175 0.030-0.344 0.086-0.5h-3.586c-0.827 0-1.5-0.673-1.5-1.5s0.673-1.5 1.5-1.5h6.167l-1.466-2.75c-0.131-0.227-0.201-0.486-0.201-0.75 0-0.827 0.673-1.5 1.5-1.5 0.412 0 0.796 0.164 1.082 0.461 0.004 0.004 0.007 0.008 0.011 0.012l3.407 3.737v-0.71c0-0.276 0.224-0.5 0.5-0.5h3c0.276 0 0.5 0.224 0.5 0.5v10c0 0.276-0.224 0.5-0.5 0.5h-3c-0.276 0-0.5-0.224-0.5-0.5v-0.691l-2.276 1.138c-0.069 0.035-0.146 0.053-0.224 0.053zM13.5 14c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5-0.5 0.224-0.5 0.5 0.224 0.5 0.5 0.5zM9.382 14l2.618-1.309v-5.997l-4.143-4.544c-0.095-0.097-0.221-0.15-0.357-0.15-0.276 0-0.5 0.224-0.5 0.5 0 0.085 0.020 0.166 0.061 0.239 0.005 0.008 0.010 0.017 0.014 0.025l1.866 3.5c0.083 0.155 0.078 0.342-0.012 0.492s-0.253 0.243-0.429 0.243h-7c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h5c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h0.5c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h2.382z"
        /> < title > { title } < / title > < / svg >
    }
}
