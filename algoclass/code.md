```python
import numpy as np
import matplotlib.pyplot as plt  # type: ignore
from typing import List


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
        self.vis = False

    def __str__(self):
        return "({},{},{})".format(self.x, self.y, self.vis)


# line p1,p2
# point is  p3
def area(p1: Point, p2: Point, p3: Point) -> int:
    val: int = p1.x * p2.y + p3.x * p1.y + p2.x * p3.y - p3.x * p2.y - p2.x * p1.y - p1.x * p3.y
    return val


def dc(points: List[Point], ver: int):
    if len(points) <= 2:
        for p in points:
            p.vis = True
        return
    pi = points[0]
    pj = points[-1]
    mx = -float('inf')
    mn = float('inf')
    imax = -1
    imin = -1
    for a in range(1, len(points) - 1):
        s = area(pi, pj, points[a])
        if s > mx and s >= 0:
            mx = s
            imax = a
        if s < mn and s <= 0:
            mn = s
            imin = a
    s1, s2, s3, s4 = [], [], [], []
    for a in range(0, len(points) - 1):
        if area(pi, points[imax], points[a]) >= 0:
            s1.append(points[a])
        if area(pi, points[imin], points[a]) <= 0:
            s3.append(points[a])

    for a in range(1, len(points)):
        if area(points[imax], pj, points[a]) >= 0:
            s2.append(points[a])
        if area(points[imin], pj, points[a]) <= 0:
            s4.append(points[a])
    match ver:
        case 1:
            dc(s1, 1)
            dc(s2, 1)
        case 2:
            dc(s3, 2)
            dc(s4, 2)
        case 3:
            dc(s1, 1)
            dc(s2, 1)
            dc(s3, 2)
            dc(s4, 2)


if __name__ == '__main__':
    origins = np.random.randint(0, 100, (30, 2))
    plt.plot(origins[:, 0], origins[:, 1], 'o')
    ps: List[Point] = []
    for v in origins:
        ps.append(Point(v[0], v[1]))
    ps.sort(key=lambda ele: ele.x)
    dc(ps, 3)
    list1 = [ps[0]]
    list2 = [ps[0]]
    for p in ps:
        if not p.vis:
            continue
        s = area(ps[0], ps[-1], p)
        if s > 0:
            list1.append(p)
        elif s < 0:
            list2.append(p)

    list1.append(ps[-1])
    list2.append(ps[-1])
    for i in range(len(list1) - 1):
        plt.plot([list1[i].x, list1[i + 1].x], [list1[i].y, list1[i + 1].y], 'k-')
    for i in range(len(list2) - 1):
        plt.plot([list2[i].x, list2[i + 1].x], [list2[i].y, list2[i + 1].y], 'k-')
    plt.show()
```
