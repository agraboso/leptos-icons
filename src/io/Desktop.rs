#[cfg(feature = "IoDesktop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDesktop")]
/// *This icon requires the feature* `IoDesktop` *to be enabled*.
#[component]
pub fn Desktop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M16,352a48.05,48.05,0,0,0,48,48H197.88l-4,32H144a16,16,0,0,0,0,32H368a16,16,0,0,0,0-32H318.12l-4-32H448a48.05,48.05,0,0,0,48-48V304H16Zm240-16a16,16,0,1,1-16,16A16,16,0,0,1,256,336Z" /><path d="M496,96a48.05,48.05,0,0,0-48-48H64A48.05,48.05,0,0,0,16,96V288H496Z" /></svg>
   }
}