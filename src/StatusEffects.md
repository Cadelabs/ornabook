<div class="title">Status Effects</div>

Status effects are changes that affect your character during combat. They can increase or decrease your stats, inflict damage over time, cause death, ... or multiple of those at the same time. One can get a status effect through:

  - One of their own spells
  - A spell of an enemy
  - A spell of a follower
  - A spell of an ally, while playing in party mode
  - An equipped item

Some status effects are temporary, while some other are permanent. Some effects do have both a temporary and a permanent variation. Temporary status effects have a chance of fading each turn and are usually prefixed by `T.`.

# Stats altering status effects
## Generic status effects
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

## Gaits
Gaits are a family of buffs that can only be applied to oneself.
They are learnt through the following classes:

- T4 [Gait of Hildr](https://playorna.com/codex/spells/gait-of-hildr/) is learnt by [Wolf Tamer](https://playorna.com/codex/classes/98/) at level 90
- T6 [Gait of Thrud](https://playorna.com/codex/spells/gait-of-thrud/) is learnt by [Dragoon / Valkyrie](https://playorna.com/codex/classes/101/) at level 130
- T10 Gaits are learnt by [Beowulf / Bestla](https://playorna.com/codex/classes/57/) at levels 225 ([Kára](https://playorna.com/codex/spells/gait-of-kara/) and [Eir](https://playorna.com/codex/spells/gait-of-eir/)) and 230 ([Snotra](https://playorna.com/codex/spells/gait-of-snotra/) and [Gunnr](https://playorna.com/codex/spells/gait-of-gunnr/)).

While Gaits are permanent buffs, only one Gait can be active at a time.
Casting a Gait spell while another Gait is active will replace the active Gait, as opposed to granting a new status effect.
Gaits and [Stances](#stances) can both be applied at the same time (they do not override one another).

Gaits allow their caster to boost some stats at the expense of some others.
Below are the Gaits and their effects:

| Tier | Gait                     |         +++         |         +++         |         ---         |         ---         |
|:----:|:-------------------------|:-------------------:|:-------------------:|:-------------------:|:-------------------:|
|   4  | :statuses/hildr: Hildr   | +10% :statuses/def: | +10% :statuses/res: | -25% :statuses/atk: | -25% :statuses/mag: |
|   6  | :statuses/thrud: Thrud   | +10% :statuses/atk: | +10% :statuses/mag: | -25% :statuses/def: | -25% :statuses/res: |
|  10  | :statuses/eir: Eir       |                     | +50% :statuses/res: | -90% :statuses/atk: | -90% :statuses/def: |
|  10  | :statuses/gunnr: Gunnr   |                     | +50% :statuses/atk: | -90% :statuses/mag: | -90% :statuses/res: |
|  10  | :statuses/kara: Kára     |                     | +50% :statuses/def: | -90% :statuses/mag: | -90% :statuses/res: |
|  10  | :statuses/snotra: Snotra |                     | +50% :statuses/mag: | -90% :statuses/atk: | -90% :statuses/def: |


## Stances
Stances are a family of Giant buffs that can only be applied to oneself.
They are only obtainable through off-hand abilities (on off-hands, dual-wielding or two-handed weapons).
The weapons are obtained by defeating bosses and raids summoned by the [Celestial Stardrops](https://playorna.com/codex/items/celestial-stardrop/) and [Celestial Moondrops](https://playorna.com/codex/items/celestial-moondrop/) respectively (note that these can be used year-round, but only drop during the _Of Giants and Titans_ event).

Much like [Gaits](#gaits), they are permanent but only one can be active at a time.
One cannot equip two Stance spells at once in their loadout (since there is only one off-hand ability slot), but it is possible to override a stance in [battle dungeons](Dungeons.md#battle-dungeons) by leaving, changing equipment, then re-entering the dungeon (a useful strategy for endless dungeons).
[Gaits](#gaits) and Stances can both be applied at the same time (they do not override one another).

| Stance  | Effect |
|:-------:|:-------|
| Aegir   | +100% :statuses/def::statuses/res:, -30% :statuses/mag:, -90%:statuses/atk: |
| Asteria | +30% :statuses/mag:, -50% :statuses/def::statuses/res:, -5% max mana per turn |
| Atlas   | +20% Collateral Damage damage, +10% Collateral Damage chance, -20% max ward on Collateral Damage proc |
| Eistla  | +20% :statuses/atk::statuses/mag:, (makes follower and summons weaker?) |
| Gymir   | +10% damage, (DoT based on your damage per hit, 5%) |
| Ophion  | 25% chance of applying your next buff to a summon |

```admonish todo "TODO(24/08/2024, ethiraric)"
Check Eistla effect on follower and summons

Check Gymir DoT effect
```

## Miscellaneous effects
| Effect | Description |
|:-------|:------------|
| :statuses/berserk_su: Dmg ↑ | Self buff from Berserk. Increases damage output by 50% at the cost of 5% of max HP per turn. |
| :statuses/berserk_du: All ↑ | Self buff from Berserk II. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 25% each at the cost of 10% of max HP per turn. |
| :statuses/berserk_tu: All ↑↑ | Self buff from Berserk III. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 50% each at the cost of 25% of max HP per turn. |
| :statuses/break: T. All ↓ | Debuff inflicted by Break. (?) |
| :statuses/bloodshift: Bloodshift | Nullifies healing. +5% :statuses/crit:. +10% :statuses/all:. Temporary, has a 15% chance to fade each turn. |
| :statuses/windswept: Windswept | -50% :statuses/crit:. Temporary, has a 15% chance to fade each turn. |
| :statuses/target_du: Target ↑↑ | Temporary. Increases the chance an enemy targets you in party play. |
| :statuses/target_su: Target ↑  | Temporary. Increases the chance an enemy targets you in party play. |
| :statuses/target_sd: Target ↓  | Temporary. Decreases the chance an enemy targets you in party play. |
| :statuses/target_dd: Target ↓↓ | Temporary. Decreases the chance an enemy targets you in party play. |

```admonish todo "TODO(ethiraric)"
(31/07/2023): What does "Break" do exactly?

(06/11/2023): How do Target statuses work? What is their fade chance?
```

## Monster only status effects
The Cerus family of raid bosses have two spells named "Defend" that apply a self-buff. One of these applies a :statuses/def:Defending status buff, while the other applies a :statuses/res:Defending buff. These buffs increase Cerus' defense or resistance respectively and are permanent. However, much like Gaits, the two buffs cannot be active at the same time and applying one will remove the other.

The Yggdrasil family of raid bosses have two spells, Tree of Life and Tree of Demise which largely add to Yggdrasil's defenses.

```admonish todo "TODO(04/08/2023, ethiraric)"
What is Defend's stat increase?

What is Tree of Life's stat increase? Weakness? Fade chance?
```

# Status Afflictions
This section details other negative status effects that can be inflicted on an enemy.

## Disables and DoTs
### Mechanics
* **Disables** are status effects which have a chance to prevent the afflicted target from taking their turn. If one uses its turn to use consumable, then it cannot be disabled. All other actions may be disabled.

  When one is afflicted by multiple disables, each of them roll separately. This means that disable chances stack _multiplicatively_.
* **DoTs**, standing for _Damage over Time_ are status effects which deal damage every turn.

Status afflictions in this section are either Disables, DoTs, or both at the same time. Most are temporary and have a chance to fade each turn. However, once inflicted, they cannot fade on the next turn the target has. Let us consider the following non-party scenario:

> 1. Player used Stun Dart! Target is stunned.
> 2. Target couldn't attack.
> 3. Player attacked!
> 4. Target attacked! Target is no longer stunned.

Here, on `1.`, the player stunned the target. The next turn the target has is `2.`. On `2.`, it is not possible for the :statuses/stunned:Stunned status effect to fade. `4.` is the first turn where :statuses/stunned:Stunned may fade.

In the very specific case of the :statuses/asleep:Asleep disable and party play, the target may not be woken up even if damaged by entities after being afflicted but before having had their turn (that is, if an entity played in between `1.` and `2.` in the above scenario).

### DoT damage computation.
DoTs deal damage based on the afflicted target's max HP. DoTs are capped by default to 999HP _each_ per turn. This means that a player afflicted with 2 DoTs can take up to 1998 damage per turn.
Some player amities or gear (e.g.: Autumna) can alter that cap for either themselves or the entities on which they inflict DoTs.

Sometimes, the default cap is not 999:
  * Drakeblight has a cap of 500.
  * Starstruck has a cap of 9,999.
  * Doom has a cap of 99,999.
  * Titans have 25x the regular cap (i.e.: 24,975 as the default cap, 12,500 for Drakeblight, 249,975 for Starstruck and 2,499,975 for Doom).

The formula for HP lost per turn is `min(cap, max(1, floor(HPmax*(X/100)) ))`, with `X` the % of max health dealt as damage.

```admonish todo "TODO(08/11/2023, ethiraric)"
Check whether pet damage may proc the Autumna gear. Not just Drakeblight.

Check how Autumna gear affects the cap. Is the cap specific to the enemy? Or per status?
```

### List and effects
In the following table:
* _Fade_ refers to the chance that, at the end of the turn, the effect fades away
* _Dmg_ is the percentage of max HP the target loses per turn (within the cap)
* _Miss_ is the chance that the target misses its turn

| Effect                             | Fade | Dmg | Miss | Notes |
|:-----------------------------------|:----:|:---:|:----:|:------|
| :statuses/asleep: Asleep           |  25% |     | 100% | Target cannot evade an attack<br/>They wake up when they take damage from a player / monster / summon</br>They cannot be woken up this way until they have missed 1 turn at least |
| :statuses/bleeding: Bleeding       |  25% |  2% |      | Decreases :statuses/crit: Critical Chance by 5% |
| :statuses/blighted: Blight         |  25% |  2% |  25% | |
| :statuses/blinded: Blind           |  25% |     |  50% | 50% miss chance is for attacks or damaging skills/spells to hit only |
| :statuses/burning: Burning         |  10% |  5% |      | Decreases the damage from skills and spells by 20% (equivalent to :statuses/atk_sd_tmp:+:statuses/mag_sd_tmp:) |
| :statuses/confused: Confused       |  50% |     |      | Target may not perform the action they intended, even if they intended to use a consumable<br/>They may use a random spell from their loadout or Defend (?) |
| :statuses/cursed: Cursed           |      | 10% |      | |
| :statuses/doomed: Doom             |   3% |200%*|      | Has a 5 turns countdown, damage capped to 99,999 |
| :statuses/drenched: Drenched       |   5% |     |   5% | |
| :statuses/frozen: Frozen           |  50% |     |  40% | Target cannot evade an attack |
| :statuses/lulled: Lulled           |  10% |     |  20% | |
| :statuses/paralyzed: Paralyzed     |  25% |     |  25% | |
| :statuses/petrified: Petrified     |  20% |     | 100% | Target cannot evade an attack |
| :statuses/poisoned: Poisoned       |  10% |  2% |      | |
| :statuses/rotten: Rot              |  20% |  3% |      | Decreases :statuses/def::statuses/res: by 20% |
| :statuses/starstruck: Starstruck   |  50% | 10%*|      | Has a 3 turns countdown, damage capped to 9,999 |
| :statuses/stasis: Stasis           |  50% |     | 100% | Target cannot evade an attack |
| :statuses/stunned: Stunned         |  50% |     |  50% | Target cannot evade an attack |
| :statuses/toxic: Toxic             |      | 10% |  10% | |

*Damage is dealt each turn once the countdown reaches 0. For instance, a target has 3 turns once inflicted with :statuses/starstruck:Starstruck where they does not take damage. Starting on the 4th turn, they will take 10% damage until the effect fades. 

```admonish todo "TODO(04/08/2023, ethiraric)"
How often does a Confused target fail to perform their action? How is the new action selected?
```

## Sigils
```admonish todo
```

## Blights
```admonish todo "TODO(06/11/2023, ethiraric)"
Drakeblight is a 1% DoT with a 500 cap (12,500 against Titans). Sometimes causes one to be unable to attack. Seems uninfluenced by Autumna gear.
([OL convo](https://discord.com/channels/748188991852904621/811270018661613627/1171205996534841365))
```

## Miscellaneous effects
| Effect                             | Details |
|:-----------------------------------|:--------|
| :statuses/towering: Towering       | Only available through the Titan's [Build Tower](https://playorna.com/codex/spells/build-tower/) spell.<br/>Restores 2% of max HP per turn. Temporary.|
| :statuses/arcane: Arcane ↑↑        | Given only through [Aglovale](https://playorna.com/codex/items/aglovale/).<br/> Increases Arcane element damage by 50% and causes all Arcane damage to produce blue numbers as though enemies were weak to Arcane. Does not act as a weakness for Ultima or similar spells. Temporary. <br/> May have other unknown interactions.
| :statuses/dragon: Dragon ↑↑        | Given only through [Tethra](https://playorna.com/codex/items/tethra/).<br/> Increases Dragon element damage by 50% and causes all Dragon damage to produce blue numbers as though enemies were weak to Dragon. Does not act as a weakness for Ultima or similar spells. Temporary. <br/> May have other unknown interactions.

```admonish todo "TODO(08/01/2023, ethiraric)"
Arcane and Dragon ↑↑ need more testing
```

# Elemental effects
```admonish todo
```

# Calls
```admonish todo
```
