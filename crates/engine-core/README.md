# engine-core

`engine-core` is the foundational library for `science-engines`.

Current focus:

- ODE (ordinary differential equation) interface
- RK4 integrator
- Harmonic oscillator + logistic growth example models
- Validation via analytic solutions and invariants (energy)

---

## 1) ODE Formulation

We represent a dynamical system as:

- **State vector**:
  $$
  x(t) \in \mathbb{R}^n
  $$
- **ODE**:  
  $$
  \frac{dx}{dt} = f(t, x)
  $$

In code, this corresponds to the `OdeModel` trait:

- `dim()` returns $n$
- `deriv(t, x, out)` writes $f(t, x)$ into `out` (to minimize allocations)

---

## 2) Integrator Interface

An integrator advances one step:

$$
(t, x) \rightarrow (t + dt, x_{\text{new}})
$$

In code, this is the `Integrator` trait:

- `step(model, t, x, dt, scratch)`
- `x` is updated **in-place**
- `Scratch` provides reusable buffers to avoid allocating new `Vec` each step

---

## 3) RK4 (Runge–Kutta 4th Order)

RK4 advances $x$ using four slope evaluations:

$$
k_1 = f(t, x)
$$
$$
k_2 = f\left(t + \frac{dt}{2}, x + \frac{dt}{2}k_1\right)
$$
$$
k_3 = f\left(t + \frac{dt}{2}, x + \frac{dt}{2}k_2\right)
$$
$$
k_4 = f\left(t + dt, x + dtk_3\right)
$$

Then update:

$$
x_{\text{new}} = x + \frac{dt}{6}(k_1 + 2k_2 + 2k_3 + k_4)
$$

Notes:

- RK4 has good accuracy and stability for many problems.
- RK4 is **not symplectic**, so energy may drift over long simulations for Hamiltonian systems.

---

## 3.1) Symplectic Integrators (for oscillator-style systems)

For many physics systems (e.g., spring/oscillator), we often use a state
$x=[x_0, x_1]$ where $x_0$ is position and $x_1$ is velocity.

### Symplectic (Semi-Implicit) Euler

Update rule (velocity first, then position):

$$
\begin{aligned}
v_{n+1} &= v_n + a(x_n)\,dt \\
x_{n+1} &= x_n + v_{n+1}\,dt
\end{aligned}
$$

This method is **symplectic** for position/velocity systems and often keeps
energy behavior more stable than standard Euler.

### Velocity Verlet

Update rule:

$$
\begin{aligned}
x_{n+1} &= x_n + v_n\,dt + \tfrac{1}{2}a_n\,dt^2 \\
v_{n+1} &= v_n + \tfrac{1}{2}(a_n + a_{n+1})\,dt
\end{aligned}
$$

where $a_n = a(x_n)$ and $a_{n+1} = a(x_{n+1})$.

Notes:
- Velocity Verlet is symplectic and typically provides good long-term stability for energy.
- For very small $dt$, RK4 can still have very low absolute error due to its 4th-order accuracy.

---

## 4) Example Models

### 4.1 Logistic Growth

ODE:

$$
\frac{dy}{dt} = r y \left(1 - \frac{y}{K}\right)
$$

Analytic solution (used for test validation):

$$
y(t) = \frac{K}{1 + \left(\frac{K - y_0}{y_0}\right)e^{-rt}}
$$

State:

- $x=[y]$

---

### 4.2 Harmonic Oscillator (Spring)

Second-order equation:

$$
x'' + \omega^2 x = 0
$$

Converted to first-order system with state:

- $x_0=\text{position}$
- $x_1=\text{velocity}$

$$
\begin{aligned}
\dot{x}_0 &= x_1 \\
\dot{x}_1 &= -\omega^2 x_0
\end{aligned}
$$

---

## 5) Energy Invariant (Oscillator)

For the harmonic oscillator, total energy is:

$$
E = \frac{1}{2}v^2 + \frac{1}{2}\omega^2 x^2
$$

In an ideal system, $E$ should remain constant.

We use an energy drift test as an invariant-based validation:

- RK4 may drift over long horizons
- Symplectic integrators (e.g., Velocity Verlet) are recommended for better long-term energy behavior

---

## 6) Current Tests

- **Smoke tests** for `Engine` skeleton (`tests/engine_smoke.rs`)
- **Logistic**: RK4 vs analytic solution (`tests/ode_logistic.rs`)
- **Oscillator**: RK4 vs analytic solution (`tests/ode_oscillator.rs`)
- **Energy**: invariant drift check (`tests/ode_energy.rs`)
- **Energy (compare)**: Verlet vs RK4 energy behavior (`tests/ode_energy_compare.rs`)
- **Energy (invariant)**: bounded energy range checks (`tests/ode_energy.rs`)
