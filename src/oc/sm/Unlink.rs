#[cfg(feature = "OcSmUnlink")]
use leptos::*;
#[cfg(feature = "OcSmUnlink")]
///This icon requires the feature `OcSmUnlink` to be enabled.
#[component]
pub fn Unlink(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.914 5.914a2 2 0 0 0-2.828-2.828l-.837.837a.75.75 0 1 1-1.06-1.061l.836-.837a3.5 3.5 0 1 1 4.95 4.95l-.195.194a.75.75 0 0 1-1.06-1.06l.194-.195Zm-1.87 3.482a.759.759 0 0 1-.07.079c-.63.63-1.468 1.108-2.343 1.263-.89.159-1.86-.017-2.606-.763a.75.75 0 1 1 1.06-1.06c.329.327.767.438 1.284.347.493-.088 1.018-.36 1.445-.752l-1.247-.897a.709.709 0 0 1-.01-.008l-.295-.212c-.94-.597-1.984-.499-2.676.193l-2.5 2.5a2 2 0 1 0 2.828 2.828l.837-.836a.75.75 0 0 1 1.06 1.06l-.836.837a3.5 3.5 0 0 1-4.95-4.95l2.5-2.5a3.472 3.472 0 0 1 1.354-.848L2.312 3.109a.75.75 0 0 1 .876-1.218l5.93 4.27c.115.074.226.155.335.24l6.235 4.49a.75.75 0 0 1-.876 1.218l-3.768-2.713Z"
        /> < title > { title } < / title > < / svg >
    }
}
