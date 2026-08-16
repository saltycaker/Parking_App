import 'package:flutter/material.dart';
import 'package:camera/camera.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:parking_app/utils/theme.dart';

class CarDetectionScreen extends StatefulWidget {
  const CarDetectionScreen({super.key});

  @override
  State<CarDetectionScreen> createState() => _CarDetectionScreenState();
}

class _CarDetectionScreenState extends State<CarDetectionScreen> {
  CameraController? _cameraController;
  bool _isInitialized = false;
  String _statusMessage = 'Initializing camera...';
  List<CarDetection> _detectedCars = [];

  @override
  void initState() {
    super.initState();
    _initializeCamera();
  }

  Future<void> _initializeCamera() async {
    final status = await Permission.camera.request();
    if (!status.isGranted) {
      setState(() {
        _statusMessage = 'Camera permission denied';
      });
      return;
    }

    final cameras = await availableCameras();
    if (cameras.isEmpty) {
      setState(() {
        _statusMessage = 'No cameras available';
      });
      return;
    }

    final backCamera = cameras.firstWhere(
      (camera) => camera.lensDirection == CameraLensDirection.back,
      orElse: () => cameras.first,
    );

    _cameraController = CameraController(
      backCamera,
      ResolutionPreset.high,
      enableAudio: false,
    );

    try {
      await _cameraController!.initialize();
      setState(() {
        _isInitialized = true;
        _statusMessage = 'Camera ready - Place phone on dashboard';
      });
      
      // Simulate car detection for demo
      _simulateDetection();
    } catch (e) {
      setState(() {
        _statusMessage = 'Error: ${e.toString()}';
      });
    }
  }

  void _simulateDetection() {
    // Simulate detected cars with distances
    Future.delayed(const Duration(seconds: 2), () {
      if (mounted) {
        setState(() {
          _detectedCars = [
            CarDetection(
              position: const Offset(0.3, 0.4),
              distance: 2.5,
              confidence: 0.85,
            ),
            CarDetection(
              position: const Offset(0.7, 0.5),
              distance: 4.2,
              confidence: 0.72,
            ),
          ];
        });
      }
    });
  }

  @override
  void dispose() {
    _cameraController?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.black,
        title: const Text('Car Detection', style: TextStyle(color: Colors.white)),
        iconTheme: const IconThemeData(color: Colors.white),
      ),
      body: Stack(
        children: [
          if (_isInitialized && _cameraController != null)
            Center(child: CameraPreview(_cameraController!))
          else
            Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const CircularProgressIndicator(color: Colors.white),
                  const SizedBox(height: 16),
                  Text(_statusMessage, style: const TextStyle(color: Colors.white)),
                ],
              ),
            ),
          
          // Distance overlay lines
          if (_isInitialized)
            Positioned.fill(
              child: CustomPaint(
                painter: DistanceOverlayPainter(),
              ),
            ),
          
          // Car detection boxes
          if (_isInitialized)
            Positioned.fill(
              child: CustomPaint(
                painter: CarDetectionPainter(detectedCars: _detectedCars),
              ),
            ),
          
          Positioned(
            top: 16,
            left: 16,
            right: 16,
            child: Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.7),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Dash Camera Mode',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    'Detected cars: ${_detectedCars.length}',
                    style: const TextStyle(color: Colors.white70, fontSize: 14),
                  ),
                ],
              ),
            ),
          ),
          Positioned(
            bottom: 32,
            left: 16,
            right: 16,
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.8),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Column(
                children: [
                  const Text(
                    'Distance Guide',
                    style: TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceAround,
                    children: [
                      _buildDistanceIndicator('Safe', '>3m', Colors.green),
                      _buildDistanceIndicator('Caution', '1-3m', Colors.yellow),
                      _buildDistanceIndicator('Danger', '<1m', Colors.red),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildDistanceIndicator(String label, String distance, Color color) {
    return Column(
      children: [
        Container(
          width: 40,
          height: 40,
          decoration: BoxDecoration(
            color: color,
            borderRadius: BorderRadius.circular(8),
          ),
        ),
        const SizedBox(height: 8),
        Text(
          label,
          style: const TextStyle(color: Colors.white, fontSize: 12, fontWeight: FontWeight.w500),
        ),
        Text(
          distance,
          style: const TextStyle(color: Colors.white70, fontSize: 10),
        ),
      ],
    );
  }
}

class CarDetection {
  final Offset position;
  final double distance;
  final double confidence;

  CarDetection({
    required this.position,
    required this.distance,
    required this.confidence,
  });
}

class DistanceOverlayPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 3;

    // Green line (safe distance > 3m)
    paint.color = Colors.green.withOpacity(0.6);
    canvas.drawLine(
      Offset(0, size.height * 0.3),
      Offset(size.width, size.height * 0.3),
      paint,
    );

    // Yellow line (caution 1-3m)
    paint.color = Colors.yellow.withOpacity(0.6);
    canvas.drawLine(
      Offset(0, size.height * 0.5),
      Offset(size.width, size.height * 0.5),
      paint,
    );

    // Red line (danger < 1m)
    paint.color = Colors.red.withOpacity(0.6);
    canvas.drawLine(
      Offset(0, size.height * 0.7),
      Offset(size.width, size.height * 0.7),
      paint,
    );
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}

class CarDetectionPainter extends CustomPainter {
  final List<CarDetection> detectedCars;

  CarDetectionPainter({required this.detectedCars});

  @override
  void paint(Canvas canvas, Size size) {
    for (final car in detectedCars) {
      final x = car.position.dx * size.width;
      final y = car.position.dy * size.height;
      
      // Draw bounding box
      final boxPaint = Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 3
        ..color = _getDistanceColor(car.distance);
      
      final boxSize = 80.0;
      canvas.drawRect(
        Rect.fromCenter(center: Offset(x, y), width: boxSize, height: boxSize * 0.6),
        boxPaint,
      );

      // Draw distance label
      final textPainter = TextPainter(
        text: TextSpan(
          text: '${car.distance.toStringAsFixed(1)}m',
          style: TextStyle(
            color: _getDistanceColor(car.distance),
            fontSize: 16,
            fontWeight: FontWeight.bold,
          ),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(x - textPainter.width / 2, y - boxSize / 2 - 20));
    }
  }

  Color _getDistanceColor(double distance) {
    if (distance >= 3.0) return Colors.green;
    if (distance >= 1.0) return Colors.yellow;
    return Colors.red;
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}
