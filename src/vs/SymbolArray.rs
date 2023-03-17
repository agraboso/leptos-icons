#[cfg(feature = "VsSymbolArray")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolArray")]
/// *This icon requires the feature* `VsSymbolArray` *to be enabled*.
#[component]
pub fn SymbolArray(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 2l-.5.5v11l.5.5H4v-1H2V3h2V2H1.5zm13 12l.5-.5v-11l-.5-.5H12v1h2v10h-2v1h2.5z" /></svg>
   }
}