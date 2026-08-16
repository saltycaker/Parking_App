import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:parking_app/utils/theme.dart';
import 'package:parking_app/widgets/bottom_sheet.dart';
import 'package:parking_app/widgets/parking_card.dart';

class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({super.key});

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  final MapController _mapController = MapController();
  bool _showBottomSheet = true;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          // Map
          FlutterMap(
            mapController: _mapController,
            options: MapOptions(
              initialCenter: LatLng(37.7749, -122.4194), // San Francisco
              initialZoom: 14.0,
              interactionOptions: const InteractionOptions(
                flags: InteractiveFlag.all & ~InteractiveFlag.rotate,
              ),
            ),
            children: [
              TileLayer(
                urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
                userAgentPackageName: 'com.example.parking_app',
              ),
              MarkerLayer(
                markers: [
                  // Current location marker
                  Marker(
                    point: LatLng(37.7749, -122.4194),
                    width: 40,
                    height: 40,
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.black,
                        borderRadius: BorderRadius.circular(20),
                        border: Border.all(color: Colors.white, width: 4),
                        boxShadow: [
                          BoxShadow(
                            color: Colors.black.withOpacity(0.2),
                            blurRadius: 8,
                            offset: const Offset(0, 4),
                          ),
                        ],
                      ),
                      child: const Icon(
                        Icons.my_location,
                        color: Colors.white,
                        size: 20,
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),

          // Search Bar - Uber style
          Positioned(
            top: MediaQuery.of(context).padding.top + 16,
            left: 16,
            right: 16,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(8),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.08),
                    blurRadius: 16,
                    offset: const Offset(0, 4),
                  ),
                ],
              ),
              child: TextField(
                decoration: InputDecoration(
                  hintText: 'Where to?',
                  hintStyle: TextStyle(
                    color: AppTheme.tertiaryTextColor,
                    fontSize: 18,
                    fontWeight: FontWeight.w500,
                  ),
                  prefixIcon: const Icon(Icons.search, color: Colors.black),
                  suffixIcon: IconButton(
                    icon: const Icon(Icons.tune, color: Colors.black),
                    onPressed: () {
                      // Show filters
                    },
                  ),
                  border: InputBorder.none,
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 18,
                  ),
                ),
              ),
            ),
          ),

          // Current Location Button - Uber style
          Positioned(
            right: 16,
            bottom: _showBottomSheet ? 320 : 100,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(8),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.08),
                    blurRadius: 16,
                    offset: const Offset(0, 4),
                  ),
                ],
              ),
              child: IconButton(
                icon: const Icon(Icons.my_location, color: Colors.black),
                onPressed: () {
                  // Center on current location
                },
              ),
            ),
          ),

          // Profile Button - Uber style
          Positioned(
            right: 16,
            top: MediaQuery.of(context).padding.top + 80,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(8),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.08),
                    blurRadius: 16,
                    offset: const Offset(0, 4),
                  ),
                ],
              ),
              child: IconButton(
                icon: const Icon(Icons.person, color: Colors.black),
                onPressed: () {
                  context.push('/profile');
                },
              ),
            ),
          ),

          // Car Detection Button - Uber style
          Positioned(
            left: 16,
            top: MediaQuery.of(context).padding.top + 80,
            child: Container(
              decoration: BoxDecoration(
                color: Colors.black,
                borderRadius: BorderRadius.circular(8),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.2),
                    blurRadius: 16,
                    offset: const Offset(0, 4),
                  ),
                ],
              ),
              child: IconButton(
                icon: const Icon(Icons.camera_alt, color: Colors.white),
                onPressed: () {
                  context.push('/car-detection');
                },
              ),
            ),
          ),

          // Bottom Sheet
          if (_showBottomSheet)
            const ParkingBottomSheet(),
        ],
      ),
    );
  }
}
