#[cfg(feature = "VsLiveShare")]
use leptos::*;
#[cfg(feature = "VsLiveShare")]
///This icon requires the feature `VsLiveShare` to be enabled.
#[component]
pub fn LiveShare(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M13.735 1.694L15.178 1l8.029 6.328v1.388l-8.029 6.072-1.443-.694v-2.776h-.59c-4.06-.02-6.71.104-10.61 5.163l-1.534-.493a8.23 8.23 0 0 1 .271-2.255 11.026 11.026 0 0 1 3.92-6.793 11.339 11.339 0 0 1 7.502-2.547h1.04v-2.7zm1.804 7.917v2.776l5.676-4.281-5.648-4.545v2.664h-2.86A9.299 9.299 0 0 0 5.77 8.848a10.444 10.444 0 0 0-2.401 4.122c3.351-3.213 6.19-3.359 9.798-3.359h2.373zm-7.647 5.896a4.31 4.31 0 1 1 4.788 7.166 4.31 4.31 0 0 1-4.788-7.166zm.955 5.728a2.588 2.588 0 1 0 2.878-4.302 2.588 2.588 0 0 0-2.878 4.302z"
        /> < title > { title } < / title > < / svg >
    }
}
