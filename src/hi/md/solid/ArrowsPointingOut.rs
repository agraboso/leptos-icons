#[cfg(feature = "HiMdSolidArrowsPointingOut")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowsPointingOut")]
///This icon requires the feature `HiMdSolidArrowsPointingOut` to be enabled.
#[component]
pub fn ArrowsPointingOut(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.2803 7.78033L16.5 4.56066V7.25C16.5 7.66421 16.8358 8 17.25 8C17.6642 8 18 7.66421 18 7.25V2.75C18 2.33579 17.6642 2 17.25 2H12.75C12.3358 2 12 2.33579 12 2.75C12 3.16421 12.3358 3.5 12.75 3.5H15.4393L12.2197 6.71967C11.9268 7.01256 11.9268 7.48744 12.2197 7.78033C12.5126 8.07322 12.9874 8.07322 13.2803 7.78033Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 17.25V12.75C2 12.3358 2.33579 12 2.75 12C3.16421 12 3.5 12.3358 3.5 12.75V15.4393L6.71967 12.2197C7.01256 11.9268 7.48744 11.9268 7.78033 12.2197C8.07322 12.5126 8.07322 12.9874 7.78033 13.2803L4.56066 16.5H7.25C7.66421 16.5 8 16.8358 8 17.25C8 17.6642 7.66421 18 7.25 18H2.75C2.55806 18 2.36612 17.9268 2.21967 17.7803C2.14776 17.7084 2.09351 17.6255 2.05691 17.5371C2.02024 17.4487 2 17.3517 2 17.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.2197 13.2803L15.4393 16.5H12.75C12.3358 16.5 12 16.8358 12 17.25C12 17.6642 12.3358 18 12.75 18H17.25C17.4419 18 17.6339 17.9268 17.7803 17.7803C17.8522 17.7084 17.9065 17.6255 17.9431 17.5371C17.9798 17.4487 18 17.3517 18 17.25V12.75C18 12.3358 17.6642 12 17.25 12C16.8358 12 16.5 12.3358 16.5 12.75V15.4393L13.2803 12.2197C12.9874 11.9268 12.5126 11.9268 12.2197 12.2197C11.9268 12.5126 11.9268 12.9874 12.2197 13.2803Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 4.56066L6.71967 7.78033C7.01256 8.07322 7.48744 8.07322 7.78033 7.78033C8.07322 7.48744 8.07322 7.01256 7.78033 6.71967L4.56066 3.5H7.25C7.66421 3.5 8 3.16421 8 2.75C8 2.33579 7.66421 2 7.25 2H2.75C2.33579 2 2 2.33579 2 2.75V7.25C2 7.66421 2.33579 8 2.75 8C3.16421 8 3.5 7.66421 3.5 7.25V4.56066Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
