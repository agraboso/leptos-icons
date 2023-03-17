#[cfg(feature = "IoPushSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPushSharp")]
/// *This icon requires the feature* `IoPushSharp` *to be enabled*.
#[component]
pub fn PushSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M272,352V204.63l64,64L358.63,246,256,143.37,153.37,246,176,268.63l64-64V352H92a12,12,0,0,1-12-12V44A12,12,0,0,1,92,32H420a12,12,0,0,1,12,12V340a12,12,0,0,1-12,12Z" /><rect x="240" y="352" width="32" height="128" /></svg>
   }
}