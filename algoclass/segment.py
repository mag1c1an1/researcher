import numpy as np
import matplotlib.pyplot as plt

rng = np.random.default_rng()
points1 = rng.random((2, 2))
points2 = rng.random((2, 2))
plt.axis('off')
plt.plot(points1[:, 0], points1[:, 1], 'o-')
plt.plot(points2[:, 0], points2[:, 1], 'o-')
plt.show()


def direction(pi, pj, pk):
    x1, y1 = pk[0] - pi[0], pk[1] - pi[1]
    x2, y2 = pj[0] - pi[0], pj[1] - pi[1]
    return x1 * y2 - x2 * y1


def on_segment(pi, pj, pk):
    if min(pi[0], pj[0]) <= pk[0] <= max(pi[0], pj[0]) and min(pi[1], pj[1]) <= pk[1] <= max(pi[1], pj[1]):
        return True
    return False


def segment_intersect(p1, p2, p3, p4):
    d1 = direction(p3, p4, p1)
    d2 = direction(p3, p4, p2)
    d3 = direction(p1, p2, p3)
    d4 = direction(p1, p2, p4)
    if d1 * d2 < 0 and d3 * d4 < 0:
        return True
    elif d1 == 0 and on_segment(p3, p4, p1):
        return True
    elif d2 == 0 and on_segment(p3, p4, p2):
        return True
    elif d3 == 0 and on_segment(p1, p2, p3):
        return True
    elif d4 == 0 and on_segment(p4, p2, p4):
        return True
    else:
        return False


print(segment_intersect(points1[0], points1[1], points2[0], points2[1]))
