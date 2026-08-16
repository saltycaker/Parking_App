use leptos::*;
use leptos_router::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <div class="relative min-h-screen bg-gray-100">
            <!-- Map Container -->
            <div class="absolute inset-0 z-0 bg-gray-100">
                <div class="w-full h-full flex items-center justify-center">
                    <div class="text-center">
                        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"/>
                        </svg>
                        <p class="text-gray-600">Navigation map using OpenStreetMap</p>
                        <p class="text-sm text-gray-500 mt-2">Route will be displayed here</p>
                    </div>
                </div>
            </div>

            <!-- ETA Card -->
            <div class="absolute top-4 left-4 right-4 z-10">
                <div class="bg-white rounded-2xl shadow-lg p-6">
                    <h1 class="text-xl font-bold text-gray-900 mb-2">Central Parking Garage</h1>
                    <div class="flex items-baseline gap-3">
                        <span class="text-4xl font-bold text-gray-900">3 min</span>
                        <span class="text-gray-600">(0.8 km)</span>
                    </div>
                    <p class="text-gray-500 mt-1">Arrival at 12:35 PM</p>
                </div>
            </div>

            <!-- Cancel Button -->
            <div class="absolute bottom-8 left-4 right-4 z-10">
                <A
                    href="/"
                    class="block w-full bg-white text-red-600 text-center py-4 rounded-xl font-semibold hover:bg-gray-50 transition border-2 border-red-600"
                >
                    "Cancel Navigation"
                </A>
            </div>
        </div>
    }
}
