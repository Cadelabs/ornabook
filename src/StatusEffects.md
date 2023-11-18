# Status Effects
Status effects are changes that affect your character during combat. They can increase or decrease your stats, inflict damage over time, cause death, ... or multiple of those at the same time. One can get a status effect through:

  - One of their own spells
  - A spell of an enemy
  - A spell of a follower
  - A spell of an ally, while playing in party mode
  - An equipped item

Some status effects are temporary, while some other are permanent. Some effects do have both a temporary and a permanent variation. Temporary status effects have a chance of fading each turn and are usually prefixed by `T.`.

## Stats altering status effects
### Generic status effects
These status effects raise or lower a player's in-battle stats (attack, magic, defense, resistance, and/or dexterity):
    - :statuses/def: Defense: Change to the defense stat
    - :statuses/res: Resistance: Change to the resistance stat
    - :statuses/atk: Attack: Change to the damage a melee skill does (doesn't change the target's attack value)
    - :statuses/mag: Magic: Change to the damage a magical spell does (doesn't change the target's magic value)
    - :statuses/dex: Dexterity: Change to the dexterity stat
    - :statuses/crit: Critical Chance: Change to the chance of dealing a critical strike
    - :statuses/all: All: Change to :statuses/def::statuses/res::statuses/atk::statuses/mag::statuses/dex: all at once

There exists different variations of the above effects, often referred to as "up"/"down", "single"/"double"/"triple" and "temporary"/"permanent". Temporary effects (whether positive or negative) are denoted with purple arrows, permanent positive effects are denoted with green arrows and permanent negative effects are denoted with orange arrows.

The table below describes, for each buff, the alteration percentage for each combination of stat and arrows. Empty cells denote combinations that do not exist and triple question marks (`???`) combinations that do exist but whose alteration is not known or hasn't yet been contributed.

| Stat(s) altered | :statuses/td_tmp: | :statuses/dd_tmp: | :statuses/sd_tmp: | :statuses/su_tmp: | :statuses/du_tmp: | :statuses/tu_tmp: | :statuses/sd: | :statuses/dd: | :statuses/su: | :statuses/du: |
|:---------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-------------:|:-------------:|:-------------:|:-------------:|
| :statuses/def::statuses/res::statuses/atk::statuses/mag: | -100% | -80% | -20% | +25% | +50% | +100% | -20% | -80% | +25% | +50% |
| :statuses/dex: |     |     | ??? | ??? |     | ??? |     |     | ??? |     |
| :statuses/all: |     |     |     | +20% |     | +100% |     |     |     |     |
| :statuses/crit: |     |     |     | +20% | ??? | +80%* |     |     | +20% |     |

*:statuses/crit_tu_tmp: also grants +10%:statuses/atk: (no additional effect on :statuses/mag:)

```admonish note
Individual buff combinations are considered different buffs and stack *multiplicatively*. This means that :statuses/def_su:+:statuses/def_du: grants a `1.25*1.5=1.875` factor to defense (+87.5%). However, using different sources for the same buff does not apply the buff twice. For example, if :statuses/mag_su: is granted using either of Magic Boost or Wyvern Speed, casting the other spell will not grant another :statuses/mag_su:.
```

### Gaits
Gaits are a family of buffs that can only be applied to oneself. They are learnt by the Beowulf/Bestla at levels 225 and 230. While Gaits are permanent buffs, only one Gait can be active at a time. Casting a Gait spell while another Gait is active will replace the active Gait, as opposed to granting a new status effects.

Gaits allow their caster to boost one of their stats at the expense of 2 others. The boosted stat is increased by 50%, while the 2 sacrificed stats are decreased by 90% each.

Below are the 4 Gaits and their effects:

| Gait | +++ | --- | --- |
|:-----|:---:|:---:|:---:|
| :statuses/eir: Eir | +50% :statuses/res: | -90% :statuses/atk: | -90% :statuses/def: |
| :statuses/gunnr: Gunnr | +50% :statuses/atk: | -90% :statuses/mag: | -90% :statuses/res: |
| :statuses/kara: Kára | +50% :statuses/def: | -90% :statuses/mag: | -90% :statuses/res: |
| :statuses/snotra: Snotra | +50% :statuses/mag: | -90% :statuses/atk: | -90% :statuses/def: |

### Miscellaneous effects
| Effect | Description |
|:-------|:------------|
| :statuses/berserk_su: Dmg ↑ | Self buff from Berserk. Increases damage output by 50% at the cost of 5% of max HP per turn. |
| :statuses/berserk_du: All ↑ | Self buff from Berserk II. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 25% each at the cost of 10% of max HP per turn. |
| :statuses/berserk_tu: All ↑↑ | Self buff from Berserk III. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 50% each at the cost of 25% of max HP per turn. |
| :statuses/break: T. All ↓ | Debuff inflicted by Break. (?) |
| :statuses/bloodshift: Bloodshift | Nullifies healing. +5% :statuses/crit:. +10% :statuses/all:. Temporary, has a 15% chance to fade each turn. |
| :statuses/windswept: Windswept | Reduces :statuses/crit: (?). Temporary, has a 15% chance to fade each turn. |
| Target ↑↑ | |
| Target ↑  | |
| Target ↓  | |
| Target ↓↓ | |

