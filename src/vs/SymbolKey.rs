#[cfg(feature = "VsSymbolKey")]
use leptos::*;
#[cfg(feature = "VsSymbolKey")]
///This icon requires the feature `VsSymbolKey` to be enabled.
#[component]
pub fn SymbolKey(
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
        "M7.223 10.933c.326.192.699.29 1.077.282a2.159 2.159 0 0 0 1.754-.842 3.291 3.291 0 0 0 .654-2.113 2.886 2.886 0 0 0-.576-1.877 1.99 1.99 0 0 0-1.634-.733 2.294 2.294 0 0 0-1.523.567V3.475h-.991V11.1h.995v-.344c.076.066.158.125.244.177zM7.85 6.7c.186-.079.388-.113.59-.1a1.08 1.08 0 0 1 .896.428c.257.363.382.802.357 1.245a2.485 2.485 0 0 1-.4 1.484 1.133 1.133 0 0 1-.96.508 1.224 1.224 0 0 1-.976-.417A1.522 1.522 0 0 1 6.975 8.8v-.6a1.722 1.722 0 0 1 .393-1.145c.13-.154.296-.276.482-.355zM3.289 5.675a3.03 3.03 0 0 0-.937.162 2.59 2.59 0 0 0-.8.4l-.1.077v1.2l.423-.359a2.1 2.1 0 0 1 1.366-.572.758.758 0 0 1 .661.282c.15.232.23.503.231.779L2.9 7.825a2.6 2.6 0 0 0-1.378.575 1.65 1.65 0 0 0-.022 2.336 1.737 1.737 0 0 0 1.253.454 1.96 1.96 0 0 0 1.107-.332c.102-.068.197-.145.286-.229v.444h.941V7.715a2.193 2.193 0 0 0-.469-1.5 1.687 1.687 0 0 0-1.329-.54zm.857 3.041c.02.418-.12.829-.391 1.148a1.221 1.221 0 0 1-.955.422.832.832 0 0 1-.608-.2.833.833 0 0 1 0-1.091c.281-.174.6-.277.93-.3l1.02-.148.004.169zm8.313 2.317c.307.13.64.193.973.182.495.012.983-.114 1.41-.365l.123-.075.013-.007V9.615l-.446.32c-.316.224-.696.34-1.084.329A1.3 1.3 0 0 1 12.4 9.8a1.975 1.975 0 0 1-.4-1.312 2.01 2.01 0 0 1 .453-1.381A1.432 1.432 0 0 1 13.6 6.6a1.8 1.8 0 0 1 .971.279l.43.265V5.97l-.17-.073a2.9 2.9 0 0 0-1.17-.247 2.52 2.52 0 0 0-1.929.817 2.9 2.9 0 0 0-.747 2.049c-.028.707.21 1.4.67 1.939.222.249.497.446.804.578z"
        /> < title > { title } < / title > < / svg >
    }
}
