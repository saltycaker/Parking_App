import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:parking_app/models/parking_location.dart';

final parkingProvider = StateNotifierProvider<ParkingNotifier, ParkingState>((ref) {
  return ParkingNotifier();
});

class ParkingState {
  final List<ParkingLocation> parkingLocations;
  final bool isLoading;
  final String? error;
  final ParkingLocation? selectedParking;

  ParkingState({
    this.parkingLocations = const [],
    this.isLoading = false,
    this.error,
    this.selectedParking,
  });

  ParkingState copyWith({
    List<ParkingLocation>? parkingLocations,
    bool? isLoading,
    String? error,
    ParkingLocation? selectedParking,
  }) {
    return ParkingState(
      parkingLocations: parkingLocations ?? this.parkingLocations,
      isLoading: isLoading ?? this.isLoading,
      error: error ?? this.error,
      selectedParking: selectedParking ?? this.selectedParking,
    );
  }
}

class ParkingNotifier extends StateNotifier<ParkingState> {
  ParkingNotifier() : super(ParkingState());

  Future<void> searchParking(double latitude, double longitude, {int radius = 500}) async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // This would call the API
      // For now, return mock data
      await Future.delayed(const Duration(seconds: 1));

      state = state.copyWith(
        parkingLocations: [],
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  void selectParking(ParkingLocation parking) {
    state = state.copyWith(selectedParking: parking);
  }

  void clearSelection() {
    state = state.copyWith(selectedParking: null);
  }
}
