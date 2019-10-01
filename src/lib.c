#include <stdint.h>

typedef struct {
  uint32_t line;
  uint32_t column;
} Point;

Point add_points_in_c(Point a, Point b) {
  if (b.line > 0) {
    return (Point) {a.line + b.line, b.column};
  } else {
    return (Point) {a.line, a.column + b.column};
  }
}
