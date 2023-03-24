#[cfg(feature = "SiFujifilm")]
use leptos::*;
#[cfg(feature = "SiFujifilm")]
///This icon requires the feature `SiFujifilm` to be enabled.
#[component]
pub fn Fujifilm(
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
        "M.484 10.003c-.483 0-.484.238-.484.533v3.329h.786c.165.003.24-.057.243-.207v-1.556h1.444c.034 0 .055-.018.055-.052v-.39H1.03v-.98c0-.173.086-.255.258-.255H2.56c.037 0 .049-.024.049-.05v-.372zm7.997 0c-.111 0-.197.052-.197.21v2.654c0 .377-.143.632-.573.693-.29.041-.678.03-.847.024v.324c0 .03.014.052.049.06.025.006.208.029.62.029.722 0 1.054-.156 1.245-.292.365-.263.495-.575.495-1.192v-2.51zm-5.13 0c-.148 0-.194.07-.194.199v2.658c0 .538.26.866.814 1.033.463.14 1.21.137 1.701.003.37-.1.787-.373.787-1.037v-2.856h-.77c-.172 0-.219.073-.219.196v2.748c0 .294-.063.443-.229.536-.242.135-.645.133-.883-.004-.177-.103-.213-.28-.213-.533v-2.943zm9.308 0c-.483 0-.483.239-.483.533v.995c0 .02-.01.029-.018.038l-.677.533h.695v1.763h.786c.164.003.24-.057.242-.207v-1.556h1.444c.034 0 .055-.018.055-.052v-.39h-1.499v-.98c0-.173.087-.255.258-.255h1.274c.036 0 .05-.024.05-.05v-.372zm3.701 0h-.768c-.175 0-.222.082-.222.203v3.66h.783c.138 0 .207-.042.207-.204zm1.922 0h-.76c-.173-.003-.229.066-.229.213v3.108c0 .258.076.541.591.541h1.668c.044 0 .065-.027.065-.066v-.351h-1.03c-.205-.01-.305-.115-.305-.318zm5.718 0h-1.144c-.195 0-.306.092-.35.266l-.63 2.663h-.018l-.65-2.928h-.89c-.172 0-.226.088-.226.224v3.637h.376c.06 0 .086-.03.086-.092v-3.049h.059l.804 3.141h.527c.199 0 .273-.081.322-.278l.722-2.863h.058v3.141h.703c.172 0 .251-.067.251-.237zm-13.608.001c-.173 0-.22.084-.22.205v1.87l.93-.73c.036-.028.056-.05.058-.11v-1.235zm-.22 2.098v1.763h.781c.138 0 .208-.04.208-.203v-1.56z"
        /> < title > { title } < / title > < / svg >
    }
}
