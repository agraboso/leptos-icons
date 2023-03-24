#[cfg(feature = "BiLogosCreativeCommons")]
use leptos::*;
#[cfg(feature = "BiLogosCreativeCommons")]
///This icon requires the feature `BiLogosCreativeCommons` to be enabled.
#[component]
pub fn CreativeCommons(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.354 6.479a10.021 10.021 0 0 0-7.421-4.429c-3.108-.294-6.031.771-8.123 2.963C3.533 6.35 2.699 7.839 2.21 9.66c-.217.805-.247 1.104-.244 2.396.002 1.293.034 1.599.255 2.432a10.232 10.232 0 0 0 7.451 7.332c.315.078.702.16.859.182.696.097 2.381.056 3.131-.075 3.088-.538 5.832-2.531 7.24-5.258 1.644-3.181 1.426-7.222-.548-10.19zm-.41 7.688c-.808 2.99-3.263 5.272-6.361 5.912-4.831.997-9.538-2.658-9.839-7.641-.194-3.217 1.755-6.446 4.745-7.863 1.133-.536 2.045-.733 3.425-.738 1.327-.004 2.064.132 3.223.596 2.324.931 4.146 3.04 4.816 5.573.281 1.06.276 3.103-.009 4.161z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.042 14.955c-.915-.324-1.616-1.275-1.74-2.36-.183-1.607.422-2.856 1.654-3.415.669-.303 1.882-.296 2.603.016.438.19 1.261.933 1.261 1.139 0 .033-.284.201-.631.372l-.632.312-.337-.337c-.187-.188-.475-.363-.649-.396-.433-.082-.952.111-1.187.44-.389.546-.415 1.972-.048 2.533.191.291.512.494.813.518.635.05.796-.006 1.172-.401l.379-.398.488.269c.269.148.527.305.575.347.164.148-.592.92-1.199 1.224-.517.259-.679.293-1.358.286-.425-.006-.949-.074-1.164-.149zm5.816 0c-.901-.32-1.591-1.241-1.739-2.325-.215-1.569.419-2.888 1.654-3.45.717-.324 1.934-.3 2.661.056.45.221 1.201.911 1.201 1.104 0 .034-.295.203-.654.377l-.654.317-.341-.37c-.304-.332-.385-.369-.802-.369-.576 0-.945.225-1.145.698-.18.423-.201 1.461-.043 1.934.324.961 1.505 1.188 2.175.419l.304-.346.58.294c.32.161.582.319.582.352 0 .219-.75.918-1.256 1.17-.517.259-.679.293-1.358.287-.425-.005-.949-.073-1.165-.148z"
        /> < title > { title } < / title > < / svg >
    }
}
