#[cfg(feature = "TbCircuitResistor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircuitResistor")]
/// *This icon requires the feature* `TbCircuitResistor` *to be enabled*.
#[component]
pub fn CircuitResistor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circuit-resistor" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M2 12h2l2 -5l3 10l3 -10l3 10l3 -10l1.5 5h2.5" /></svg>
   }
}