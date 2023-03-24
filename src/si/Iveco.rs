#[cfg(feature = "SiIveco")]
use leptos::*;
#[cfg(feature = "SiIveco")]
///This icon requires the feature `SiIveco` to be enabled.
#[component]
pub fn Iveco(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M.24 14.402h1.433c.12 0 .239-.1.239-.243V9.87a.243.243 0 0 0-.244-.242H.24a.237.237 0 0 0-.24.24v4.294c0 .169.148.239.24.239m2.566-4.424l1.01 3.737c.064.25.244.435.391.53.17.11.338.156.61.156h1.119c.474 0 .833-.294.963-.794l.91-3.62a.431.431 0 0 0 .014-.108.238.238 0 0 0-.24-.25h-.827a.24.24 0 0 0-.235.182l-.718 3.135c-.022.108-.076.131-.134.131a.117.117 0 0 1-.112-.1l-.762-3.11c-.02-.157-.122-.239-.242-.239H3.03c-.188 0-.254.17-.243.273.001.022.013.043.019.077m7.728 1.49l-.002-.413c0-.15.116-.25.24-.25h1.7a.24.24 0 0 0 .238-.24l.001-.695a.243.243 0 0 0-.244-.242H9.57c-.468 0-.948.364-.948 1.024v2.79c0 .602.508.959.945.959h2.93c.124 0 .249-.093.243-.265v-.69a.24.24 0 0 0-.24-.24h-1.7c-.159 0-.266-.093-.266-.243v-.397h1.494a.242.242 0 0 0 .238-.251v-.61a.24.24 0 0 0-.244-.238h-1.488zm5.511-.664a11.9 11.9 0 0 1 1.283 0 .238.238 0 0 0 .239-.239v-.005l.005-.762a.239.239 0 0 0-.217-.238 12.611 12.611 0 0 0-.893-.024c-.42 0-.833.022-1.237.069l.025-.002c-.879.07-1.595.72-1.756 1.57a5.162 5.162 0 0 0-.074.82c0 .27.02.527.058.776a1.943 1.943 0 0 0 1.63 1.622l-.026-.004a9.788 9.788 0 0 0 2.303.053.239.239 0 0 0 .216-.236l-.001.017.002-.766a.239.239 0 0 0-.239-.239h-.007c-.239.012-.479.012-.722.007a12.965 12.965 0 0 1-.61-.021.675.675 0 0 1-.588-.594 6.287 6.287 0 0 1-.022-1.131.678.678 0 0 1 .631-.673M24 12.05c.003-.434-.06-.83-.178-1.195a1.916 1.916 0 0 0-1.59-1.285 9.213 9.213 0 0 0-1.085-.062c-.406 0-.76.01-1.156.06a1.932 1.932 0 0 0-1.6 1.392 3.792 3.792 0 0 0 .029 2.172 1.906 1.906 0 0 0 1.596 1.306c.342.037.684.054 1.037.054.443 0 .805 0 1.232-.058.85-.153 1.452-.752 1.613-1.511.065-.282.1-.57.102-.873m-1.91-.02c0 .222-.01.434-.037.65a.677.677 0 0 1-.62.565 8.625 8.625 0 0 1-.541.007.674.674 0 0 1-.664-.576 5.302 5.302 0 0 1-.014-1.337.677.677 0 0 1 .625-.601 8.615 8.615 0 0 1 .532-.004c.353 0 .643.268.675.612.03.223.044.452.044.684"
        /> < title > { title } < / title > < / svg >
    }
}
