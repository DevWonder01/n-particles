# Tangential Orbital Velocity

In physics, for an object to maintain a stable circular orbit around a central mass, the **Gravitational Force** pulling it inward must be exactly balanced by the **Centripetal Force** required to keep it moving in a circle.

## The Derivation

### 1. Gravitational Force ($F_g$)
According to Newton's law of universal gravitation:
$$ F_g = \frac{G \cdot M \cdot m}{r^2} $$
Where:
- $G$ is the gravitational constant.
- $M$ is the mass of the central body (the Sun).
- $m$ is the mass of the orbiting body (the Planet).
- $r$ is the distance between the two.

### 2. Centripetal Force ($F_c$)
The force required to keep an object of mass $m$ moving at velocity $v$ in a circle of radius $r$:
$$ F_c = \frac{m \cdot v^2}{r} $$

### 3. Setting them Equal
For a stable orbit, $F_g = F_c$:
$$ \frac{G \cdot M \cdot m}{r^2} = \frac{m \cdot v^2}{r} $$

### 4. Solving for Velocity ($v$)
Cancel $m$ from both sides and multiply by $r$:
$$ \frac{G \cdot M}{r} = v^2 $$

Take the square root of both sides:
$$ v = \sqrt{\frac{G \cdot M}{r}} $$

---

## Implementation in the Code

To apply this velocity in a 2D simulation at a specific **angle** ($\theta$), we must convert this speed ($v$) into $x$ and $y$ components. Since the velocity must be **perpendicular** to the radius vector:

$$ v_x = -\sin(\theta) \cdot v $$
$$ v_y = \cos(\theta) \cdot v $$

This ensures the particle starts its journey moving "sideways" relative to the central mass!
