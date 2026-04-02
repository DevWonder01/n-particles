# Collision Physics Mathematics

This document details the step-by-step mathematical derivation used in the `resolve_collision` implementation.

## 1. Geometric Context
Two circular bodies $B_1$ and $B_2$ with positions $\vec{P}_1, \vec{P}_2$, velocities $\vec{v}_1, \vec{v}_2$, and masses $m_1, m_2$.
The radius $R$ is derived from mass $m$ assuming uniform density and area $A = \pi R^2$:
$$ R = \sqrt{\frac{m}{\pi}} $$

## 2. Detection & Normal Vector
The separation vector $\vec{d}$ and distance $dist$ are:
$$ \vec{d} = \vec{P}_2 - \vec{P}_1 $$
$$ dist = \| \vec{d} \| = \sqrt{dx^2 + dy^2} $$

The **Collision Normal** $\vec{n}$ is the unit vector along $\vec{d}$:
$$ \vec{n} = \frac{\vec{d}}{dist} $$

## 3. Static Resolution (Non-Penetration)
To prevent objects from merging, we calculate the overlap $L$:
$$ L = (R_1 + R_2) - dist $$

Each body is moved back along the normal proportional to the inverse of its mass (the lighter object moves more):
$$ \vec{P}_1 \leftarrow \vec{P}_1 - \vec{n} \cdot L \cdot \frac{m_2}{m_1 + m_2} $$
$$ \vec{P}_2 \leftarrow \vec{P}_2 + \vec{n} \cdot L \cdot \frac{m_1}{m_1 + m_2} $$

## 4. Dynamic Resolution (Impulse Response)
We use the **Impulse-Based** approach for elastic collisions.

### Relative Velocity
The relative velocity $\vec{v}_{rel}$ is:
$$ \vec{v}_{rel} = \vec{v}_1 - \vec{v}_2 $$

### Normal Velocity
The velocity component projected onto the collision normal:
$$ v_n = \vec{v}_{rel} \cdot \vec{n} = (v_{1x} - v_{2x})n_x + (v_{1y} - v_{2y})n_y $$

### Impulse Magnitude ($j$)
For a perfectly elastic collision (Coefficient of Restitution $e = 1$), the impulse $j$ required to satisfy both conservation of momentum and energy is:
$$ j = \frac{2 \cdot v_n}{\frac{1}{m_1} + \frac{1}{m_2}} = \frac{2 \cdot v_n \cdot m_1 \cdot m_2}{m_1 + m_2} $$

*Note: In the code, we apply $(j / m_1)$ to $v_1$, which simplifies to:*
$$ \Delta \vec{v}_1 = - \frac{j}{m_1} \vec{n} = - \frac{2 \cdot v_n \cdot m_2}{m_1 + m_2} \vec{n} $$

### Final State
$$ \vec{v}_1' = \vec{v}_1 - \vec{n} \cdot \frac{j}{m_1} $$
$$ \vec{v}_2' = \vec{v}_2 + \vec{n} \cdot \frac{j}{m_2} $$
