<div class="title">Gear</div>

# Gear bonus
## Dual-wielding
Note: This section purposefully ignores "negative boni" (e.g.: XP on the Ring of Restraint, Broken equipment, ...).

### Example
When dual-wielding gear with boni, the dual-wielding penalty is applied to both weapons.
However, dual-wielding two weapons with the same boni always yields slightly more than wielding only one of the weapons.

Let `B` be the bonus given by the weapon.
We express that bonus not as 1.5x, but rather as 0.5x more.
For instance, a Godforged (GF) Questing Staff increases Orn gains by 57.5%, so we would have `B = 0.575`.
If we have a single GF Questing Staff equipped without dual-wielding another weapon, our total multiplier is:

\\[ \text{multiplier}_{\text{single-wield}}\ = (1 + B) = 1.575 \\]

If we dual-wield GF Questing Staves, we have a 35% penalty applied to each weapon.
The multiplier for a single GF Questing Staff when dual-wielding (e.g.: GF Questing Staff in the left hand and an Arisen Kaladanda in the right hand) is thus:

\\[ \text{multiplier}_{\text{questing-staff-dual-wield}}\ = (1 + 0.65 * B) = (1 + 0.65 * 0.575) = 1.37375 \\]

This is to be expected; the penalty of dual-wielding reduces our bonus.
If however we dual-wield 2 GF Questing Staves, our multiplier above is applied twice (once for each staff), leading to the following:

\\[ \text{multiplier}_{\text{2-questing-staves}} \\
    =\ \text{multiplier} _{\text{questing-staff-dual-wield}}\ ^{2}  \\
    =\ 1.37375^{2} \\
    = 1.8871890625 \\]

A single GF Questing Staff increases our Orn gains by 57.5%, while dual-wielding 2 increases our Orn gains by ~88.72%.

### Generic case
Let us consider `B` the bonus as described in the above section.

With 1 weapon, we have the following multiplier:

\\[ \text{multiplier}_{\text{single-wield}}\ = (1 + B) \\]

When dual-wielding 2 weapons with the same bonus, we have the following multiplier:

\\[ \text{multiplier}_{\text{dual-wield}}\ = (1 + 0.65 * B)^{2} \\]

The ratio between single-wielding and dual-wielding can be expressed as:

\\[ \text{dual-wield-ratio}\ 
    = \frac{\text{multiplier} _{\text{dual-wield}}}{\text{multiplier} _{\text{single-wield}}}
    = \frac{(1 + 0.65 * B)^{2}}{1 + B} \\]

