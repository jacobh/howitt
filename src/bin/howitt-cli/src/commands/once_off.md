# Ride Segment Analysis Tool

This tool analyzes a ride by splitting it into segments of at least 250 meters each and calculating various metrics for each segment.

## Overview

The Ride Segment Analysis Tool provides detailed insights into ride segments, including:

- Elapsed time for each segment
- Distance traveled within each segment
- Elevation gained and lost
- Euclidean coordinates (x,y) of each segment endpoint relative to the segment's starting point

This allows for detailed analysis of ride performance across different terrain types and ride segments.

## Usage

```bash
howitt-cli once-off <ride_id>
```

Where `<ride_id>` is the UUID of the ride you want to analyze.

## Output

The tool outputs a JSON structure containing:

- Ride information (ID, name)
- Total number of segments
- Total distance of the ride
- Detailed metrics for each segment

Example output:

```json
{
  "ride_id": "RIDE#abcdef1234567890",
  "ride_name": "Morning Mountain Loop",
  "total_segments": 12,
  "total_distance_m": 3204.5,
  "segments": [
    {
      "segment_index": 0,
      "start_datetime": "2023-07-15T08:30:00Z",
      "end_datetime": "2023-07-15T08:35:45Z",
      "elapsed_time_secs": 345,
      "distance_m": 250.3,
      "elevation_gain_m": 15.6,
      "elevation_loss_m": 2.3,
      "x_offset_m": 240.5,
      "y_offset_m": 70.2
    },
    ...
  ]
}
```

## Metrics Explanation

- **segment_index**: Zero-based index of the segment
- **start_datetime/end_datetime**: Timestamps for the segment start and end points
- **elapsed_time_secs**: Time spent traveling through the segment (in seconds)
- **distance_m**: Distance traveled within the segment (in meters)
- **elevation_gain_m**: Total elevation gained within the segment (in meters)
- **elevation_loss_m**: Total elevation lost within the segment (in meters)
- **x_offset_m/y_offset_m**: Euclidean coordinates of the segment endpoint relative to the segment's start point (in meters)

## How It Works

1. The tool retrieves the ride and its GPS points from the database
2. It splits the ride into segments where each segment is at least 250 meters in straight-line distance
3. For each segment, it calculates metrics using all points within the segment
4. The first point of each segment is converted to Euclidean coordinate (0,0)
5. The end point of each segment is expressed relative to its segment's starting point, not the entire ride's start

## Notes

- Segments are created based on straight-line distance between start and end points
- No interpolation is performed between existing GPS points
- If the ride has fewer than 2 points, no segments will be created
- The last segment may be shorter than 250m if there aren't enough remaining points

## Use Cases

- Analyzing performance on different terrain types
- Comparing uphill vs. downhill segments
- Visualizing ride data in a Cartesian coordinate system
- Identifying segments with unusual elevation profiles
