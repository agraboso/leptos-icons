#[cfg(feature = "BiLogosYelp")]
use leptos::*;
#[cfg(feature = "BiLogosYelp")]
///This icon requires the feature `BiLogosYelp` to be enabled.
#[component]
pub fn Yelp(
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
        "M18.824 16.675c-.105.726-1.587 2.609-2.269 2.882-.232.093-.457.07-.637-.068-.116-.09-.235-.272-1.833-2.866l-.475-.773c-.183-.277-.148-.642.078-.92.223-.27.549-.37.832-.262.016.008 1.195.397 1.195.397 2.688.883 2.775.917 2.891 1.002.17.139.249.356.214.608h.004zm-5.386-3.946c-.19-.289-.187-.63.01-.865l.746-1.019c1.642-2.235 1.732-2.353 1.843-2.431a.636.636 0 0 1 .652-.024c.651.316 1.966 2.269 2.045 3.019v.025a.623.623 0 0 1-.259.587c-.123.079-.235.125-3.291.866-.483.123-.752.19-.91.247l.021-.023c-.302.091-.64-.055-.831-.357l-.026-.025zm-1.875-1.159c-.146.046-.591.185-1.139-.702 0 0-3.694-5.813-3.78-5.992-.053-.201.011-.429.19-.613.55-.569 3.534-1.405 4.315-1.215.255.066.43.227.491.453.045.25.408 5.646.46 6.854.051 1.034-.392 1.171-.537 1.215zm.487 5.913c-.008 2.827-.015 2.922-.061 3.056-.079.211-.26.352-.51.398-.72.122-2.972-.71-3.441-1.267a.739.739 0 0 1-.157-.37c-.013-.09 0-.18.033-.259.057-.146.135-.259 2.158-2.63l.594-.706c.203-.26.563-.338.899-.204.325.124.528.405.506.708v1.259l-.021.015zm-6.13-1.808c-.222-.006-.42-.14-.535-.358-.083-.162-.142-.428-.18-.752-.102-.974.023-2.444.314-2.911.138-.214.338-.327.559-.319.146 0 .276.046 3.167 1.236l.848.337c.302.111.489.427.472.787-.022.348-.224.616-.521.696l-1.202.382c-2.689.864-2.778.888-2.919.877l-.003.025zm11.22 5.322h-.004l-.003.003.007-.003z"
        /> < title > { title } < / title > < / svg >
    }
}
