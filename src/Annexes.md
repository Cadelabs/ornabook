<div class="title">Annexes</div>

# Gear boni dual-wield plot
The code for the [plot for gear boni when dual-wielding](./Gear.md#dual-wielding) is the following:

```py
import matplotlib.pyplot as plt
import numpy as np

def f(x):
    return (1 + 0.65 * x)**2 / (1 + x)

plt.style.use({
    'figure.facecolor': '#0f1419',
    'text.color': '#ffffff',
    'axes.facecolor': '#0f1419',
    'axes.edgecolor': '#ffffff',
    'axes.labelcolor': '#ffffff',
    'xtick.color': '#ffffff',
    'xtick.labelcolor': '#ffffff',
    'ytick.color': '#ffffff',
    'ytick.labelcolor': '#ffffff',
})

x = np.linspace(0, 1, 10000)
y = f(x)

fig, ax = plt.subplots()
fig.set_size_inches(10, 5)
ax.plot(x, y)
ax.set_title('Ratio of boni when dual-wielding over single-wielding', size=14)
plt.savefig('boni_dual_wield_ratio_plot.svg', dpi=100)
```
