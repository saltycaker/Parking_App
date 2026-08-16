use leptos::*;
use leptos_router::*;
use crate::types::*;
use crate::services::api;

#[component]
pub fn ParkingDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();
    
    let (loading, set_loading) = create_signal(true);
    let (parking, set_parking) = create_signal(None::<ParkingLocation>);

    create_effect(move |_| {
        let parking_id = id();
        if !parking_id.is_empty() {
            set_loading.set(true);
            
            spawn_local(async move {
                match uuid::Uuid::parse_str(&parking_id) {
                    Ok(uuid) => {
                        match api::get_parking(uuid).await {
                            Ok(location) => {
                                set_parking.set(Some(location));
                                set_loading.set(false);
                            }
                            Err(e) => {
                                log::error!("Failed to load parking: {}", e);
                                set_loading.set(false);
                            }
                        }
                    }
                    Err(_) => {
                        set_loading.set(false);
                    }
                }
            });
        }
    });

    view! {
        <div class="min-h-screen bg-white">
            if loading.get() {
                <div class="flex items-center justify-center h-screen">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
                </div>
            } else if let Some(location) = parking.get() {
                <div class="max-w-2xl mx-auto p-4">
                    // Header
                    <div class="mb-6">
                        <A href="/" class="inline-flex items-center text-gray-600 hover:text-gray-900 mb-4">
                            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                            </svg>
                            "Back"
                        </A>
                        <h1 class="text-3xl font-bold text-gray-900">{location.name.clone()}</h1>
                        <p class="text-gray-600 mt-1">{location.address.clone()}</p>
                    </div>

                    // Availability Card
                    <div class="bg-green-50 rounded-2xl p-6 mb-6">
                        <h2 class="text-sm text-gray-600 mb-2">Estimated Availability</h2>
                        <div class="flex items-baseline gap-3">
                            <span class="text-4xl font-bold text-green-600">82%</span>
                            <span class="text-lg font-semibold text-green-600">High Chance</span>
                        </div>
                        <p class="text-sm text-gray-500 mt-2">Based on historical data and recent reports</p>
                    </div>

                    // Details
                    <div class="space-y-4 mb-6">
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                            </svg>
                            <span class="text-gray-900">180 m away</span>
                        </div>
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                            <span class="text-gray-900">3 min drive</span>
                        </div>
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                            </svg>
                            <span class="text-gray-900">{location.rating.unwrap_or(0.0)} ★ ({location.review_count.unwrap_or(0)} reviews)</span>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="space-y-3">
                        <A
                            href=format!("/navigation/{}", id())
                            class="block w-full bg-purple-600 text-white text-center py-4 rounded-xl font-semibold hover:bg-purple-700 transition"
                        >
                            "Navigate"
                        </A>
                        <div class="flex gap-3">
                            <button class="flex-1 border-2 border-purple-600 text-purple-600 py-3 rounded-xl font-semibold hover:bg-purple-50 transition">
                                "Save"
                            </button>
                            <button class="flex-1 border-2 border-gray-300 text-gray-700 py-3 rounded-xl font-semibold hover:bg-gray-50 transition">
                                "Report"
                            </button>
                        </div>
                    </div>
                </div>
            } else {
                <div class="flex items-center justify-center h-screen">
                    <p class="text-gray-600">Parking location not found</p>
                </div>
            }
        </div>
    }
}
