import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:parking_web_flutter/screens/home_screen.dart';
import 'package:parking_web_flutter/screens/parking_detail_screen.dart';
import 'package:parking_web_flutter/screens/navigation_screen.dart';
import 'package:parking_web_flutter/screens/profile_screen.dart';
import 'package:parking_web_flutter/utils/theme.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  
  runApp(
    ProviderScope(
      child: const ParkingWebApp(),
    ),
  );
}

class ParkingWebApp extends ConsumerWidget {
  const ParkingWebApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);

    return MaterialApp.router(
      title: 'Parking Discovery',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.lightTheme,
      routerConfig: router,
    );
  }
}

// Router configuration
final routerProvider = Provider<GoRouter>((ref) {
  return GoRouter(
    initialLocation: '/',
    routes: [
      GoRoute(
        path: '/',
        builder: (context, state) => const HomeScreen(),
      ),
      GoRoute(
        path: '/parking/:id',
        builder: (context, state) {
          final id = state.pathParameters['id']!;
          return ParkingDetailScreen(parkingId: id);
        },
      ),
      GoRoute(
        path: '/navigation/:id',
        builder: (context, state) {
          final id = state.pathParameters['id']!;
          return NavigationScreen(parkingId: id);
        },
      ),
      GoRoute(
        path: '/profile',
        builder: (context, state) => const ProfileScreen(),
      ),
    ],
  );
});
