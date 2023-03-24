#[cfg(feature = "VsFileBinary")]
use leptos::*;
#[cfg(feature = "VsFileBinary")]
///This icon requires the feature `VsFileBinary` to be enabled.
#[component]
pub fn FileBinary(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10.57 1.14l3.28 3.3.15.36v9.7l-.5.5h-11l-.5-.5v-13l.5-.5h7.72l.35.14zM3 2v12h10V5l-3-3H3zm1.46 4.052c0 1.287.458 1.93 1.374 1.93.457 0 .807-.173 1.05-.52.246-.348.368-.847.368-1.499C7.252 4.654 6.805 4 5.91 4c-.471 0-.831.175-1.08.526-.247.35-.37.858-.37 1.526zm.862-.022c0-.922.183-1.383.55-1.383.344 0 .516.448.516 1.343s-.176 1.343-.527 1.343c-.36 0-.54-.434-.54-1.303zm3.187 1.886h2.435v-.672h-.792V4l-1.665.336v.687l.82-.177v2.398h-.798v.672zm-1.337 5H4.736v-.672h.798V9.846l-.82.177v-.687L6.38 9v3.244h.792v.671zm1.035-1.931c0 1.287.458 1.93 1.375 1.93.457 0 .807-.173 1.05-.52.245-.348.368-.847.368-1.499 0-1.309-.448-1.963-1.343-1.963-.47 0-.83.175-1.08.526-.246.35-.37.858-.37 1.526zm.862-.022c0-.922.184-1.383.55-1.383.344 0 .516.448.516 1.343s-.175 1.343-.526 1.343c-.36 0-.54-.434-.54-1.303z"
        /> < title > { title } < / title > < / svg >
    }
}