We can ask [Wolframalpha](https://www.wolframalpha.com/input?i=%281%2B%280.65x%29%29*%281%2B%280.65x%29%29%2F%281%2Bx%29%2C+x+in+%5B0%2C+1%5D) for an interactive plot of this function over `B`.
A static plot of it (with [matplotlib](./Annexes.md#gear-boni-dual-wield-plot)) is available below.

<center>
<img alt="Plot of the aforementioned function. It ranges from 1 at B=0 to around 1.35 at B=1. The plot is almost a straight line. " src="/img/boni_dual_wield_ratio_plot.svg" />
</center>

This plot shows the ratio can be read as:

> If I dual-wield 2 copies of my weapon, how much more would I gain compared to only using one of the weapon?

Note that it does NOT compare dual-wielding with not having a bonus weapon at all.

To use the chart, first check the bonus on your weapon.
Let's reuse the GF Questing Staff from the previous section.
Its bonus is 57.5%, which is 0.575.
If we try to read the chart, we can see the `y` value at `x=0.575` is close to 1.20.
We can check this by computing the exact ratio, which would be 1.8871890625/1.575 which is ~1.198.

We can see that the ratio is always superior to 1 (except on 0, but weapons without any bonus are not discussed here), meaning that dual-wielding weapons with boni always yields more rewards than wielding only one.

# Hybrid
Orna has four different "kinds" of Hybrid:
- All skills and spells cast will use both your Attack and Magic stats
- Pure Hybrid
- Dynamic Hybrid
- Hybrid Monster (e.g.: [Beowulf](https://playorna.com/codex/classes/57/)) / Hybrid Damage (e.g.: [Arms](https://playorna.com/codex/items/arms-of-selene/) / [Hands](https://playorna.com/codex/items/steady-hands-of-selene/) of Selene)

## All skills and spells cast will use both your Attack and Magic stats
Instead of just using your Attack or Magic stat, all your skills and spells will use a combination of both of them.
Additionally, they will use the average of the Defence and the Resistance of your target.
This is not normally relevant, as monsters have the same Defence and Resistance values, but can be important if an enemy has certain buffs (Cerus' Defends) or during a Fomorian Houses event.
The following formula replaces the attack or magic stat in the usual damage formula: \\( (\text{Attack} + \text{Magic}) * \frac{3}{5} \\) (or \\( (\text{Attack} + \text{Magic}) / 1.66 \\)).
If your attack and magic stats are equal, this is roughly a 20% damage increase.

This has no effect at all on skills that are already hybrid, like Beaststrike or Verse.
The effects of this are applied directly to the damage formula and cannot be seen in the Status menu.

This changes the damage formula to (note the 4 instead of 2 below `def + res` to account for the averaging):

\\[
    \text{damage} = \lfloor ((\text{atk} + \text{mag}) * \frac{3}{5} * \text{stat-multiplier} - \frac{\text{def} + \text{res}}{4}) * \text{damage-multiplier} \rfloor
\\]

## Pure Hybrid
Pure hybrid skills use 2/3 of your attack and 2/3 of your magic stat.
The following formula replaces the attack or magic stat in the usual damage formula: \\( (\text{atk} + \text{mag}) * \frac{2}{3} \\) or \\( (\text{atk} + \text{mag}) * 0.6667 \\).

The buffs used in the formula depend on whether a skill or a spell is cast.
If a :skill_sword:skill is cast, the game will use the caster's :statuses/atk:Attack buffs and the target's :statuses/def:Defence buffs.
If a :spell_staff:spell is cast, the game will use the caster's :statuses/mag:Magic buffs and the target's :statuses/res:Resistance buffs.

This changes the damage formula to the following:

\\[
    \text{damage} = \lfloor ((\text{atk} + \text{mag}) * \frac{2}{3} * \text{stat-multiplier} - \frac{\text{def} + \text{res}}{4}) * \text{damage-multiplier} \rfloor
\\]

## Dynamic Hybrid
We call Dynamic Hybrid all skills or spells that "*use either attack or magic, whichever is higher*".
[Sands of Aaru](https://playorna.com/codex/spells/sands-of-aaru/) or God classes' [Eventualus Apex skills](https://playorna.com/codex/spells/?q=eventualus) are examples of this.

As the description mentions, these spells replace the Attack or Magic value from the damage formula with whichever stat is the highest.
The defensive stat is replaced by the average of the target's Defence and Resistance.

The offensive buffs that are applied depend on whether the :statuses/atk:Attack stat or the :statuses/mag:Magic stat of the caster is the highest.
If the Attack stat is the highest, Attack buffs are used.
Otherwise, Magic buffs are used.
The game does not base this decision on the potential damage output.
If one has :statuses/atk:1000 and :statuses/mag:1001, the Magic buffs are chosen, even if the caster has the following buffs: :statuses/atk_tu_tmp::statuses/atk_du_tmp::statuses/atk_du: :statuses/mag_dd_tmp::statuses/mag_sd_tmp::statuses/mag_sd:.

The defensive buffs used in the formula depend on whether a skill or a spell is cast.
If a :skill_sword:skill is cast, the game will use the target's :statuses/def:Defence buffs.
If a :spell_staff:spell is cast, the game will use the target's :statuses/res:Resistance buffs.

This changes the damage formula to the following:

\\[
    \text{damage} = \lfloor (\text{max} (\text{atk} , \text{mag})  * \text{stat-multiplier} - \frac{\text{def}+\text{res}}{4}) * \text{damage-multiplier} \rfloor
\\]

## Hybrid Monster (e.g.: [Beowulf](https://playorna.com/codex/classes/57/)) / Hybrid Damage (e.g.: [Arms](https://playorna.com/codex/items/arms-of-selene/) / [Hands](https://playorna.com/codex/items/steady-hands-of-selene/) of Selene)
This refers to the effects described as "Hybrid damage will be increased by X%".

Contrary to the description, the implementation of this is not a multiplier to the damage of Hybrid skills and spells.
It increases your attack stat by X% of your magic stat and your magic stat by X% of your attack stat.
Since it changes your raw stats, its effects are visible in the Status menu.
This effect stacks additively.

This effect works with other kinds of hybrids and does not change the damage formula for regular skills and spells.

## Summary
Let us consider the following genric damage formula:

\\[
    \text{damage} = \lfloor ({\color{red} \text{O}}  * \text{stat-multiplier} - \frac{\text{def} + \text{res}}{4} * {\color{yellow} \text{DB}}) * \text{damage-multiplier} * {\color{green} \text{OB}} \rfloor
\\]

Where `O` is the offensive stat and `OB` / `DB` the offensive and defensive buffs respectively.
For all hybrid kinds (except Hybrid Monster / Hybrid Damage), we have:

\\[
    {\color{yellow} \text{defensive-buffs}} = \text{if using }
    \begin{cases} \text{a skill} \Rightarrow\text{def buffs} \\\\
                  \text{a spell} \Rightarrow\text{res buffs}
    \end{cases}
\\]

##### All skills and spells cast will use both your Attack and Magic stats
\\[
    {\color{red} \text{offensive-stat}} = (\text{atk} + \text{mag}) * \frac{3}{5}
\\]
\\[
    {\color{green} \text{offensive-buffs}} = \text{if using }
    \begin{cases} \text{a skill} \Rightarrow\text{atk buffs} \\\\
                  \text{a spell} \Rightarrow\text{mag buffs}
    \end{cases}
\\]

##### Pure Hybrid
\\[
    {\color{red} \text{offensive-stat}} = (\text{atk} + \text{mag}) * \frac{2}{3}
\\]
\\[
    {\color{green} \text{offensive-buffs}} = \text{if using }
    \begin{cases} \text{a skill} \Rightarrow\text{atk buffs} \\\\
                  \text{a spell} \Rightarrow\text{mag buffs}
    \end{cases}
\\]

##### Dynamic Hybrid Skills
\\[
    {\color{red} \text{offensive-stat}} = \text{max}(\text{atk}, \text{mag})
\\]
\\[
    {\color{green} \text{offensive-buffs}} = \text{if }
    \begin{cases} \text{atk} > \text{mag} \Rightarrow \text{atk buffs} \\\\
                  \text{mag} >= \text{atk} \Rightarrow \text{mag buffs}
    \end{cases}
\\]