```admonish todo "TODO(ethiraric)"
(31/07/2023): What does "Break" do exactly?

(04/08/2023): By how much does Windswept reduce crit chance?
```

### Monster only status effects
The Cerus family of raid bosses have two spells named "Defend" that apply a self-buff. One of these applies a :statuses/def:Defending status buff, while the other applies a :statuses/res:Defending buff. These buffs increase Cerus' defense or resistance respectively and are permanent. However, much like Gaits, the two buffs cannot be active at the same time and applying one will remove the other.

The Yggdrasil family of raid bosses have two spells, Tree of Life and Tree of Demise which largely add to Yggdrasil's defenses.

```admonish todo "TODO(04/08/2023, ethiraric)"
What is Defend's stat increase?

What is Tree of Life's stat increase? Weakness? Fade chance?
```

## Status Afflictions
This section details other negative status effects that can be inflicted on an enemy.

### Disables and DoTs
#### Mechanics
* **Disables** are status effects which have a chance to prevent the afflicted target from taking their turn. If one uses its turn to use consumable, then it cannot be disabled. All other actions may be disabled.

  When one is afflicted by multiple disables, each of them roll separately. This means that disable chances stack _multiplicatively_.
* **DoTs**, standing for _Damage over Time_ are status effects which deal damage every turn. DoTs are capped to 999HP _each_ per turn. A player having 2 DoTs can lose up to 1998HP each turn. Some amities may change this cap.

Status afflictions in this section are either Disables, DoTs, or both at the same time. Most are temporary and have a chance to fade each turn. However, once inflicted, they cannot fade on the next turn the target has. Let us consider the following non-party scenario:

> 1. Player used Stun Dart! Target is stunned.
> 2. Target couldn't attack.
> 3. Player attacked!
> 4. Target attacked! Target is no longer stunned.

Here, on `1.`, the player stunned the target. The next turn the target has is `2.`. On `2.`, it is not possible for the :statuses/stunned:Stunned status effect to fade. `4.` is the first turn where :statuses/stunned:Stunned may fade.

In the very specific case of the :statuses/asleep:Asleep disable and party play, the target may not be woken up even if damaged by entities after being afflicted but before having had their turn (that is, if an entity played in between `1.` and `2.` in the above scenario).

#### List and effects
In the following table:
* _Fade_ refers to the chance that, at the end of the turn, the effect fades away
* _Dmg_ is the percentage of max HP the target loses per turn (within the cap)
* _Miss_ is the chance that the target misses its turn

| Effect                             | Fade | Dmg | Miss | Note |
|:-----------------------------------|:----:|:---:|:----:|:-----|
| :statuses/asleep: Asleep           |  25% |     | 100% | Target cannot evade an attack<br/>They wake up when they take damage from a player / monster / summon</br>They cannot be woken up this way until they have missed 1 turn at least |
| :statuses/bleeding: Bleeding       |  25% |  2% |      | Decreases :statuses/crit: Critical Chance by 5% |
| :statuses/blighted: Blight         |  25% |  2% |  25% | |
| :statuses/blinded: Blind           |  25% |     |  50% | 50% miss chance is for attacks or damaging skills/spells to hit only |
| :statuses/burning: Burning         |  10% |  5% |      | Decreases the damage from skills and spells by 20% (equivalent to :statuses/atk_sd_tmp:+:statuses/mag_sd_tmp:) |
| :statuses/confused: Confused       |  50% |     |      | Target may not perform the action they intended, even if they intended to use a consumable<br/>They may use a random spell from their loadout or Defend (?) |
| :statuses/cursed: Cursed           |      | 10% |      | |
| :statuses/doomed: Doom             |   3% |200%*|      | Has a 5 turns countdown, damage cap unknown but extremely high |
| :statuses/drenched: Drenched       |   5% |     |   5% | |
| :statuses/frozen: Frozen           |  50% |     |  40% | Target cannot evade an attack |
| :statuses/lulled: Lulled           |  10% |     |  20% | |
| :statuses/paralyzed: Paralyzed     |  25% |     |  25% | |
| :statuses/petrified: Petrified     |  20% |     | 100% | Target cannot evade an attack |
| :statuses/poisoned: Poisoned       |  10% |  2% |      | |
| :statuses/rotten: Rot              |  20% |  3% |      | Decreases :statuses/def::statuses/res: by 20% |
| :statuses/starstruck: Starstruck   |  50% | 10%*|      | Has a 3 turns countdown, damage capped to 9999 |
| :statuses/stasis: Stasis           |  50% |     | 100% | Target cannot evade an attack |
| :statuses/stunned: Stunned         |  50% |     |  50% | Target cannot evade an attack |
| :statuses/toxic: Toxic             |      | 10% |  10% | |

*Damage is dealt each turn once the countdown reaches 0. For instance, a target has 3 turns once inflicted with :statuses/starstruck:Starstruck where they does not take damage. Starting on the 4th turn, they will take 10% damage until the effect fades. 

```admonish todo "TODO(04/08/2023, ethiraric)"
How often does a Confused target fail to perform their action? How is the new action selected?
```

### Sigils
```admonish todo
```

### Blights
```admonish todo
```

### Miscellaneous effects
```admonish todo
Towering
```

## Elemental resistances and alignments
```admonish todo
```

## Calls
```admonish todo
```

