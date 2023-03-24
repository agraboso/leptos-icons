#[cfg(feature = "SiStandardjs")]
use leptos::*;
#[cfg(feature = "SiStandardjs")]
///This icon requires the feature `SiStandardjs` to be enabled.
#[component]
pub fn Standardjs(
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
        "M11.966 0 1.608 5.965v12.07L12.035 24l10.358-5.965V5.965Zm1.421 7.02h.651v3.483h-.651l.015-.3c-.116.218-.333.343-.647.343-.501 0-.93-.443-.93-1.075 0-.632.424-1.066.92-1.066.329 0 .512.13.642.333zm6.83 0h.65v3.483h-.65l.014-.3c-.116.218-.333.343-.646.343-.502 0-.931-.443-.931-1.075 0-.632.424-1.066.921-1.066.328 0 .511.13.641.333zm-14.928.358h.646v1.07h.526v.54h-.526v.763c0 .207.068.25.17.25.105 0 .183-.058.226-.086l.246.453a.865.865 0 0 1-.574.188c-.415 0-.714-.207-.714-.825v-.743h-.304v-.54h.304zM3.895 8.405c.338 0 .58.11.748.284l-.347.372a.634.634 0 0 0-.396-.154c-.15 0-.222.038-.222.12 0 .116.13.13.41.193.3.068.676.207.676.641 0 .377-.198.685-.845.685-.477 0-.757-.178-.935-.39l.38-.338a.74.74 0 0 0 .512.217c.212 0 .28-.039.28-.125 0-.111-.13-.111-.463-.198-.28-.072-.593-.227-.593-.656 0-.463.356-.651.795-.651zm4.018 0c.526 0 .926.284.926.93v1.168h-.646l.014-.285c-.154.237-.333.328-.608.328-.376 0-.723-.169-.723-.646 0-.439.333-.617.84-.617h.477c-.01-.237-.126-.343-.343-.343a.643.643 0 0 0-.468.188l-.361-.385c.193-.189.41-.338.892-.338zm2.71 0c.449 0 .772.328.772.911v1.187h-.651V9.36c0-.232-.11-.362-.342-.362-.26 0-.415.145-.415.405v1.1h-.651V8.448h.65l-.014.338a.703.703 0 0 1 .651-.381zm4.857 0c.526 0 .926.284.926.93v1.168h-.646l.014-.285c-.154.237-.332.328-.607.328-.376 0-.724-.169-.724-.646 0-.439.333-.617.84-.617h.477c-.01-.237-.125-.343-.343-.343a.643.643 0 0 0-.467.188l-.362-.385c.193-.189.41-.338.892-.338zm2.667 0c.135 0 .237.038.319.082l-.208.593a.567.567 0 0 0-.25-.063c-.295 0-.454.135-.454.574v.912h-.651V8.448h.651l-.01.372c.15-.338.391-.415.603-.415zm-5.204.574c-.275 0-.448.188-.448.492 0 .323.173.506.448.506.28 0 .463-.183.463-.506 0-.304-.183-.492-.463-.492zm6.83 0c-.275 0-.449.188-.449.492 0 .323.174.506.449.506.28 0 .463-.183.463-.506 0-.304-.184-.492-.463-.492zm-12.024.69c-.164 0-.236.072-.236.188s.096.183.28.183c.265 0 .4-.116.4-.29v-.082zm7.567 0c-.164 0-.236.072-.236.188s.096.183.28.183c.265 0 .4-.116.4-.29v-.082zm-1.261 2.478c.766 0 1.317.267 1.713.965l-.938.603c-.207-.37-.431-.517-.775-.517-.354 0-.578.224-.578.517 0 .361.224.508.741.732l.302.129c1.024.439 1.602.887 1.602 1.895 0 1.085-.853 1.679-1.999 1.679-1.12 0-1.843-.534-2.196-1.232l.982-.568c.258.422.594.732 1.189.732.5 0 .818-.25.818-.594 0-.414-.328-.56-.879-.801l-.301-.13c-.87-.37-1.447-.835-1.447-1.817 0-.904.689-1.593 1.766-1.593zm-3.988.069h1.206v4.117c0 1.249-.732 1.817-1.8 1.817-.965 0-1.525-.5-1.809-1.102l.982-.595c.19.336.362.62.775.62.396 0 .646-.155.646-.757z"
        /> < title > { title } < / title > < / svg >
    }
}
