#[cfg(feature = "VsWorkspaceUntrusted")]
use leptos::*;
#[cfg(feature = "VsWorkspaceUntrusted")]
///This icon requires the feature `VsWorkspaceUntrusted` to be enabled.
#[component]
pub fn WorkspaceUntrusted(
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
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.067 0c.263 0 .52.017.76.057a4.1 4.1 0 0 1 .697.154c.229.069.452.155.675.263.217.103.44.229.662.366a7.2 7.2 0 0 0 1.126.589 7.534 7.534 0 0 0 2.332.525c.405.029.822.046 1.257.046v4c0 .76-.097 1.48-.292 2.166a8.996 8.996 0 0 1-.788 1.943 10.306 10.306 0 0 1-1.189 1.725 15.082 15.082 0 0 1-1.491 1.532 17.57 17.57 0 0 1-1.703 1.325c-.594.412-1.194.795-1.794 1.143l-.24.143-.24-.143a27.088 27.088 0 0 1-1.806-1.143 15.579 15.579 0 0 1-1.703-1.325 15.08 15.08 0 0 1-1.491-1.532 10.948 10.948 0 0 1-1.195-1.725 9.753 9.753 0 0 1-.788-1.943A7.897 7.897 0 0 1 .57 6V2c.434 0 .851-.017 1.257-.046a8.16 8.16 0 0 0 1.189-.171c.383-.086.765-.2 1.143-.354a6.563 6.563 0 0 0 1.12-.583C5.719.56 6.164.349 6.604.21A4.79 4.79 0 0 1 8.067 0zm6.503 2.983a9.567 9.567 0 0 1-2.234-.377 7.96 7.96 0 0 1-2.046-.943 4.264 4.264 0 0 0-1.063-.503A3.885 3.885 0 0 0 8.073.994a3.99 3.99 0 0 0-1.166.166 3.946 3.946 0 0 0-1.057.503 7.927 7.927 0 0 1-2.051.948c-.709.206-1.452.332-2.229.378v3.017c0 .663.086 1.297.257 1.908a8.58 8.58 0 0 0 .72 1.743 9.604 9.604 0 0 0 1.08 1.572c.417.491.863.948 1.343 1.382.48.435.983.835 1.509 1.206.531.372 1.062.709 1.594 1.017a22.4 22.4 0 0 0 1.588-1.017 15.384 15.384 0 0 0 1.515-1.206c.48-.434.925-.891 1.343-1.382a9.609 9.609 0 0 0 1.08-1.572 8.269 8.269 0 0 0 .708-1.743 6.814 6.814 0 0 0 .263-1.908V2.983z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.787 5.446l-.4-.406h-.206L8.2 7.023 6.216 5.04h-.2l-.406.406v.2l1.983 1.983L5.61 9.61v.206l.406.4h.2l1.983-1.983 1.982 1.983h.206l.4-.4V9.61L8.804 7.63l1.983-1.983v-.2z"
        /> < title > { title } < / title > < / svg >
    }
}
